//! Throwaway renderer for the Vivante 2D core (GC320/GC520) through etnaviv.
//!
//! A port of the proof-of-concept Weston renderer in Weston MR !1622 to
//! smithay's renderer traits. Every draw is a 2D stretch blit or clear:
//! linear buffers only, nearest-neighbour scaling, no buffer or output
//! transforms, SHM imported through a CPU copy.

use std::fmt;
use std::marker::PhantomData;
use std::os::fd::{AsRawFd, OwnedFd};
use std::sync::{Arc, Mutex, Once};

use drm_fourcc::{DrmFormat, DrmFourcc, DrmModifier};
use tracing::{info, warn};

use crate::backend::{
    SwapBuffersError,
    allocator::{
        Buffer,
        dmabuf::{Dmabuf, WeakDmabuf},
        format::FormatSet,
    },
};
use crate::utils::{Buffer as BufferCoords, Physical, Rectangle, Size, Transform};

#[cfg(feature = "wayland_frontend")]
use crate::{
    backend::renderer::{ImportDmaWl, ImportMemWl},
    wayland::{compositor::SurfaceData, shm},
};
#[cfg(feature = "wayland_frontend")]
use wayland_server::protocol::wl_buffer;

use super::{
    Bind, Color32F, ContextId, DebugFlags, Frame, FrameContext, ImportDma, ImportMem, Renderer,
    RendererSuper, Texture, TextureFilter, sync::SyncPoint,
};

mod ffi;
mod regs;

use regs::*;

/// Command stream size in 32-bit words.
const STREAM_WORDS: u32 = 0x4000;
/// Damage rectangles emitted per batch of state; bounds the words one batch needs.
const RECTS_PER_BATCH: usize = 64;

const MEM_FORMATS: &[DrmFourcc] = &[
    DrmFourcc::Argb8888,
    DrmFourcc::Xrgb8888,
    DrmFourcc::Abgr8888,
    DrmFourcc::Xbgr8888,
    DrmFourcc::Rgb565,
];
const TARGET_FORMATS: &[DrmFourcc] = &[DrmFourcc::Argb8888, DrmFourcc::Xrgb8888];

/// Error returned by the etnaviv 2D renderer
#[derive(Debug, thiserror::Error)]
pub enum EtnaError {
    /// Device or core setup failed
    #[error("etnaviv setup failed: {0}")]
    Setup(&'static str),
    /// Unsupported pixel format
    #[error("unsupported pixel format: {0:?}")]
    UnsupportedFormat(DrmFourcc),
    /// Unsupported modifier
    #[error("unsupported modifier: {0:?}")]
    UnsupportedModifier(DrmModifier),
    /// Unsupported buffer layout
    #[error("unsupported buffer: {0}")]
    Unsupported(&'static str),
    /// Importing a buffer failed
    #[error("import failed")]
    ImportFailed,
    /// Accessing a shm buffer failed
    #[cfg(feature = "wayland_frontend")]
    #[error("buffer access failed: {0}")]
    BufferAccess(#[from] crate::wayland::shm::BufferAccessError),
    /// Waiting for a sync point was interrupted
    #[error("blocking for a synchronization primitive got interrupted")]
    SyncInterrupted,
}

impl From<EtnaError> for SwapBuffersError {
    fn from(value: EtnaError) -> Self {
        match value {
            x @ EtnaError::SyncInterrupted => SwapBuffersError::TemporaryFailure(Box::new(x)),
            x => SwapBuffersError::ContextLost(Box::new(x)),
        }
    }
}

struct Bo(*mut ffi::etna_bo);

// SAFETY: libdrm_etnaviv locks its bo table; a bo is only touched from the
// compositor thread in practice.
unsafe impl Send for Bo {}
unsafe impl Sync for Bo {}

impl Drop for Bo {
    fn drop(&mut self) {
        unsafe { ffi::etna_bo_del(self.0) }
    }
}

impl Bo {
    /// Run `f` on the CPU mapping, after the GPU is done with the bo.
    fn with_map<T>(&self, op: u32, f: impl FnOnce(*mut u8) -> T) -> Result<T, EtnaError> {
        let map = unsafe { ffi::etna_bo_map(self.0) } as *mut u8;
        if map.is_null() {
            return Err(EtnaError::Unsupported("bo map failed"));
        }
        unsafe { ffi::etna_bo_cpu_prep(self.0, op) };
        let ret = f(map);
        unsafe { ffi::etna_bo_cpu_fini(self.0) };
        Ok(ret)
    }
}

#[derive(Clone, Copy, Debug)]
struct PixFormat {
    fourcc: DrmFourcc,
    de_format: u32,
    swizzle: u32,
    alpha: bool,
    bpp: u32,
}

fn pix_format(fourcc: DrmFourcc) -> Option<PixFormat> {
    let (de_format, swizzle, alpha, bpp) = match fourcc {
        DrmFourcc::Argb8888 => (DE_FORMAT_A8R8G8B8, DE_SWIZZLE_ARGB, true, 4),
        DrmFourcc::Xrgb8888 => (DE_FORMAT_X8R8G8B8, DE_SWIZZLE_ARGB, false, 4),
        DrmFourcc::Abgr8888 => (DE_FORMAT_A8R8G8B8, DE_SWIZZLE_ABGR, true, 4),
        DrmFourcc::Xbgr8888 => (DE_FORMAT_X8R8G8B8, DE_SWIZZLE_ABGR, false, 4),
        DrmFourcc::Rgb565 => (DE_FORMAT_R5G6B5, DE_SWIZZLE_ARGB, false, 2),
        _ => return None,
    };
    Some(PixFormat {
        fourcc,
        de_format,
        swizzle,
        alpha,
        bpp,
    })
}

struct Surface {
    bo: Bo,
    offset: u32,
    width: i32,
    height: i32,
    stride: u32,
    format: PixFormat,
}

impl fmt::Debug for Surface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Surface")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("stride", &self.stride)
            .field("format", &self.format.fourcc)
            .finish()
    }
}

/// A texture of the [`EtnaRenderer`]
#[derive(Debug, Clone)]
pub struct EtnaTexture(Arc<Surface>);

impl Texture for EtnaTexture {
    fn width(&self) -> u32 {
        self.0.width as u32
    }
    fn height(&self) -> u32 {
        self.0.height as u32
    }
    fn format(&self) -> Option<DrmFourcc> {
        Some(self.0.format.fourcc)
    }
}

/// A framebuffer of the [`EtnaRenderer`]
#[derive(Debug)]
pub struct EtnaTarget<'a> {
    surface: Arc<Surface>,
    _buffer: PhantomData<&'a mut Dmabuf>,
}

impl Texture for EtnaTarget<'_> {
    fn width(&self) -> u32 {
        self.surface.width as u32
    }
    fn height(&self) -> u32 {
        self.surface.height as u32
    }
    fn format(&self) -> Option<DrmFourcc> {
        Some(self.surface.format.fourcc)
    }
}

/// Renderer on a Vivante 2D core through libdrm_etnaviv
pub struct EtnaRenderer {
    _fd: OwnedFd,
    dev: *mut ffi::etna_device,
    gpu: *mut ffi::etna_gpu,
    pipe: *mut ffi::etna_pipe,
    stream: *mut ffi::etna_cmd_stream,
    supertile_version: u32,
    context_id: ContextId<EtnaTexture>,
    debug_flags: DebugFlags,
    dmabufs: Vec<(WeakDmabuf, Arc<Surface>)>,
    /// One small bo per translucent solid colour, filled once.
    solids: Vec<(u32, Arc<Surface>)>,
}

impl fmt::Debug for EtnaRenderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EtnaRenderer").finish_non_exhaustive()
    }
}

impl Drop for EtnaRenderer {
    fn drop(&mut self) {
        self.dmabufs.clear();
        self.solids.clear();
        unsafe {
            ffi::etna_cmd_stream_del(self.stream);
            ffi::etna_pipe_del(self.pipe);
            ffi::etna_gpu_del(self.gpu);
            ffi::etna_device_del(self.dev);
        }
    }
}

const CHIP_FEATURES_PIPE_3D: u64 = 0x0000_0004;
const CHIP_FEATURES_PIPE_2D: u64 = 0x0000_0200;
const CHIP_MINOR_FEATURES5_HALTI5: u64 = 0x2000_0000;

impl EtnaRenderer {
    /// Open the first 2D-capable core behind an etnaviv render node.
    pub fn new(fd: OwnedFd) -> Result<Self, EtnaError> {
        let dev = unsafe { ffi::etna_device_new(fd.as_raw_fd()) };
        if dev.is_null() {
            return Err(EtnaError::Setup("etna_device_new"));
        }
        let mut supertile_version = 1;
        let mut gpu: *mut ffi::etna_gpu = std::ptr::null_mut();
        let mut pipe: *mut ffi::etna_pipe = std::ptr::null_mut();
        for core in 0.. {
            if !gpu.is_null() {
                unsafe { ffi::etna_gpu_del(gpu) };
            }
            gpu = unsafe { ffi::etna_gpu_new(dev, core) };
            if gpu.is_null() {
                break;
            }
            let mut feat = 0u64;
            if unsafe { ffi::etna_gpu_get_param(gpu, ffi::ETNA_GPU_FEATURES_0, &mut feat) } != 0 {
                continue;
            }
            if feat & CHIP_FEATURES_PIPE_3D != 0 {
                let mut feat6 = 0u64;
                if unsafe { ffi::etna_gpu_get_param(gpu, ffi::ETNA_GPU_FEATURES_6, &mut feat6) } == 0
                    && feat6 & CHIP_MINOR_FEATURES5_HALTI5 != 0
                {
                    supertile_version = 2;
                }
            }
            if feat & CHIP_FEATURES_PIPE_2D == 0 {
                continue;
            }
            pipe = unsafe { ffi::etna_pipe_new(gpu, ffi::ETNA_PIPE_2D) };
            if !pipe.is_null() {
                info!(core, "etnaviv: using 2D core");
                break;
            }
        }
        if pipe.is_null() {
            unsafe {
                if !gpu.is_null() {
                    ffi::etna_gpu_del(gpu);
                }
                ffi::etna_device_del(dev);
            }
            return Err(EtnaError::Setup("no 2D core"));
        }
        let stream = unsafe { ffi::etna_cmd_stream_new(pipe, STREAM_WORDS, None, std::ptr::null_mut()) };
        if stream.is_null() {
            unsafe {
                ffi::etna_pipe_del(pipe);
                ffi::etna_gpu_del(gpu);
                ffi::etna_device_del(dev);
            }
            return Err(EtnaError::Setup("etna_cmd_stream_new"));
        }
        Ok(Self {
            _fd: fd,
            dev,
            gpu,
            pipe,
            stream,
            supertile_version,
            context_id: ContextId::new(),
            debug_flags: DebugFlags::empty(),
            dmabufs: Vec::new(),
            solids: Vec::new(),
        })
    }

    /// Formats the renderer can draw into (linear only).
    pub fn render_formats(&self) -> FormatSet {
        linear(TARGET_FORMATS)
    }

    fn new_surface(&mut self, format: PixFormat, width: i32, height: i32) -> Result<Surface, EtnaError> {
        // FIXME: device alignment rules; 16 px / 8 rows is what Weston gets away with.
        let stride = (((width as u32) + 15) & !15) * format.bpp;
        let rows = ((height as u32) + 7) & !7;
        let bo = unsafe { ffi::etna_bo_new(self.dev, stride * rows, ffi::DRM_ETNA_GEM_CACHE_WC) };
        if bo.is_null() {
            return Err(EtnaError::ImportFailed);
        }
        Ok(Surface {
            bo: Bo(bo),
            offset: 0,
            width,
            height,
            stride,
            format,
        })
    }

    fn import_dmabuf_surface(&mut self, dmabuf: &Dmabuf) -> Result<Arc<Surface>, EtnaError> {
        self.dmabufs.retain(|(weak, _)| !weak.is_gone());
        if let Some((_, surface)) = self
            .dmabufs
            .iter()
            .find(|(weak, _)| weak.upgrade().is_some_and(|buf| &buf == dmabuf))
        {
            return Ok(surface.clone());
        }
        if dmabuf.num_planes() != 1 {
            return Err(EtnaError::Unsupported("multi-planar"));
        }
        let format = dmabuf.format();
        if !matches!(format.modifier, DrmModifier::Linear | DrmModifier::Invalid) {
            return Err(EtnaError::UnsupportedModifier(format.modifier));
        }
        let pix = pix_format(format.code).ok_or(EtnaError::UnsupportedFormat(format.code))?;
        let fd = dmabuf.handles().next().ok_or(EtnaError::ImportFailed)?;
        let bo = unsafe { ffi::etna_bo_from_dmabuf(self.dev, fd.as_raw_fd()) };
        if bo.is_null() {
            return Err(EtnaError::ImportFailed);
        }
        let size = dmabuf.size();
        let surface = Arc::new(Surface {
            bo: Bo(bo),
            offset: dmabuf.offsets().next().unwrap_or(0),
            width: size.w,
            height: size.h,
            stride: dmabuf.strides().next().unwrap_or(0),
            format: pix,
        });
        self.dmabufs.push((dmabuf.weak(), surface.clone()));
        Ok(surface)
    }

    fn solid_surface(&mut self, argb: u32) -> Result<Arc<Surface>, EtnaError> {
        if let Some((_, surface)) = self.solids.iter().find(|(c, _)| *c == argb) {
            return Ok(surface.clone());
        }
        if self.solids.len() >= 64 {
            self.solids.remove(0);
        }
        let surface = self.new_surface(pix_format(DrmFourcc::Argb8888).unwrap(), 1, 1)?;
        surface.bo.with_map(ffi::DRM_ETNA_PREP_WRITE, |map| unsafe {
            *(map as *mut u32) = argb;
        })?;
        let surface = Arc::new(surface);
        self.solids.push((argb, surface.clone()));
        Ok(surface)
    }

    // Command stream

    fn avail(&self) -> u32 {
        let s = unsafe { &*self.stream };
        s.size - s.offset - 2
    }

    fn reserve(&mut self, words: u32) {
        if self.avail() < words {
            unsafe { ffi::etna_cmd_stream_flush(self.stream) };
        }
    }

    fn emit(&mut self, v: u32) {
        unsafe {
            let s = &mut *self.stream;
            *s.buffer.add(s.offset as usize) = v;
            s.offset += 1;
        }
    }

    fn load_state(&mut self, address: u32) {
        self.emit(
            VIV_FE_LOAD_STATE_HEADER_OP_LOAD_STATE
                | VIV_FE_LOAD_STATE_HEADER_OFFSET(address >> 2)
                | VIV_FE_LOAD_STATE_HEADER_COUNT(1),
        );
    }

    fn set_state(&mut self, address: u32, value: u32) {
        self.load_state(address);
        self.emit(value);
    }

    fn set_state_bo(&mut self, address: u32, surface: &Surface, flags: u32) {
        self.load_state(address);
        let reloc = ffi::etna_reloc {
            bo: surface.bo.0,
            flags,
            offset: surface.offset,
        };
        unsafe { ffi::etna_cmd_stream_reloc(self.stream, &reloc) };
    }

    fn flush_stall(&mut self) {
        self.set_state(1 << 2, 0);
        self.set_state(1 << 2, 0);
        self.set_state(1 << 2, 0);
        self.set_state(VIVS_GL_FLUSH_CACHE, VIVS_GL_FLUSH_CACHE_PE2D);
        self.set_state(
            VIVS_GL_SEMAPHORE_TOKEN,
            VIVS_GL_SEMAPHORE_TOKEN_FROM(SYNC_RECIPIENT_FE) | VIVS_GL_SEMAPHORE_TOKEN_TO(SYNC_RECIPIENT_PE),
        );
        self.emit(VIV_FE_STALL_HEADER_OP_STALL);
        self.emit(VIV_FE_STALL_TOKEN_FROM(SYNC_RECIPIENT_FE) | VIV_FE_STALL_TOKEN_TO(SYNC_RECIPIENT_PE));
    }

    fn emit_target(&mut self, target: &Surface) {
        self.set_state(
            VIVS_DE_ROP,
            VIVS_DE_ROP_ROP_FG(0xcc) | VIVS_DE_ROP_ROP_BG(0xcc) | VIVS_DE_ROP_TYPE_ROP4,
        );
        self.set_state(VIVS_DE_PE_CONTROL, 0);
        self.set_state(VIVS_DE_PE_DITHER_LOW, 0xffff_ffff);
        self.set_state(VIVS_DE_PE_DITHER_HIGH, 0xffff_ffff);
        self.set_state(VIVS_DE_CONFIG, 0);
        self.set_state(VIVS_DE_SRC_ORIGIN_FRACTION, 0);
        self.set_state(
            VIVS_DE_TILE_CONFIG,
            VIVS_DE_TILE_CONFIG_SUPERTILE_VERSION(self.supertile_version),
        );
        self.set_state_bo(
            VIVS_DE_DEST_ADDRESS,
            target,
            ffi::ETNA_RELOC_READ | ffi::ETNA_RELOC_WRITE,
        );
        self.set_state(VIVS_DE_DEST_STRIDE, target.stride);
        self.set_state(VIVS_DE_DEST_ROTATION_HEIGHT, target.height as u32);
        self.set_state(
            VIVS_DE_DEST_ROTATION_CONFIG,
            VIVS_DE_DEST_ROTATION_CONFIG_WIDTH(target.width as u32),
        );
        self.set_state(VIVS_DE_CLIP_TOP_LEFT, VIVS_DE_CLIP_TOP_LEFT_X(0) | VIVS_DE_CLIP_TOP_LEFT_Y(0));
        self.set_state(
            VIVS_DE_CLIP_BOTTOM_RIGHT,
            VIVS_DE_CLIP_BOTTOM_RIGHT_X(target.width as u32)
                | VIVS_DE_CLIP_BOTTOM_RIGHT_Y(target.height as u32),
        );
    }

    fn dest_config(&mut self, target: &Surface, command: u32) {
        self.set_state(
            VIVS_DE_DEST_CONFIG,
            command
                | VIVS_DE_DEST_CONFIG_FORMAT(target.format.de_format)
                | VIVS_DE_DEST_CONFIG_SWIZZLE(target.format.swizzle)
                | VIVS_DE_DEST_CONFIG_TILED_DISABLE
                | VIVS_DE_DEST_CONFIG_MINOR_TILED_DISABLE,
        );
        self.set_state(VIVS_DE_ROT_ANGLE, VIVS_DE_ROT_ANGLE_SRC(DE_ROT_MODE_ROT0));
    }

    fn draw_rect(&mut self, r: Rectangle<i32, Physical>) {
        self.emit(VIV_FE_DRAW_2D_HEADER_OP_DRAW_2D | VIV_FE_DRAW_2D_HEADER_COUNT(1));
        self.emit(0xdead_beef);
        self.emit(VIV_FE_DRAW_2D_TOP_LEFT_X(r.loc.x as u32) | VIV_FE_DRAW_2D_TOP_LEFT_Y(r.loc.y as u32));
        self.emit(
            VIV_FE_DRAW_2D_BOTTOM_RIGHT_X((r.loc.x + r.size.w) as u32)
                | VIV_FE_DRAW_2D_BOTTOM_RIGHT_Y((r.loc.y + r.size.h) as u32),
        );
    }

    /// Fill `rects` (target coordinates) with an ARGB8888 value, no blending.
    fn clear_rects(&mut self, target: &Surface, argb: u32, rects: &[Rectangle<i32, Physical>]) {
        for batch in rects.chunks(RECTS_PER_BATCH) {
            self.reserve(64 + 4 * batch.len() as u32);
            self.emit_target(target);
            self.set_state(VIVS_DE_CLEAR_PIXEL_VALUE32, argb);
            self.dest_config(target, VIVS_DE_DEST_CONFIG_COMMAND_CLEAR);
            self.set_state(VIVS_DE_ALPHA_CONTROL, VIVS_DE_ALPHA_CONTROL_ENABLE_OFF);
            for r in batch {
                self.draw_rect(*r);
            }
            self.flush_stall();
        }
    }

    /// Stretch-blit `src` into each `(dst, src_rect)` pair, blending when the
    /// source has alpha or `alpha` < 1.
    fn blit(&mut self, target: &Surface, src: &Surface, alpha: f32, rects: &[(Rectangle<i32, Physical>, [f64; 4])]) {
        for batch in rects.chunks(RECTS_PER_BATCH) {
            self.reserve(80 + 20 * batch.len() as u32);
            self.emit_target(target);
            self.set_state_bo(VIVS_DE_SRC_ADDRESS, src, ffi::ETNA_RELOC_READ);
            self.set_state(VIVS_DE_SRC_STRIDE, src.stride);
            self.set_state(
                VIVS_DE_SRC_ROTATION_HEIGHT,
                VIVS_DE_SRC_ROTATION_HEIGHT_HEIGHT(src.height as u32),
            );
            self.set_state(
                VIVS_DE_SRC_ROTATION_CONFIG,
                VIVS_DE_SRC_ROTATION_CONFIG_WIDTH(src.width as u32),
            );
            self.set_state(
                VIVS_DE_SRC_CONFIG,
                VIVS_DE_SRC_CONFIG_SOURCE_FORMAT(src.format.de_format)
                    | VIVS_DE_SRC_CONFIG_SWIZZLE(src.format.swizzle),
            );
            self.set_state(VIVS_DE_SRC_EX_CONFIG, 0);
            self.dest_config(target, VIVS_DE_DEST_CONFIG_COMMAND_STRETCH_BLT);

            if alpha >= 1.0 && !src.format.alpha {
                self.set_state(VIVS_DE_ALPHA_CONTROL, VIVS_DE_ALPHA_CONTROL_ENABLE_OFF);
            } else {
                let mut alpha_modes = VIVS_DE_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_NORMAL;
                let mut color = VIVS_DE_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_DISABLE;
                self.set_state(VIVS_DE_ALPHA_CONTROL, VIVS_DE_ALPHA_CONTROL_ENABLE_ON);
                if alpha < 1.0 {
                    let a = (alpha.clamp(0.0, 1.0) * 255.0) as u32;
                    self.set_state(VIVS_DE_GLOBAL_SRC_COLOR, a << 24);
                    alpha_modes = if src.format.alpha {
                        VIVS_DE_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_SCALED
                    } else {
                        VIVS_DE_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_GLOBAL
                    };
                    color = VIVS_DE_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_ALPHA;
                }
                alpha_modes |= VIVS_DE_ALPHA_MODES_SRC_BLENDING_MODE(DE_BLENDMODE_ONE)
                    | VIVS_DE_ALPHA_MODES_DST_BLENDING_MODE(DE_BLENDMODE_INVERSED);
                self.set_state(VIVS_DE_ALPHA_MODES, alpha_modes);
                color |= VIVS_DE_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY_DISABLE
                    | VIVS_DE_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY_DISABLE
                    | VIVS_DE_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY_DISABLE;
                self.set_state(VIVS_DE_COLOR_MULTIPLY_MODES, color);
            }

            let lim_w = src.width.max(1);
            let lim_h = src.height.max(1);
            for (dst, [sx, sy, sw, sh]) in batch {
                // Keep at least one source pixel inside the buffer and derive
                // the factors from the unrounded extent (Weston's MMU-fault fix).
                let src_x = (sx.floor() as i32).max(0).min(lim_w - 1);
                let src_y = (sy.floor() as i32).max(0).min(lim_h - 1);
                let src_w = (sw.round() as i32).max(1).min(lim_w - src_x);
                let src_h = (sh.round() as i32).max(1).min(lim_h - src_y);
                self.set_state(
                    VIVS_DE_SRC_ORIGIN,
                    VIVS_DE_SRC_ORIGIN_X(src_x as u32) | VIVS_DE_SRC_ORIGIN_Y(src_y as u32),
                );
                self.set_state(
                    VIVS_DE_SRC_SIZE,
                    VIVS_DE_SRC_SIZE_X(src_w as u32) | VIVS_DE_SRC_SIZE_Y(src_h as u32),
                );
                self.set_state(
                    VIVS_DE_STRETCH_FACTOR_LOW,
                    VIVS_DE_STRETCH_FACTOR_LOW_X(stretch_factor(*sw, dst.size.w)),
                );
                self.set_state(
                    VIVS_DE_STRETCH_FACTOR_HIGH,
                    VIVS_DE_STRETCH_FACTOR_HIGH_Y(stretch_factor(*sh, dst.size.h)),
                );
                self.draw_rect(*dst);
            }
            self.flush_stall();
        }
    }

    fn submit(&mut self) {
        let s = unsafe { &*self.stream };
        if s.offset > 0 {
            unsafe { ffi::etna_cmd_stream_flush(self.stream) };
        }
    }
}

/// 16.16 source step per destination pixel, endpoint to endpoint.
fn stretch_factor(src: f64, dst: i32) -> u32 {
    if dst <= 1 || src <= 1.0 {
        return 0;
    }
    ((src - 1.0) * 65536.0 / (dst - 1) as f64) as u32
}

fn linear(formats: &[DrmFourcc]) -> FormatSet {
    formats
        .iter()
        .map(|code| DrmFormat {
            code: *code,
            modifier: DrmModifier::Linear,
        })
        .collect()
}

fn argb(color: Color32F) -> u32 {
    let c = |v: f32| ((v.clamp(0.0, 1.0) * 255.0).round()) as u32;
    c(color.a()) << 24 | c(color.r()) << 16 | c(color.g()) << 8 | c(color.b())
}

fn warn_once(what: &'static str) {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| warn!("etnaviv 2D renderer: {what} not supported, drawing untransformed"));
}

/// A frame of the [`EtnaRenderer`]
pub struct EtnaFrame<'frame, 'buffer> {
    renderer: &'frame mut EtnaRenderer,
    target: &'frame mut EtnaTarget<'buffer>,
    transform: Transform,
    output_size: Size<i32, Physical>,
    finished: bool,
}

impl fmt::Debug for EtnaFrame<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EtnaFrame").finish_non_exhaustive()
    }
}

impl EtnaFrame<'_, '_> {
    fn bounds(&self) -> Rectangle<i32, Physical> {
        Rectangle::from_size((self.target.surface.width, self.target.surface.height).into())
    }

    /// `damage` is relative to `dst`; returns the parts inside both, in target space.
    fn damaged(&self, dst: Rectangle<i32, Physical>, damage: &[Rectangle<i32, Physical>]) -> Vec<Rectangle<i32, Physical>> {
        let bounds = self.bounds();
        damage
            .iter()
            .filter_map(|d| {
                Rectangle::new(dst.loc + d.loc, d.size)
                    .intersection(dst)
                    .and_then(|r| r.intersection(bounds))
            })
            .filter(|r| !r.is_empty())
            .collect()
    }
}

impl Frame for EtnaFrame<'_, '_> {
    type Error = EtnaError;
    type TextureId = EtnaTexture;

    fn context_id(&self) -> ContextId<EtnaTexture> {
        self.renderer.context_id.clone()
    }

    fn clear(&mut self, color: Color32F, at: &[Rectangle<i32, Physical>]) -> Result<(), Self::Error> {
        let bounds = self.bounds();
        let rects: Vec<_> = at.iter().filter_map(|r| r.intersection(bounds)).collect();
        let target = self.target.surface.clone();
        self.renderer.clear_rects(&target, argb(color), &rects);
        Ok(())
    }

    fn draw_solid(
        &mut self,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        color: Color32F,
    ) -> Result<(), Self::Error> {
        let rects = self.damaged(dst, damage);
        if rects.is_empty() {
            return Ok(());
        }
        let target = self.target.surface.clone();
        if color.is_opaque() {
            self.renderer.clear_rects(&target, argb(color), &rects);
        } else {
            let src = self.renderer.solid_surface(argb(color))?;
            let pairs: Vec<_> = rects.into_iter().map(|r| (r, [0.0, 0.0, 1.0, 1.0])).collect();
            self.renderer.blit(&target, &src, 1.0, &pairs);
        }
        Ok(())
    }

    fn render_texture_from_to(
        &mut self,
        texture: &EtnaTexture,
        src: Rectangle<f64, BufferCoords>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        _opaque_regions: &[Rectangle<i32, Physical>],
        src_transform: Transform,
        alpha: f32,
    ) -> Result<(), Self::Error> {
        if src_transform != Transform::Normal {
            warn_once("buffer transform");
        }
        if dst.is_empty() {
            return Ok(());
        }
        let scale_x = src.size.w / dst.size.w as f64;
        let scale_y = src.size.h / dst.size.h as f64;
        let pairs: Vec<_> = self
            .damaged(dst, damage)
            .into_iter()
            .map(|r| {
                let sx = src.loc.x + (r.loc.x - dst.loc.x) as f64 * scale_x;
                let sy = src.loc.y + (r.loc.y - dst.loc.y) as f64 * scale_y;
                (r, [sx, sy, r.size.w as f64 * scale_x, r.size.h as f64 * scale_y])
            })
            .collect();
        if pairs.is_empty() {
            return Ok(());
        }
        let target = self.target.surface.clone();
        self.renderer.blit(&target, &texture.0, alpha, &pairs);
        Ok(())
    }

    fn transformation(&self) -> Transform {
        self.transform
    }

    fn output_size(&self) -> Size<i32, Physical> {
        self.output_size
    }

    fn wait(&mut self, sync: &SyncPoint) -> Result<(), Self::Error> {
        sync.wait().map_err(|_| EtnaError::SyncInterrupted)
    }

    fn finish(mut self) -> Result<SyncPoint, Self::Error> {
        self.finished = true;
        self.renderer.submit();
        // etnaviv attaches its fence to the buffers' reservation objects;
        // KMS waits on it implicitly.
        Ok(SyncPoint::signaled())
    }
}

impl Drop for EtnaFrame<'_, '_> {
    fn drop(&mut self) {
        if !self.finished {
            self.renderer.submit();
        }
    }
}

impl RendererSuper for EtnaRenderer {
    type Error = EtnaError;
    type TextureId = EtnaTexture;
    type Framebuffer<'buffer> = EtnaTarget<'buffer>;
    type Frame<'frame, 'buffer>
        = EtnaFrame<'frame, 'buffer>
    where
        'buffer: 'frame;
}

impl Renderer for EtnaRenderer {
    fn context_id(&self) -> ContextId<EtnaTexture> {
        self.context_id.clone()
    }

    fn downscale_filter(&mut self, _filter: TextureFilter) -> Result<(), Self::Error> {
        Ok(())
    }

    fn upscale_filter(&mut self, _filter: TextureFilter) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_debug_flags(&mut self, flags: DebugFlags) {
        self.debug_flags = flags;
    }

    fn debug_flags(&self) -> DebugFlags {
        self.debug_flags
    }

    fn render<'frame, 'buffer>(
        &'frame mut self,
        target: &'frame mut EtnaTarget<'buffer>,
        output_size: Size<i32, Physical>,
        dst_transform: Transform,
    ) -> Result<EtnaFrame<'frame, 'buffer>, Self::Error>
    where
        'buffer: 'frame,
    {
        if dst_transform != Transform::Normal {
            warn_once("output transform");
        }
        self.dmabufs.retain(|(weak, _)| !weak.is_gone());
        Ok(EtnaFrame {
            renderer: self,
            target,
            transform: dst_transform,
            output_size,
            finished: false,
        })
    }

    fn wait(&mut self, sync: &SyncPoint) -> Result<(), Self::Error> {
        sync.wait().map_err(|_| EtnaError::SyncInterrupted)
    }

    fn cleanup_texture_cache(&mut self) -> Result<(), Self::Error> {
        self.dmabufs.retain(|(weak, _)| !weak.is_gone());
        Ok(())
    }

    fn invalidate_caches(&mut self) -> Result<(), Self::Error> {
        self.dmabufs.clear();
        self.solids.clear();
        Ok(())
    }
}

/// Copy `rect` of a `src_stride`-pitched image into `dst`.
fn copy_rect(dst: *mut u8, dst_stride: u32, src: &[u8], src_stride: usize, bpp: u32, rect: Rectangle<i32, BufferCoords>) {
    let bytes = rect.size.w as usize * bpp as usize;
    for y in rect.loc.y..rect.loc.y + rect.size.h {
        let s = y as usize * src_stride + rect.loc.x as usize * bpp as usize;
        if s + bytes > src.len() {
            break;
        }
        let d = y as usize * dst_stride as usize + rect.loc.x as usize * bpp as usize;
        unsafe { std::ptr::copy_nonoverlapping(src.as_ptr().add(s), dst.add(d), bytes) };
    }
}

impl ImportMem for EtnaRenderer {
    fn import_memory(
        &mut self,
        data: &[u8],
        format: DrmFourcc,
        size: Size<i32, BufferCoords>,
        _flipped: bool,
    ) -> Result<EtnaTexture, Self::Error> {
        let pix = pix_format(format).ok_or(EtnaError::UnsupportedFormat(format))?;
        let surface = self.new_surface(pix, size.w, size.h)?;
        let src_stride = (size.w as u32 * pix.bpp) as usize;
        surface.bo.with_map(ffi::DRM_ETNA_PREP_WRITE, |map| {
            copy_rect(map, surface.stride, data, src_stride, pix.bpp, Rectangle::from_size(size));
        })?;
        Ok(EtnaTexture(Arc::new(surface)))
    }

    fn update_memory(
        &mut self,
        texture: &EtnaTexture,
        data: &[u8],
        region: Rectangle<i32, BufferCoords>,
    ) -> Result<(), Self::Error> {
        let surface = &texture.0;
        let src_stride = (surface.width as u32 * surface.format.bpp) as usize;
        let region = region
            .intersection(Rectangle::from_size((surface.width, surface.height).into()))
            .unwrap_or_default();
        surface.bo.with_map(ffi::DRM_ETNA_PREP_WRITE, |map| {
            copy_rect(map, surface.stride, data, src_stride, surface.format.bpp, region);
        })
    }

    fn mem_formats(&self) -> Box<dyn Iterator<Item = DrmFourcc>> {
        Box::new(MEM_FORMATS.iter().copied())
    }
}

/// Per-surface shm upload target, so damage relative to the previous commit
/// applies whichever wl_buffer the client attached.
#[cfg(feature = "wayland_frontend")]
#[derive(Default)]
struct ShmCache(Mutex<Option<EtnaTexture>>);

#[cfg(feature = "wayland_frontend")]
impl ImportMemWl for EtnaRenderer {
    fn import_shm_buffer(
        &mut self,
        buffer: &wl_buffer::WlBuffer,
        surface: Option<&SurfaceData>,
        damage: &[Rectangle<i32, BufferCoords>],
    ) -> Result<EtnaTexture, Self::Error> {
        shm::with_buffer_contents(buffer, |ptr, len, data| {
            let fourcc = shm::shm_format_to_fourcc(data.format)
                .ok_or(EtnaError::Unsupported("shm format"))?;
            let pix = pix_format(fourcc).ok_or(EtnaError::UnsupportedFormat(fourcc))?;
            let start = data.offset as usize;
            let end = start + (data.stride * data.height) as usize;
            if end > len {
                return Err(EtnaError::ImportFailed);
            }
            let bytes = unsafe { std::slice::from_raw_parts(ptr.add(start), end - start) };
            let size: Size<i32, BufferCoords> = (data.width, data.height).into();

            let cache = surface.map(|s| {
                s.data_map.insert_if_missing_threadsafe(ShmCache::default);
                s.data_map.get::<ShmCache>().unwrap()
            });
            let mut slot = cache.map(|c| c.0.lock().unwrap());
            let reuse = slot.as_ref().and_then(|s| s.as_ref()).filter(|t| {
                t.0.width == size.w && t.0.height == size.h && t.0.format.fourcc == fourcc
            });
            let (texture, full) = match reuse {
                Some(t) => (t.clone(), false),
                None => (EtnaTexture(Arc::new(self.new_surface(pix, size.w, size.h)?)), true),
            };
            let bounds = Rectangle::from_size(size);
            let rects: Vec<_> = if full {
                vec![bounds]
            } else {
                damage.iter().filter_map(|d| d.intersection(bounds)).collect()
            };
            if !rects.is_empty() {
                let s = &texture.0;
                s.bo.with_map(ffi::DRM_ETNA_PREP_WRITE, |map| {
                    for r in rects {
                        copy_rect(map, s.stride, bytes, data.stride as usize, pix.bpp, r);
                    }
                })?;
            }
            if let Some(slot) = slot.as_mut() {
                **slot = Some(texture.clone());
            }
            Ok(texture)
        })?
    }
}

impl ImportDma for EtnaRenderer {
    fn import_dmabuf(
        &mut self,
        dmabuf: &Dmabuf,
        _damage: Option<&[Rectangle<i32, BufferCoords>]>,
    ) -> Result<EtnaTexture, Self::Error> {
        self.import_dmabuf_surface(dmabuf).map(EtnaTexture)
    }

    fn dmabuf_formats(&self) -> FormatSet {
        linear(MEM_FORMATS)
    }
}

#[cfg(feature = "wayland_frontend")]
impl ImportDmaWl for EtnaRenderer {}

impl Bind<Dmabuf> for EtnaRenderer {
    fn bind<'a>(&mut self, target: &'a mut Dmabuf) -> Result<EtnaTarget<'a>, Self::Error> {
        let surface = self.import_dmabuf_surface(target)?;
        if !TARGET_FORMATS.contains(&surface.format.fourcc) {
            return Err(EtnaError::UnsupportedFormat(surface.format.fourcc));
        }
        Ok(EtnaTarget {
            surface,
            _buffer: PhantomData,
        })
    }

    fn supported_formats(&self) -> Option<FormatSet> {
        Some(linear(TARGET_FORMATS))
    }
}

/// Guard wrapping the [`EtnaRenderer`] of an [`EtnaFrame`]
#[derive(Debug)]
pub struct EtnaFrameGuard<'a, 'frame> {
    renderer: &'a mut &'frame mut EtnaRenderer,
}

impl AsRef<EtnaRenderer> for EtnaFrameGuard<'_, '_> {
    fn as_ref(&self) -> &EtnaRenderer {
        self.renderer
    }
}

impl AsMut<EtnaRenderer> for EtnaFrameGuard<'_, '_> {
    fn as_mut(&mut self) -> &mut EtnaRenderer {
        self.renderer
    }
}

impl<'a, 'frame, 'buffer> FrameContext<'a, 'frame, 'buffer, EtnaRenderer> for EtnaFrame<'frame, 'buffer>
where
    'frame: 'a,
{
    type Guard = EtnaFrameGuard<'a, 'frame>;

    fn renderer(&'a mut self) -> Self::Guard {
        EtnaFrameGuard {
            renderer: &mut self.renderer,
        }
    }
}
