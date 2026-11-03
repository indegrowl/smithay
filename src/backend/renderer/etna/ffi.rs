//! Minimal bindings to libdrm_etnaviv.
#![allow(non_camel_case_types, missing_docs, dead_code)]

use std::os::raw::{c_int, c_uint, c_void};

#[repr(C)]
pub struct etna_device {
    _p: [u8; 0],
}
#[repr(C)]
pub struct etna_gpu {
    _p: [u8; 0],
}
#[repr(C)]
pub struct etna_pipe {
    _p: [u8; 0],
}
#[repr(C)]
pub struct etna_bo {
    _p: [u8; 0],
}

#[repr(C)]
pub struct etna_cmd_stream {
    pub buffer: *mut u32,
    /// In 32-bit words.
    pub offset: u32,
    /// In 32-bit words.
    pub size: u32,
}

#[repr(C)]
pub struct etna_reloc {
    pub bo: *mut etna_bo,
    pub flags: u32,
    pub offset: u32,
}

pub const ETNA_PIPE_2D: c_int = 1;
pub const ETNA_GPU_FEATURES_0: c_int = 0x3;
pub const ETNA_GPU_FEATURES_6: c_int = 0x9;

pub const DRM_ETNA_GEM_CACHE_WC: u32 = 0x0002_0000;
pub const DRM_ETNA_PREP_READ: u32 = 0x01;
pub const DRM_ETNA_PREP_WRITE: u32 = 0x02;

pub const ETNA_RELOC_READ: u32 = 0x0001;
pub const ETNA_RELOC_WRITE: u32 = 0x0002;

#[link(name = "drm_etnaviv")]
unsafe extern "C" {
    pub fn etna_device_new(fd: c_int) -> *mut etna_device;
    pub fn etna_device_del(dev: *mut etna_device);
    pub fn etna_gpu_new(dev: *mut etna_device, core: c_uint) -> *mut etna_gpu;
    pub fn etna_gpu_del(gpu: *mut etna_gpu);
    pub fn etna_gpu_get_param(gpu: *mut etna_gpu, param: c_int, value: *mut u64) -> c_int;
    pub fn etna_pipe_new(gpu: *mut etna_gpu, id: c_int) -> *mut etna_pipe;
    pub fn etna_pipe_del(pipe: *mut etna_pipe);
    pub fn etna_bo_new(dev: *mut etna_device, size: u32, flags: u32) -> *mut etna_bo;
    pub fn etna_bo_from_dmabuf(dev: *mut etna_device, fd: c_int) -> *mut etna_bo;
    pub fn etna_bo_del(bo: *mut etna_bo);
    pub fn etna_bo_map(bo: *mut etna_bo) -> *mut c_void;
    pub fn etna_bo_cpu_prep(bo: *mut etna_bo, op: u32) -> c_int;
    pub fn etna_bo_cpu_fini(bo: *mut etna_bo);
    pub fn etna_cmd_stream_new(
        pipe: *mut etna_pipe,
        size: u32,
        reset_notify: Option<unsafe extern "C" fn(*mut etna_cmd_stream, *mut c_void)>,
        priv_: *mut c_void,
    ) -> *mut etna_cmd_stream;
    pub fn etna_cmd_stream_del(stream: *mut etna_cmd_stream);
    pub fn etna_cmd_stream_flush(stream: *mut etna_cmd_stream);
    pub fn etna_cmd_stream_reloc(stream: *mut etna_cmd_stream, r: *const etna_reloc);
}
