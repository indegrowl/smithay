//! Vivante register and command-stream definitions, generated from the
//! rnndb headers in Weston's etnaviv renderer (common, cmdstream, state, state_2d).
#![allow(dead_code, non_snake_case, non_upper_case_globals, missing_docs, clippy::unreadable_literal)]

pub const PIPE_ID_PIPE_3D: u32 = 0x00000000;
pub const PIPE_ID_PIPE_2D: u32 = 0x00000001;
pub const SYNC_RECIPIENT_FE: u32 = 0x00000001;
pub const SYNC_RECIPIENT_RA: u32 = 0x00000005;
pub const SYNC_RECIPIENT_PE: u32 = 0x00000007;
pub const SYNC_RECIPIENT_DE: u32 = 0x0000000b;
pub const SYNC_RECIPIENT_BLT: u32 = 0x00000010;
pub const ENDIAN_MODE_NO_SWAP: u32 = 0x00000000;
pub const ENDIAN_MODE_SWAP_16: u32 = 0x00000001;
pub const ENDIAN_MODE_SWAP_32: u32 = 0x00000002;
pub const chipModel_GC200: u32 = 0x00000200;
pub const chipModel_GC300: u32 = 0x00000300;
pub const chipModel_GC320: u32 = 0x00000320;
pub const chipModel_GC328: u32 = 0x00000328;
pub const chipModel_GC350: u32 = 0x00000350;
pub const chipModel_GC355: u32 = 0x00000355;
pub const chipModel_GC400: u32 = 0x00000400;
pub const chipModel_GC410: u32 = 0x00000410;
pub const chipModel_GC420: u32 = 0x00000420;
pub const chipModel_GC428: u32 = 0x00000428;
pub const chipModel_GC450: u32 = 0x00000450;
pub const chipModel_GC500: u32 = 0x00000500;
pub const chipModel_GC520: u32 = 0x00000520;
pub const chipModel_GC530: u32 = 0x00000530;
pub const chipModel_GC600: u32 = 0x00000600;
pub const chipModel_GC700: u32 = 0x00000700;
pub const chipModel_GC800: u32 = 0x00000800;
pub const chipModel_GC860: u32 = 0x00000860;
pub const chipModel_GC880: u32 = 0x00000880;
pub const chipModel_GC900: u32 = 0x00000900;
pub const chipModel_GC1000: u32 = 0x00001000;
pub const chipModel_GC1500: u32 = 0x00001500;
pub const chipModel_GC2000: u32 = 0x00002000;
pub const chipModel_GC2100: u32 = 0x00002100;
pub const chipModel_GC2200: u32 = 0x00002200;
pub const chipModel_GC2500: u32 = 0x00002500;
pub const chipModel_GC3000: u32 = 0x00003000;
pub const chipModel_GC4000: u32 = 0x00004000;
pub const chipModel_GC5000: u32 = 0x00005000;
pub const chipModel_GC5200: u32 = 0x00005200;
pub const chipModel_GC6400: u32 = 0x00006400;
pub const chipModel_GC7000: u32 = 0x00007000;
pub const chipModel_GC7400: u32 = 0x00007400;
pub const chipModel_GC8000: u32 = 0x00008000;
pub const chipModel_GC8100: u32 = 0x00008100;
pub const chipModel_GC8200: u32 = 0x00008200;
pub const chipModel_GC8400: u32 = 0x00008400;
pub const RGBA_BITS_R: u32 = 0x00000001;
pub const RGBA_BITS_G: u32 = 0x00000002;
pub const RGBA_BITS_B: u32 = 0x00000004;
pub const RGBA_BITS_A: u32 = 0x00000008;
pub const chipFeatures_FAST_CLEAR: u32 = 0x00000001;
pub const chipFeatures_SPECIAL_ANTI_ALIASING: u32 = 0x00000002;
pub const chipFeatures_PIPE_3D: u32 = 0x00000004;
pub const chipFeatures_DXT_TEXTURE_COMPRESSION: u32 = 0x00000008;
pub const chipFeatures_DEBUG_MODE: u32 = 0x00000010;
pub const chipFeatures_Z_COMPRESSION: u32 = 0x00000020;
pub const chipFeatures_YUV420_SCALER: u32 = 0x00000040;
pub const chipFeatures_MSAA: u32 = 0x00000080;
pub const chipFeatures_DC: u32 = 0x00000100;
pub const chipFeatures_PIPE_2D: u32 = 0x00000200;
pub const chipFeatures_ETC1_TEXTURE_COMPRESSION: u32 = 0x00000400;
pub const chipFeatures_FAST_SCALER: u32 = 0x00000800;
pub const chipFeatures_HIGH_DYNAMIC_RANGE: u32 = 0x00001000;
pub const chipFeatures_YUV420_TILER: u32 = 0x00002000;
pub const chipFeatures_MODULE_CG: u32 = 0x00004000;
pub const chipFeatures_MIN_AREA: u32 = 0x00008000;
pub const chipFeatures_NO_EARLY_Z: u32 = 0x00010000;
pub const chipFeatures_NO_422_TEXTURE: u32 = 0x00020000;
pub const chipFeatures_BUFFER_INTERLEAVING: u32 = 0x00040000;
pub const chipFeatures_BYTE_WRITE_2D: u32 = 0x00080000;
pub const chipFeatures_NO_SCALER: u32 = 0x00100000;
pub const chipFeatures_YUY2_AVERAGING: u32 = 0x00200000;
pub const chipFeatures_HALF_PE_CACHE: u32 = 0x00400000;
pub const chipFeatures_HALF_TX_CACHE: u32 = 0x00800000;
pub const chipFeatures_YUY2_RENDER_TARGET: u32 = 0x01000000;
pub const chipFeatures_MEM32: u32 = 0x02000000;
pub const chipFeatures_PIPE_VG: u32 = 0x04000000;
pub const chipFeatures_VGTS: u32 = 0x08000000;
pub const chipFeatures_FE20: u32 = 0x10000000;
pub const chipFeatures_BYTE_WRITE_3D: u32 = 0x20000000;
pub const chipFeatures_RS_YUV_TARGET: u32 = 0x40000000;
pub const chipFeatures_32_BIT_INDICES: u32 = 0x80000000;
pub const chipMinorFeatures0_FLIP_Y: u32 = 0x00000001;
pub const chipMinorFeatures0_DUAL_RETURN_BUS: u32 = 0x00000002;
pub const chipMinorFeatures0_ENDIANNESS_CONFIG: u32 = 0x00000004;
pub const chipMinorFeatures0_TEXTURE_8K: u32 = 0x00000008;
pub const chipMinorFeatures0_CORRECT_TEXTURE_CONVERTER: u32 = 0x00000010;
pub const chipMinorFeatures0_SPECIAL_MSAA_LOD: u32 = 0x00000020;
pub const chipMinorFeatures0_FAST_CLEAR_FLUSH: u32 = 0x00000040;
pub const chipMinorFeatures0_2DPE20: u32 = 0x00000080;
pub const chipMinorFeatures0_CORRECT_AUTO_DISABLE: u32 = 0x00000100;
pub const chipMinorFeatures0_RENDERTARGET_8K: u32 = 0x00000200;
pub const chipMinorFeatures0_2BITPERTILE: u32 = 0x00000400;
pub const chipMinorFeatures0_SEPARATE_TILE_STATUS_WHEN_INTERLEAVED: u32 = 0x00000800;
pub const chipMinorFeatures0_SUPER_TILED: u32 = 0x00001000;
pub const chipMinorFeatures0_VG_20: u32 = 0x00002000;
pub const chipMinorFeatures0_TS_EXTENDED_COMMANDS: u32 = 0x00004000;
pub const chipMinorFeatures0_COMPRESSION_FIFO_FIXED: u32 = 0x00008000;
pub const chipMinorFeatures0_HAS_SIGN_FLOOR_CEIL: u32 = 0x00010000;
pub const chipMinorFeatures0_VG_FILTER: u32 = 0x00020000;
pub const chipMinorFeatures0_VG_21: u32 = 0x00040000;
pub const chipMinorFeatures0_SHADER_HAS_W: u32 = 0x00080000;
pub const chipMinorFeatures0_HAS_SQRT_TRIG: u32 = 0x00100000;
pub const chipMinorFeatures0_MORE_MINOR_FEATURES: u32 = 0x00200000;
pub const chipMinorFeatures0_MC20: u32 = 0x00400000;
pub const chipMinorFeatures0_MSAA_SIDEBAND: u32 = 0x00800000;
pub const chipMinorFeatures0_BUG_FIXES0: u32 = 0x01000000;
pub const chipMinorFeatures0_VAA: u32 = 0x02000000;
pub const chipMinorFeatures0_BYPASS_IN_MSAA: u32 = 0x04000000;
pub const chipMinorFeatures0_HZ: u32 = 0x08000000;
pub const chipMinorFeatures0_NEW_TEXTURE: u32 = 0x10000000;
pub const chipMinorFeatures0_2D_A8_TARGET: u32 = 0x20000000;
pub const chipMinorFeatures0_CORRECT_STENCIL: u32 = 0x40000000;
pub const chipMinorFeatures0_ENHANCE_VR: u32 = 0x80000000;
pub const chipMinorFeatures1_RSUV_SWIZZLE: u32 = 0x00000001;
pub const chipMinorFeatures1_V2_COMPRESSION: u32 = 0x00000002;
pub const chipMinorFeatures1_VG_DOUBLE_BUFFER: u32 = 0x00000004;
pub const chipMinorFeatures1_EXTRA_EVENT_STATES: u32 = 0x00000008;
pub const chipMinorFeatures1_NO_STRIPING_NEEDED: u32 = 0x00000010;
pub const chipMinorFeatures1_TEXTURE_STRIDE: u32 = 0x00000020;
pub const chipMinorFeatures1_BUG_FIXES3: u32 = 0x00000040;
pub const chipMinorFeatures1_AUTO_DISABLE: u32 = 0x00000080;
pub const chipMinorFeatures1_AUTO_RESTART_TS: u32 = 0x00000100;
pub const chipMinorFeatures1_DISABLE_PE_GATING: u32 = 0x00000200;
pub const chipMinorFeatures1_L2_WINDOWING: u32 = 0x00000400;
pub const chipMinorFeatures1_HALF_FLOAT: u32 = 0x00000800;
pub const chipMinorFeatures1_PIXEL_DITHER: u32 = 0x00001000;
pub const chipMinorFeatures1_TWO_STENCIL_REFERENCE: u32 = 0x00002000;
pub const chipMinorFeatures1_EXTENDED_PIXEL_FORMAT: u32 = 0x00004000;
pub const chipMinorFeatures1_CORRECT_MIN_MAX_DEPTH: u32 = 0x00008000;
pub const chipMinorFeatures1_2D_DITHER: u32 = 0x00010000;
pub const chipMinorFeatures1_BUG_FIXES5: u32 = 0x00020000;
pub const chipMinorFeatures1_NEW_2D: u32 = 0x00040000;
pub const chipMinorFeatures1_NEW_FP: u32 = 0x00080000;
pub const chipMinorFeatures1_TEXTURE_HALIGN: u32 = 0x00100000;
pub const chipMinorFeatures1_NON_POWER_OF_TWO: u32 = 0x00200000;
pub const chipMinorFeatures1_LINEAR_TEXTURE_SUPPORT: u32 = 0x00400000;
pub const chipMinorFeatures1_HALTI0: u32 = 0x00800000;
pub const chipMinorFeatures1_CORRECT_OVERFLOW_VG: u32 = 0x01000000;
pub const chipMinorFeatures1_NEGATIVE_LOG_FIX: u32 = 0x02000000;
pub const chipMinorFeatures1_RESOLVE_OFFSET: u32 = 0x04000000;
pub const chipMinorFeatures1_OK_TO_GATE_AXI_CLOCK: u32 = 0x08000000;
pub const chipMinorFeatures1_MMU_VERSION: u32 = 0x10000000;
pub const chipMinorFeatures1_WIDE_LINE: u32 = 0x20000000;
pub const chipMinorFeatures1_BUG_FIXES6: u32 = 0x40000000;
pub const chipMinorFeatures1_FC_FLUSH_STALL: u32 = 0x80000000;
pub const chipMinorFeatures2_LINE_LOOP: u32 = 0x00000001;
pub const chipMinorFeatures2_LOGIC_OP: u32 = 0x00000002;
pub const chipMinorFeatures2_SEAMLESS_CUBE_MAP: u32 = 0x00000004;
pub const chipMinorFeatures2_SUPERTILED_TEXTURE: u32 = 0x00000008;
pub const chipMinorFeatures2_LINEAR_PE: u32 = 0x00000010;
pub const chipMinorFeatures2_RECT_PRIMITIVE: u32 = 0x00000020;
pub const chipMinorFeatures2_COMPOSITION: u32 = 0x00000040;
pub const chipMinorFeatures2_CORRECT_AUTO_DISABLE_COUNT: u32 = 0x00000080;
pub const chipMinorFeatures2_PE_SWIZZLE: u32 = 0x00000100;
pub const chipMinorFeatures2_END_EVENT: u32 = 0x00000200;
pub const chipMinorFeatures2_S1S8: u32 = 0x00000400;
pub const chipMinorFeatures2_HALTI1: u32 = 0x00000800;
pub const chipMinorFeatures2_RGB888: u32 = 0x00001000;
pub const chipMinorFeatures2_TX__YUV_ASSEMBLER: u32 = 0x00002000;
pub const chipMinorFeatures2_DYNAMIC_FREQUENCY_SCALING: u32 = 0x00004000;
pub const chipMinorFeatures2_TX_FILTER: u32 = 0x00008000;
pub const chipMinorFeatures2_FULL_DIRECTFB: u32 = 0x00010000;
pub const chipMinorFeatures2_2D_TILING: u32 = 0x00020000;
pub const chipMinorFeatures2_THREAD_WALKER_IN_PS: u32 = 0x00040000;
pub const chipMinorFeatures2_TILE_FILLER: u32 = 0x00080000;
pub const chipMinorFeatures2_YUV_STANDARD: u32 = 0x00100000;
pub const chipMinorFeatures2_2D_MULTI_SOURCE_BLIT: u32 = 0x00200000;
pub const chipMinorFeatures2_YUV_CONVERSION: u32 = 0x00400000;
pub const chipMinorFeatures2_FLUSH_FIXED_2D: u32 = 0x00800000;
pub const chipMinorFeatures2_INTERLEAVER: u32 = 0x01000000;
pub const chipMinorFeatures2_MIXED_STREAMS: u32 = 0x02000000;
pub const chipMinorFeatures2_2D_420_L2CACHE: u32 = 0x04000000;
pub const chipMinorFeatures2_BUG_FIXES7: u32 = 0x08000000;
pub const chipMinorFeatures2_2D_NO_INDEX8_BRUSH: u32 = 0x10000000;
pub const chipMinorFeatures2_TEXTURE_TILED_READ: u32 = 0x20000000;
pub const chipMinorFeatures2_DECOMPRESS_Z16: u32 = 0x40000000;
pub const chipMinorFeatures2_BUG_FIXES8: u32 = 0x80000000;
pub const chipMinorFeatures3_ROTATION_STALL_FIX: u32 = 0x00000001;
pub const chipMinorFeatures3_OCL_ONLY: u32 = 0x00000002;
pub const chipMinorFeatures3_2D_MULTI_SOURCE_BLT_EX: u32 = 0x00000004;
pub const chipMinorFeatures3_INSTRUCTION_CACHE: u32 = 0x00000008;
pub const chipMinorFeatures3_GEOMETRY_SHADER: u32 = 0x00000010;
pub const chipMinorFeatures3_TEX_COMPRESSION_SUPERTILED: u32 = 0x00000020;
pub const chipMinorFeatures3_GENERICS: u32 = 0x00000040;
pub const chipMinorFeatures3_BUG_FIXES9: u32 = 0x00000080;
pub const chipMinorFeatures3_FAST_MSAA: u32 = 0x00000100;
pub const chipMinorFeatures3_WCLIP: u32 = 0x00000200;
pub const chipMinorFeatures3_BUG_FIXES10: u32 = 0x00000400;
pub const chipMinorFeatures3_UNIFIED_SAMPLERS: u32 = 0x00000800;
pub const chipMinorFeatures3_BUG_FIXES11: u32 = 0x00001000;
pub const chipMinorFeatures3_PERFORMANCE_COUNTERS: u32 = 0x00002000;
pub const chipMinorFeatures3_HAS_FAST_TRANSCENDENTALS: u32 = 0x00004000;
pub const chipMinorFeatures3_BUG_FIXES12: u32 = 0x00008000;
pub const chipMinorFeatures3_BUG_FIXES13: u32 = 0x00010000;
pub const chipMinorFeatures3_DE_ENHANCEMENTS1: u32 = 0x00020000;
pub const chipMinorFeatures3_ACE: u32 = 0x00040000;
pub const chipMinorFeatures3_TX_ENHANCEMENTS1: u32 = 0x00080000;
pub const chipMinorFeatures3_SH_ENHANCEMENTS1: u32 = 0x00100000;
pub const chipMinorFeatures3_SH_ENHANCEMENTS2: u32 = 0x00200000;
pub const chipMinorFeatures3_PE_ENHANCEMENTS1: u32 = 0x00400000;
pub const chipMinorFeatures3_2D_FC_SOURCE: u32 = 0x00800000;
pub const chipMinorFeatures3_BUG_FIXES_14: u32 = 0x01000000;
pub const chipMinorFeatures3_POWER_OPTIMIZATIONS_0: u32 = 0x02000000;
pub const chipMinorFeatures3_NEW_HZ: u32 = 0x04000000;
pub const chipMinorFeatures3_PE_DITHER_FIX: u32 = 0x08000000;
pub const chipMinorFeatures3_DE_ENHANCEMENTS3: u32 = 0x10000000;
pub const chipMinorFeatures3_SH_ENHANCEMENTS3: u32 = 0x20000000;
pub const chipMinorFeatures3_SH_ENHANCEMENTS4: u32 = 0x40000000;
pub const chipMinorFeatures3_TX_ENHANCEMENTS2: u32 = 0x80000000;
pub const chipMinorFeatures4_FE_ENHANCEMENTS1: u32 = 0x00000001;
pub const chipMinorFeatures4_PE_ENHANCEMENTS2: u32 = 0x00000002;
pub const chipMinorFeatures4_FRUSTUM_CLIP_FIX: u32 = 0x00000004;
pub const chipMinorFeatures4_DE_NO_GAMMA: u32 = 0x00000008;
pub const chipMinorFeatures4_PA_ENHANCEMENTS_2: u32 = 0x00000010;
pub const chipMinorFeatures4_2D_GAMMA: u32 = 0x00000020;
pub const chipMinorFeatures4_SINGLE_BUFFER: u32 = 0x00000040;
pub const chipMinorFeatures4_HI_ENHANCEMENTS_1: u32 = 0x00000080;
pub const chipMinorFeatures4_TX_ENHANCEMENTS_3: u32 = 0x00000100;
pub const chipMinorFeatures4_SH_ENHANCEMENTS_5: u32 = 0x00000200;
pub const chipMinorFeatures4_FE_ENHANCEMENTS_2: u32 = 0x00000400;
pub const chipMinorFeatures4_TX_LERP_PRECISION_FIX: u32 = 0x00000800;
pub const chipMinorFeatures4_2D_COLOR_SPACE_CONVERSION: u32 = 0x00001000;
pub const chipMinorFeatures4_TEXTURE_ASTC: u32 = 0x00002000;
pub const chipMinorFeatures4_PE_ENHANCEMENTS_4: u32 = 0x00004000;
pub const chipMinorFeatures4_MC_ENHANCEMENTS_1: u32 = 0x00008000;
pub const chipMinorFeatures4_HALTI2: u32 = 0x00010000;
pub const chipMinorFeatures4_2D_MIRROR_EXTENSION: u32 = 0x00020000;
pub const chipMinorFeatures4_SMALL_MSAA: u32 = 0x00040000;
pub const chipMinorFeatures4_BUG_FIXES_17: u32 = 0x00080000;
pub const chipMinorFeatures4_NEW_RA: u32 = 0x00100000;
pub const chipMinorFeatures4_2D_OPF_YUV_OUTPUT: u32 = 0x00200000;
pub const chipMinorFeatures4_2D_MULTI_SOURCE_BLT_EX2: u32 = 0x00400000;
pub const chipMinorFeatures4_NO_USER_CSC: u32 = 0x00800000;
pub const chipMinorFeatures4_ZFIXES: u32 = 0x01000000;
pub const chipMinorFeatures4_BUG_FIXES18: u32 = 0x02000000;
pub const chipMinorFeatures4_2D_COMPRESSION: u32 = 0x04000000;
pub const chipMinorFeatures4_PROBE: u32 = 0x08000000;
pub const chipMinorFeatures4_MEDIUM_PRECISION: u32 = 0x10000000;
pub const chipMinorFeatures4_2D_SUPER_TILE_VERSION: u32 = 0x20000000;
pub const chipMinorFeatures4_BUG_FIXES19: u32 = 0x40000000;
pub const chipMinorFeatures4_SH_ENHANCEMENTS6: u32 = 0x80000000;
pub const chipMinorFeatures5_SH_ENHANCEMENTS7: u32 = 0x00000001;
pub const chipMinorFeatures5_BUG_FIXES20: u32 = 0x00000002;
pub const chipMinorFeatures5_DE_ADDRESS_40: u32 = 0x00000004;
pub const chipMinorFeatures5_MINI_MMU_FIX: u32 = 0x00000008;
pub const chipMinorFeatures5_EEZ: u32 = 0x00000010;
pub const chipMinorFeatures5_BUG_FIXES21: u32 = 0x00000020;
pub const chipMinorFeatures5_EXTRA_VG_CAPS: u32 = 0x00000040;
pub const chipMinorFeatures5_MULTI_SRC_V15: u32 = 0x00000080;
pub const chipMinorFeatures5_BUG_FIXES22: u32 = 0x00000100;
pub const chipMinorFeatures5_HALTI3: u32 = 0x00000200;
pub const chipMinorFeatures5_TESSELATION_SHADERS: u32 = 0x00000400;
pub const chipMinorFeatures5_2D_ONE_PASS_FILTER_TAP: u32 = 0x00000800;
pub const chipMinorFeatures5_MULTI_SRC_V2_STR_QUAD: u32 = 0x00001000;
pub const chipMinorFeatures5_SEPARATE_SRC_DST: u32 = 0x00002000;
pub const chipMinorFeatures5_HALTI4: u32 = 0x00004000;
pub const chipMinorFeatures5_RA_WRITE_DEPTH: u32 = 0x00008000;
pub const chipMinorFeatures5_ANDROID_ONLY: u32 = 0x00010000;
pub const chipMinorFeatures5_HAS_PRODUCTID: u32 = 0x00020000;
pub const chipMinorFeatures5_TX_SUPPORT_DEC: u32 = 0x00040000;
pub const chipMinorFeatures5_S8_MSAA_COMPRESSION: u32 = 0x00080000;
pub const chipMinorFeatures5_PE_DITHER_FIX2: u32 = 0x00100000;
pub const chipMinorFeatures5_L2_CACHE_REMOVE: u32 = 0x00200000;
pub const chipMinorFeatures5_FE_ALLOW_RND_VTX_CNT: u32 = 0x00400000;
pub const chipMinorFeatures5_CUBE_MAP_FL28: u32 = 0x00800000;
pub const chipMinorFeatures5_TX_6BIT_FRAC: u32 = 0x01000000;
pub const chipMinorFeatures5_FE_ALLOW_STALL_PREFETCH_ENG: u32 = 0x02000000;
pub const chipMinorFeatures5_THIRD_PARTY_COMPRESSION: u32 = 0x04000000;
pub const chipMinorFeatures5_RS_DEPTHSTENCIL_NATIVE_SUPPORT: u32 = 0x08000000;
pub const chipMinorFeatures5_V2_MSAA_COMP_FIX: u32 = 0x10000000;
pub const chipMinorFeatures5_HALTI5: u32 = 0x20000000;
pub const chipMinorFeatures5_EVIS: u32 = 0x40000000;
pub const chipMinorFeatures5_BLT_ENGINE: u32 = 0x80000000;
pub const chipMinorFeatures6_BUG_FIXES_23: u32 = 0x00000001;
pub const chipMinorFeatures6_BUG_FIXES_24: u32 = 0x00000002;
pub const chipMinorFeatures6_DEC: u32 = 0x00000004;
pub const chipMinorFeatures6_VS_TILE_NV12: u32 = 0x00000008;
pub const chipMinorFeatures6_VS_TILE_NV12_10BIT: u32 = 0x00000010;
pub const chipMinorFeatures6_RENDER_TARGET_8: u32 = 0x00000020;
pub const chipMinorFeatures6_TEX_LOD_FLOW_CORR: u32 = 0x00000040;
pub const chipMinorFeatures6_FACE_LOD: u32 = 0x00000080;
pub const chipMinorFeatures6_MULTI_CORE_SEMAPHORE_STALL_V2: u32 = 0x00000100;
pub const chipMinorFeatures6_VMSAA: u32 = 0x00000200;
pub const chipMinorFeatures6_CHIP_ENABLE_LINK: u32 = 0x00000400;
pub const chipMinorFeatures6_MULTI_SRC_BLT_1_5_ENHANCEMENT: u32 = 0x00000800;
pub const chipMinorFeatures6_MULTI_SRC_BLT_BILINEAR_FILTER: u32 = 0x00001000;
pub const chipMinorFeatures6_RA_HZEZ_CLOCK_CONTROL: u32 = 0x00002000;
pub const chipMinorFeatures6_CACHE128B256BPERLINE: u32 = 0x00004000;
pub const chipMinorFeatures6_V4_COMPRESSION: u32 = 0x00008000;
pub const chipMinorFeatures6_PE2D_MAJOR_SUPER_TILE: u32 = 0x00010000;
pub const chipMinorFeatures6_PE_32BPC_COLORMASK_FIX: u32 = 0x00020000;
pub const chipMinorFeatures6_ALPHA_BLENDING_OPT: u32 = 0x00040000;
pub const chipMinorFeatures6_NEW_GPIPE: u32 = 0x00080000;
pub const chipMinorFeatures6_PIPELINE_32_ATTRIBUTES: u32 = 0x00100000;
pub const chipMinorFeatures6_MSAA_SHADING: u32 = 0x00200000;
pub const chipMinorFeatures6_NO_ANISTRO_FILTER: u32 = 0x00400000;
pub const chipMinorFeatures6_NO_ASTC: u32 = 0x00800000;
pub const chipMinorFeatures6_NO_DXT: u32 = 0x01000000;
pub const chipMinorFeatures6_HWTFB: u32 = 0x02000000;
pub const chipMinorFeatures6_RA_DEPTH_WRITE_MSAA1X_FIX: u32 = 0x04000000;
pub const chipMinorFeatures6_EZHZ_CLOCKGATE_FIX: u32 = 0x08000000;
pub const chipMinorFeatures6_SH_SNAP2PAGE_FIX: u32 = 0x10000000;
pub const chipMinorFeatures6_SH_HALFDEPENDENCY_FIX: u32 = 0x20000000;
pub const chipMinorFeatures6_USC_MCFILL_FIX: u32 = 0x40000000;
pub const chipMinorFeatures6_TPG_TCPERF_FIX: u32 = 0x80000000;
pub const chipMinorFeatures7_USC_MDFIFO_OVERFLOW_FIX: u32 = 0x00000001;
pub const chipMinorFeatures7_SH_TEXLD_BARRIER_IN_CS_FIX: u32 = 0x00000002;
pub const chipMinorFeatures7_RS_NEW_BASEADDR: u32 = 0x00000004;
pub const chipMinorFeatures7_PE_8BPP_DUALPIPE_FIX: u32 = 0x00000008;
pub const chipMinorFeatures7_SH_ADVANCED_INSTR: u32 = 0x00000010;
pub const chipMinorFeatures7_SH_FLAT_INTERPOLATION_DUAL16_FIX: u32 = 0x00000020;
pub const chipMinorFeatures7_USC_CONTINUOUS_FLUS_FIX: u32 = 0x00000040;
pub const chipMinorFeatures7_SH_SUPPORT_V4: u32 = 0x00000080;
pub const chipMinorFeatures7_SH_SUPPORT_ALPHA_KILL: u32 = 0x00000100;
pub const chipMinorFeatures7_PE_NO_ALPHA_TEST: u32 = 0x00000200;
pub const chipMinorFeatures7_TX_LOD_NEAREST_SELECT: u32 = 0x00000400;
pub const chipMinorFeatures7_SH_FIX_LDEXP: u32 = 0x00000800;
pub const chipMinorFeatures7_SUPPORT_MOVAI: u32 = 0x00001000;
pub const chipMinorFeatures7_SH_SNAP2PAGE_MAXPAGES_FIX: u32 = 0x00002000;
pub const chipMinorFeatures7_PE_RGBA16I_FIX: u32 = 0x00004000;
pub const chipMinorFeatures7_BLT_8bpp_256TILE_FC_FIX: u32 = 0x00008000;
pub const chipMinorFeatures7_PE_64BIT_FENCE_FIX: u32 = 0x00010000;
pub const chipMinorFeatures7_USC_FULL_CACHE_FIX: u32 = 0x00020000;
pub const chipMinorFeatures7_TX_YUV_ASSEMBLER_10BIT: u32 = 0x00040000;
pub const chipMinorFeatures7_FE_32BIT_INDEX_FIX: u32 = 0x00080000;
pub const chipMinorFeatures7_BLT_64BPP_MASKED_CLEAR_FIX: u32 = 0x00100000;
pub const chipMinorFeatures7_BIT_SECURITY: u32 = 0x00200000;
pub const chipMinorFeatures7_BIT_ROBUSTNESS: u32 = 0x00400000;
pub const chipMinorFeatures7_USC_ATOMIC_FIX: u32 = 0x00800000;
pub const chipMinorFeatures7_SH_PSO_MSAA1x_FIX: u32 = 0x01000000;
pub const chipMinorFeatures7_BIT_USC_VX_PERF_FIX: u32 = 0x02000000;
pub const chipMinorFeatures7_EVIS_NO_ABSDIFF: u32 = 0x04000000;
pub const chipMinorFeatures7_EVIS_NO_BITREPLACE: u32 = 0x08000000;
pub const chipMinorFeatures7_EVIS_NO_BOXFILTER: u32 = 0x10000000;
pub const chipMinorFeatures7_EVIS_NO_CORDIAC: u32 = 0x20000000;
pub const chipMinorFeatures7_EVIS_NO_DP32: u32 = 0x40000000;
pub const chipMinorFeatures7_EVIS_NO_FILTER: u32 = 0x80000000;
pub const chipMinorFeatures8_EVIS_NO_IADD: u32 = 0x00000001;
pub const chipMinorFeatures8_EVIS_NO_SELECTADD: u32 = 0x00000002;
pub const chipMinorFeatures8_EVIS_LERP_7OUTPUT: u32 = 0x00000004;
pub const chipMinorFeatures8_EVIS_ACCSQ_8OUTPUT: u32 = 0x00000008;
pub const chipMinorFeatures8_USC_GOS_ADDR_FIX: u32 = 0x00000010;
pub const chipMinorFeatures8_TX_8BIT_UVFRAC: u32 = 0x00000020;
pub const chipMinorFeatures8_TX_DESC_CACHE_CLOCKGATE_FIX: u32 = 0x00000040;
pub const chipMinorFeatures8_RSBLT_MSAA_DECOMPRESSION: u32 = 0x00000080;
pub const chipMinorFeatures8_TX_INTEGER_COORDINATE: u32 = 0x00000100;
pub const chipMinorFeatures8_DRAWID: u32 = 0x00000200;
pub const chipMinorFeatures8_PSIO_SAMPLEMASK_IN_R0ZW_FIX: u32 = 0x00000400;
pub const chipMinorFeatures8_TX_INTEGER_COORDINATE_V2: u32 = 0x00000800;
pub const chipMinorFeatures8_MULTI_CORE_BLOCK_SET_CONFIG: u32 = 0x00001000;
pub const chipMinorFeatures8_VG_RESOLVE_ENGINE: u32 = 0x00002000;
pub const chipMinorFeatures8_VG_PE_COLOR_KEY: u32 = 0x00004000;
pub const chipMinorFeatures8_VG_IM_INDEX_FORMAT: u32 = 0x00008000;
pub const chipMinorFeatures8_SNAPPAGE_CMD: u32 = 0x00010000;
pub const chipMinorFeatures8_SH_NO_INDEX_CONST_ON_A0: u32 = 0x00020000;
pub const chipMinorFeatures8_SH_NO_ONECONST_LIMIT: u32 = 0x00040000;
pub const chipMinorFeatures8_SH_IMG_LDST_ON_TEMP: u32 = 0x00080000;
pub const chipMinorFeatures8_COMPUTE_ONLY: u32 = 0x00100000;
pub const chipMinorFeatures8_SH_IMG_LDST_CLAMP: u32 = 0x00200000;
pub const chipMinorFeatures8_SH_ICACHE_ALLOC_COUNT_FIX: u32 = 0x00400000;
pub const chipMinorFeatures8_SH_ICACHE_PREFETCH: u32 = 0x00800000;
pub const chipMinorFeatures8_PE2D_SEPARATE_CACHE: u32 = 0x01000000;
pub const chipMinorFeatures8_VG_AYUV_INPUT_OUTPUT: u32 = 0x02000000;
pub const chipMinorFeatures8_VG_DOUBLE_IMAGE: u32 = 0x04000000;
pub const chipMinorFeatures8_VG_RECTANGLE_STRIPE_MODE: u32 = 0x08000000;
pub const chipMinorFeatures8_VG_MMU: u32 = 0x10000000;
pub const chipMinorFeatures8_VG_IM_FILTER: u32 = 0x20000000;
pub const chipMinorFeatures8_VG_IM_YUV_PACKET: u32 = 0x40000000;
pub const chipMinorFeatures8_VG_IM_YUV_PLANAR: u32 = 0x80000000;
pub const chipMinorFeatures9_VG_PE_YUV_PACKET: u32 = 0x00000001;
pub const chipMinorFeatures9_VG_COLOR_PRECISION_8_BIT: u32 = 0x00000002;
pub const chipMinorFeatures9_PE_MSAA_OQ_FIX: u32 = 0x00000004;
pub const chipMinorFeatures9_PSIO_MSAA_CL_FIX: u32 = 0x00000008;
pub const chipMinorFeatures9_USC_DEFER_FILL_FIX: u32 = 0x00000010;
pub const chipMinorFeatures9_SH_CLOCK_GATE_FIX: u32 = 0x00000020;
pub const chipMinorFeatures9_FE_NEED_DUMMYDRAW: u32 = 0x00000040;
pub const chipMinorFeatures9_PE2D_LINEAR_YUV420_OUTPUT: u32 = 0x00000080;
pub const chipMinorFeatures9_PE2D_LINEAR_YUV420_10BIT: u32 = 0x00000100;
pub const chipMinorFeatures9_MULTI_CLUSTER: u32 = 0x00000200;
pub const chipMinorFeatures9_VG_TS_CULLING: u32 = 0x00000400;
pub const chipMinorFeatures9_VG_FP25: u32 = 0x00000800;
pub const chipMinorFeatures9_SH_MULTI_WG_PACK: u32 = 0x00001000;
pub const chipMinorFeatures9_SH_DUAL16_SAMPLEMASK_ZW: u32 = 0x00002000;
pub const chipMinorFeatures9_TPG_TRIVIAL_MODE_FIX: u32 = 0x00004000;
pub const chipMinorFeatures9_TX_ASTC_MULTISLICE_FIX: u32 = 0x00008000;
pub const chipMinorFeatures9_FE_ROBUST_FIX: u32 = 0x00010000;
pub const chipMinorFeatures9_SH_GPIPE_ACCESS_FULLTEMPS: u32 = 0x00020000;
pub const chipMinorFeatures9_PSIO_INTERLOCK: u32 = 0x00040000;
pub const chipMinorFeatures9_PA_WIDELINE_FIX: u32 = 0x00080000;
pub const chipMinorFeatures9_WIDELINE_HELPER_FIX: u32 = 0x00100000;
pub const chipMinorFeatures9_G2D_3RD_PARTY_COMPRESSION_1_1: u32 = 0x00200000;
pub const chipMinorFeatures9_TX_FLUSH_L1CACHE: u32 = 0x00400000;
pub const chipMinorFeatures9_PE_DITHER_FIX2: u32 = 0x00800000;
pub const chipMinorFeatures9_G2D_DEC400: u32 = 0x01000000;
pub const chipMinorFeatures9_SH_TEXLD_U_FIX: u32 = 0x02000000;
pub const chipMinorFeatures9_MC_FCCACHE_BYTEMASK: u32 = 0x04000000;
pub const chipMinorFeatures9_SH_MULTI_WG_PACK_FIX: u32 = 0x08000000;
pub const chipMinorFeatures9_DC_OVERLAY_SCALING: u32 = 0x10000000;
pub const chipMinorFeatures9_DC_SOURCE_ROTATION: u32 = 0x20000000;
pub const chipMinorFeatures9_DC_TILED: u32 = 0x40000000;
pub const chipMinorFeatures9_DC_YUV_L1: u32 = 0x80000000;
pub const chipMinorFeatures10_DC_D30_OUTPUT: u32 = 0x00000001;
pub const chipMinorFeatures10_DC_MMU: u32 = 0x00000002;
pub const chipMinorFeatures10_DC_COMPRESSION: u32 = 0x00000004;
pub const chipMinorFeatures10_DC_QOS: u32 = 0x00000008;
pub const chipMinorFeatures10_PE_ADVANCE_BLEND_PART0: u32 = 0x00000010;
pub const chipMinorFeatures10_FE_PATCHLIST_FETCH_FIX: u32 = 0x00000020;
pub const chipMinorFeatures10_RA_CG_FIX: u32 = 0x00000040;
pub const chipMinorFeatures10_EVIS_VX2: u32 = 0x00000080;
pub const chipMinorFeatures10_NN_FLOAT: u32 = 0x00000100;
pub const chipMinorFeatures10_DEC400: u32 = 0x00000200;
pub const chipMinorFeatures10_LS_SUPPORT_PERCOMP_DEPENDENCY: u32 = 0x00000400;
pub const chipMinorFeatures10_TP_ENGINE: u32 = 0x00000800;
pub const chipMinorFeatures10_MULTI_CORE_BLOCK_SET_CONFIG2: u32 = 0x00001000;
pub const chipMinorFeatures10_PE_VMSAA_COVERAGE_CACHE_FIX: u32 = 0x00002000;
pub const chipMinorFeatures10_SECURITY_AHB: u32 = 0x00004000;
pub const chipMinorFeatures10_MULTICORE_SEMAPHORESTALL_V3: u32 = 0x00008000;
pub const chipMinorFeatures10_SMALLBATCH: u32 = 0x00010000;
pub const chipMinorFeatures10_SH_CMPLX: u32 = 0x00020000;
pub const chipMinorFeatures10_SH_IDIV0_SWZL_EHS: u32 = 0x00040000;
pub const chipMinorFeatures10_TX_LERP_LESS_BIT: u32 = 0x00080000;
pub const chipMinorFeatures10_SH_GM_ENDIAN: u32 = 0x00100000;
pub const chipMinorFeatures10_SH_GM_USC_UNALLOC: u32 = 0x00200000;
pub const chipMinorFeatures10_SH_END_OF_BB: u32 = 0x00400000;
pub const chipMinorFeatures10_VIP_V7: u32 = 0x00800000;
pub const chipMinorFeatures10_TX_BORDER_CLAMP_FIX: u32 = 0x01000000;
pub const chipMinorFeatures10_SH_IMG_LD_LASTPIXEL_FIX: u32 = 0x02000000;
pub const chipMinorFeatures10_ASYNC_BLT: u32 = 0x04000000;
pub const chipMinorFeatures10_ASYNC_FE_FENCE_FIX: u32 = 0x08000000;
pub const chipMinorFeatures10_PSCS_THROTTLE: u32 = 0x10000000;
pub const chipMinorFeatures10_SEPARATE_LS: u32 = 0x20000000;
pub const chipMinorFeatures10_MCFE: u32 = 0x40000000;
pub const chipMinorFeatures10_WIDELINE_TRIANGLE_EMU: u32 = 0x80000000;
pub const chipMinorFeatures11_VG_RESOLUTION_8K: u32 = 0x00000001;
pub const chipMinorFeatures11_FENCE_32BIT: u32 = 0x00000002;
pub const chipMinorFeatures11_FENCE_64BIT: u32 = 0x00000004;
pub const chipMinorFeatures11_NN_INTERLEVE8: u32 = 0x00000008;
pub const chipMinorFeatures11_TP_REORDER: u32 = 0x00000010;
pub const chipMinorFeatures11_PE_DEPTH_ONLY_OQFIX: u32 = 0x00000020;
pub const FE_OPCODE_LOAD_STATE: u32 = 0x00000001;
pub const FE_OPCODE_END: u32 = 0x00000002;
pub const FE_OPCODE_NOP: u32 = 0x00000003;
pub const FE_OPCODE_DRAW_2D: u32 = 0x00000004;
pub const FE_OPCODE_DRAW_PRIMITIVES: u32 = 0x00000005;
pub const FE_OPCODE_DRAW_INDEXED_PRIMITIVES: u32 = 0x00000006;
pub const FE_OPCODE_WAIT: u32 = 0x00000007;
pub const FE_OPCODE_LINK: u32 = 0x00000008;
pub const FE_OPCODE_STALL: u32 = 0x00000009;
pub const FE_OPCODE_CALL: u32 = 0x0000000a;
pub const FE_OPCODE_RETURN: u32 = 0x0000000b;
pub const FE_OPCODE_DRAW_INSTANCED: u32 = 0x0000000c;
pub const FE_OPCODE_CHIP_SELECT: u32 = 0x0000000d;
pub const FE_OPCODE_WAIT_FENCE: u32 = 0x0000000f;
pub const FE_OPCODE_DRAW_INDIRECT: u32 = 0x00000010;
pub const FE_OPCODE_SNAP_PAGES: u32 = 0x00000013;
pub const PRIMITIVE_TYPE_POINTS: u32 = 0x00000001;
pub const PRIMITIVE_TYPE_LINES: u32 = 0x00000002;
pub const PRIMITIVE_TYPE_LINE_STRIP: u32 = 0x00000003;
pub const PRIMITIVE_TYPE_TRIANGLES: u32 = 0x00000004;
pub const PRIMITIVE_TYPE_TRIANGLE_STRIP: u32 = 0x00000005;
pub const PRIMITIVE_TYPE_TRIANGLE_FAN: u32 = 0x00000006;
pub const PRIMITIVE_TYPE_LINE_LOOP: u32 = 0x00000007;
pub const PRIMITIVE_TYPE_QUADS: u32 = 0x00000008;
pub const VIV_FE_LOAD_STATE: u32 = 0x00000000;
pub const VIV_FE_LOAD_STATE_HEADER: u32 = 0x00000000;
pub const VIV_FE_LOAD_STATE_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_LOAD_STATE_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_LOAD_STATE_HEADER_OP_LOAD_STATE: u32 = 0x08000000;
pub const VIV_FE_LOAD_STATE_HEADER_FIXP: u32 = 0x04000000;
pub const VIV_FE_LOAD_STATE_HEADER_COUNT__MASK: u32 = 0x03ff0000;
pub const VIV_FE_LOAD_STATE_HEADER_COUNT__SHIFT: u32 = 0x00000010;
pub const fn VIV_FE_LOAD_STATE_HEADER_COUNT(x: u32) -> u32 { (x << VIV_FE_LOAD_STATE_HEADER_COUNT__SHIFT) & VIV_FE_LOAD_STATE_HEADER_COUNT__MASK }
pub const VIV_FE_LOAD_STATE_HEADER_OFFSET__MASK: u32 = 0x0000ffff;
pub const VIV_FE_LOAD_STATE_HEADER_OFFSET__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_LOAD_STATE_HEADER_OFFSET(x: u32) -> u32 { (x << VIV_FE_LOAD_STATE_HEADER_OFFSET__SHIFT) & VIV_FE_LOAD_STATE_HEADER_OFFSET__MASK }
pub const VIV_FE_LOAD_STATE_HEADER_OFFSET__SHR: u32 = 0x00000002;
pub const VIV_FE_END: u32 = 0x00000000;
pub const VIV_FE_END_HEADER: u32 = 0x00000000;
pub const VIV_FE_END_HEADER_EVENT_ID__MASK: u32 = 0x0000001f;
pub const VIV_FE_END_HEADER_EVENT_ID__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_END_HEADER_EVENT_ID(x: u32) -> u32 { (x << VIV_FE_END_HEADER_EVENT_ID__SHIFT) & VIV_FE_END_HEADER_EVENT_ID__MASK }
pub const VIV_FE_END_HEADER_EVENT_ENABLE: u32 = 0x00000100;
pub const VIV_FE_END_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_END_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_END_HEADER_OP_END: u32 = 0x10000000;
pub const VIV_FE_NOP: u32 = 0x00000000;
pub const VIV_FE_NOP_HEADER: u32 = 0x00000000;
pub const VIV_FE_NOP_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_NOP_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_NOP_HEADER_OP_NOP: u32 = 0x18000000;
pub const VIV_FE_DRAW_2D: u32 = 0x00000000;
pub const VIV_FE_DRAW_2D_HEADER: u32 = 0x00000000;
pub const VIV_FE_DRAW_2D_HEADER_COUNT__MASK: u32 = 0x0000ff00;
pub const VIV_FE_DRAW_2D_HEADER_COUNT__SHIFT: u32 = 0x00000008;
pub const fn VIV_FE_DRAW_2D_HEADER_COUNT(x: u32) -> u32 { (x << VIV_FE_DRAW_2D_HEADER_COUNT__SHIFT) & VIV_FE_DRAW_2D_HEADER_COUNT__MASK }
pub const VIV_FE_DRAW_2D_HEADER_DATA_COUNT__MASK: u32 = 0x07ff0000;
pub const VIV_FE_DRAW_2D_HEADER_DATA_COUNT__SHIFT: u32 = 0x00000010;
pub const fn VIV_FE_DRAW_2D_HEADER_DATA_COUNT(x: u32) -> u32 { (x << VIV_FE_DRAW_2D_HEADER_DATA_COUNT__SHIFT) & VIV_FE_DRAW_2D_HEADER_DATA_COUNT__MASK }
pub const VIV_FE_DRAW_2D_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_DRAW_2D_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_DRAW_2D_HEADER_OP_DRAW_2D: u32 = 0x20000000;
pub const VIV_FE_DRAW_2D_TOP_LEFT: u32 = 0x00000008;
pub const VIV_FE_DRAW_2D_TOP_LEFT_X__MASK: u32 = 0x0000ffff;
pub const VIV_FE_DRAW_2D_TOP_LEFT_X__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_DRAW_2D_TOP_LEFT_X(x: u32) -> u32 { (x << VIV_FE_DRAW_2D_TOP_LEFT_X__SHIFT) & VIV_FE_DRAW_2D_TOP_LEFT_X__MASK }
pub const VIV_FE_DRAW_2D_TOP_LEFT_Y__MASK: u32 = 0xffff0000;
pub const VIV_FE_DRAW_2D_TOP_LEFT_Y__SHIFT: u32 = 0x00000010;
pub const fn VIV_FE_DRAW_2D_TOP_LEFT_Y(x: u32) -> u32 { (x << VIV_FE_DRAW_2D_TOP_LEFT_Y__SHIFT) & VIV_FE_DRAW_2D_TOP_LEFT_Y__MASK }
pub const VIV_FE_DRAW_2D_BOTTOM_RIGHT: u32 = 0x0000000c;
pub const VIV_FE_DRAW_2D_BOTTOM_RIGHT_X__MASK: u32 = 0x0000ffff;
pub const VIV_FE_DRAW_2D_BOTTOM_RIGHT_X__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_DRAW_2D_BOTTOM_RIGHT_X(x: u32) -> u32 { (x << VIV_FE_DRAW_2D_BOTTOM_RIGHT_X__SHIFT) & VIV_FE_DRAW_2D_BOTTOM_RIGHT_X__MASK }
pub const VIV_FE_DRAW_2D_BOTTOM_RIGHT_Y__MASK: u32 = 0xffff0000;
pub const VIV_FE_DRAW_2D_BOTTOM_RIGHT_Y__SHIFT: u32 = 0x00000010;
pub const fn VIV_FE_DRAW_2D_BOTTOM_RIGHT_Y(x: u32) -> u32 { (x << VIV_FE_DRAW_2D_BOTTOM_RIGHT_Y__SHIFT) & VIV_FE_DRAW_2D_BOTTOM_RIGHT_Y__MASK }
pub const VIV_FE_DRAW_PRIMITIVES: u32 = 0x00000000;
pub const VIV_FE_DRAW_PRIMITIVES_HEADER: u32 = 0x00000000;
pub const VIV_FE_DRAW_PRIMITIVES_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_DRAW_PRIMITIVES_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_DRAW_PRIMITIVES_HEADER_OP_DRAW_PRIMITIVES: u32 = 0x28000000;
pub const VIV_FE_DRAW_PRIMITIVES_COMMAND: u32 = 0x00000004;
pub const VIV_FE_DRAW_PRIMITIVES_COMMAND_TYPE__MASK: u32 = 0x000000ff;
pub const VIV_FE_DRAW_PRIMITIVES_COMMAND_TYPE__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_DRAW_PRIMITIVES_COMMAND_TYPE(x: u32) -> u32 { (x << VIV_FE_DRAW_PRIMITIVES_COMMAND_TYPE__SHIFT) & VIV_FE_DRAW_PRIMITIVES_COMMAND_TYPE__MASK }
pub const VIV_FE_DRAW_PRIMITIVES_START: u32 = 0x00000008;
pub const VIV_FE_DRAW_PRIMITIVES_COUNT: u32 = 0x0000000c;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES: u32 = 0x00000000;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_HEADER: u32 = 0x00000000;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_HEADER_OP_DRAW_INDEXED_PRIMITIVES: u32 = 0x30000000;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_COMMAND: u32 = 0x00000004;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_COMMAND_TYPE__MASK: u32 = 0x000000ff;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_COMMAND_TYPE__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_DRAW_INDEXED_PRIMITIVES_COMMAND_TYPE(x: u32) -> u32 { (x << VIV_FE_DRAW_INDEXED_PRIMITIVES_COMMAND_TYPE__SHIFT) & VIV_FE_DRAW_INDEXED_PRIMITIVES_COMMAND_TYPE__MASK }
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_START: u32 = 0x00000008;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_COUNT: u32 = 0x0000000c;
pub const VIV_FE_DRAW_INDEXED_PRIMITIVES_OFFSET: u32 = 0x00000010;
pub const VIV_FE_WAIT: u32 = 0x00000000;
pub const VIV_FE_WAIT_HEADER: u32 = 0x00000000;
pub const VIV_FE_WAIT_HEADER_DELAY__MASK: u32 = 0x0000ffff;
pub const VIV_FE_WAIT_HEADER_DELAY__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_WAIT_HEADER_DELAY(x: u32) -> u32 { (x << VIV_FE_WAIT_HEADER_DELAY__SHIFT) & VIV_FE_WAIT_HEADER_DELAY__MASK }
pub const VIV_FE_WAIT_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_WAIT_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_WAIT_HEADER_OP_WAIT: u32 = 0x38000000;
pub const VIV_FE_LINK: u32 = 0x00000000;
pub const VIV_FE_LINK_HEADER: u32 = 0x00000000;
pub const VIV_FE_LINK_HEADER_PREFETCH__MASK: u32 = 0x0000ffff;
pub const VIV_FE_LINK_HEADER_PREFETCH__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_LINK_HEADER_PREFETCH(x: u32) -> u32 { (x << VIV_FE_LINK_HEADER_PREFETCH__SHIFT) & VIV_FE_LINK_HEADER_PREFETCH__MASK }
pub const VIV_FE_LINK_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_LINK_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_LINK_HEADER_OP_LINK: u32 = 0x40000000;
pub const VIV_FE_LINK_ADDRESS: u32 = 0x00000004;
pub const VIV_FE_STALL: u32 = 0x00000000;
pub const VIV_FE_STALL_HEADER: u32 = 0x00000000;
pub const VIV_FE_STALL_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_STALL_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_STALL_HEADER_OP_STALL: u32 = 0x48000000;
pub const VIV_FE_STALL_TOKEN: u32 = 0x00000004;
pub const VIV_FE_STALL_TOKEN_FROM__MASK: u32 = 0x0000001f;
pub const VIV_FE_STALL_TOKEN_FROM__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_STALL_TOKEN_FROM(x: u32) -> u32 { (x << VIV_FE_STALL_TOKEN_FROM__SHIFT) & VIV_FE_STALL_TOKEN_FROM__MASK }
pub const VIV_FE_STALL_TOKEN_TO__MASK: u32 = 0x00001f00;
pub const VIV_FE_STALL_TOKEN_TO__SHIFT: u32 = 0x00000008;
pub const fn VIV_FE_STALL_TOKEN_TO(x: u32) -> u32 { (x << VIV_FE_STALL_TOKEN_TO__SHIFT) & VIV_FE_STALL_TOKEN_TO__MASK }
pub const VIV_FE_STALL_TOKEN_UNK28__MASK: u32 = 0x30000000;
pub const VIV_FE_STALL_TOKEN_UNK28__SHIFT: u32 = 0x0000001c;
pub const fn VIV_FE_STALL_TOKEN_UNK28(x: u32) -> u32 { (x << VIV_FE_STALL_TOKEN_UNK28__SHIFT) & VIV_FE_STALL_TOKEN_UNK28__MASK }
pub const VIV_FE_CALL: u32 = 0x00000000;
pub const VIV_FE_CALL_HEADER: u32 = 0x00000000;
pub const VIV_FE_CALL_HEADER_PREFETCH__MASK: u32 = 0x0000ffff;
pub const VIV_FE_CALL_HEADER_PREFETCH__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_CALL_HEADER_PREFETCH(x: u32) -> u32 { (x << VIV_FE_CALL_HEADER_PREFETCH__SHIFT) & VIV_FE_CALL_HEADER_PREFETCH__MASK }
pub const VIV_FE_CALL_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_CALL_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_CALL_HEADER_OP_CALL: u32 = 0x50000000;
pub const VIV_FE_CALL_ADDRESS: u32 = 0x00000004;
pub const VIV_FE_CALL_RETURN_PREFETCH: u32 = 0x00000008;
pub const VIV_FE_CALL_RETURN_ADDRESS: u32 = 0x0000000c;
pub const VIV_FE_RETURN: u32 = 0x00000000;
pub const VIV_FE_RETURN_HEADER: u32 = 0x00000000;
pub const VIV_FE_RETURN_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_RETURN_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_RETURN_HEADER_OP_RETURN: u32 = 0x58000000;
pub const VIV_FE_CHIP_SELECT: u32 = 0x00000000;
pub const VIV_FE_CHIP_SELECT_HEADER: u32 = 0x00000000;
pub const VIV_FE_CHIP_SELECT_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_CHIP_SELECT_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_CHIP_SELECT_HEADER_OP_CHIP_SELECT: u32 = 0x68000000;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP15: u32 = 0x00008000;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP14: u32 = 0x00004000;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP13: u32 = 0x00002000;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP12: u32 = 0x00001000;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP11: u32 = 0x00000800;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP10: u32 = 0x00000400;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP9: u32 = 0x00000200;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP8: u32 = 0x00000100;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP7: u32 = 0x00000080;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP6: u32 = 0x00000040;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP5: u32 = 0x00000020;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP4: u32 = 0x00000010;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP3: u32 = 0x00000008;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP2: u32 = 0x00000004;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP1: u32 = 0x00000002;
pub const VIV_FE_CHIP_SELECT_HEADER_ENABLE_CHIP0: u32 = 0x00000001;
pub const VIV_FE_DRAW_INSTANCED: u32 = 0x00000000;
pub const VIV_FE_DRAW_INSTANCED_HEADER: u32 = 0x00000000;
pub const VIV_FE_DRAW_INSTANCED_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_DRAW_INSTANCED_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_DRAW_INSTANCED_HEADER_OP_DRAW_INSTANCED: u32 = 0x60000000;
pub const VIV_FE_DRAW_INSTANCED_HEADER_INDEXED: u32 = 0x00100000;
pub const VIV_FE_DRAW_INSTANCED_HEADER_TYPE__MASK: u32 = 0x000f0000;
pub const VIV_FE_DRAW_INSTANCED_HEADER_TYPE__SHIFT: u32 = 0x00000010;
pub const fn VIV_FE_DRAW_INSTANCED_HEADER_TYPE(x: u32) -> u32 { (x << VIV_FE_DRAW_INSTANCED_HEADER_TYPE__SHIFT) & VIV_FE_DRAW_INSTANCED_HEADER_TYPE__MASK }
pub const VIV_FE_DRAW_INSTANCED_HEADER_INSTANCE_COUNT_LO__MASK: u32 = 0x0000ffff;
pub const VIV_FE_DRAW_INSTANCED_HEADER_INSTANCE_COUNT_LO__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_DRAW_INSTANCED_HEADER_INSTANCE_COUNT_LO(x: u32) -> u32 { (x << VIV_FE_DRAW_INSTANCED_HEADER_INSTANCE_COUNT_LO__SHIFT) & VIV_FE_DRAW_INSTANCED_HEADER_INSTANCE_COUNT_LO__MASK }
pub const VIV_FE_DRAW_INSTANCED_COUNT: u32 = 0x00000004;
pub const VIV_FE_DRAW_INSTANCED_COUNT_INSTANCE_COUNT_HI__MASK: u32 = 0xff000000;
pub const VIV_FE_DRAW_INSTANCED_COUNT_INSTANCE_COUNT_HI__SHIFT: u32 = 0x00000018;
pub const fn VIV_FE_DRAW_INSTANCED_COUNT_INSTANCE_COUNT_HI(x: u32) -> u32 { (x << VIV_FE_DRAW_INSTANCED_COUNT_INSTANCE_COUNT_HI__SHIFT) & VIV_FE_DRAW_INSTANCED_COUNT_INSTANCE_COUNT_HI__MASK }
pub const VIV_FE_DRAW_INSTANCED_COUNT_VERTEX_COUNT__MASK: u32 = 0x00ffffff;
pub const VIV_FE_DRAW_INSTANCED_COUNT_VERTEX_COUNT__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_DRAW_INSTANCED_COUNT_VERTEX_COUNT(x: u32) -> u32 { (x << VIV_FE_DRAW_INSTANCED_COUNT_VERTEX_COUNT__SHIFT) & VIV_FE_DRAW_INSTANCED_COUNT_VERTEX_COUNT__MASK }
pub const VIV_FE_DRAW_INSTANCED_START: u32 = 0x00000008;
pub const VIV_FE_DRAW_INSTANCED_START_INDEX__MASK: u32 = 0xffffffff;
pub const VIV_FE_DRAW_INSTANCED_START_INDEX__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_DRAW_INSTANCED_START_INDEX(x: u32) -> u32 { (x << VIV_FE_DRAW_INSTANCED_START_INDEX__SHIFT) & VIV_FE_DRAW_INSTANCED_START_INDEX__MASK }
pub const VIV_FE_WAIT_FENCE: u32 = 0x00000000;
pub const VIV_FE_WAIT_FENCE_HEADER: u32 = 0x00000000;
pub const VIV_FE_WAIT_FENCE_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_WAIT_FENCE_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_WAIT_FENCE_HEADER_OP_WAIT_FENCE: u32 = 0x78000000;
pub const VIV_FE_WAIT_FENCE_HEADER_UNK16__MASK: u32 = 0x00030000;
pub const VIV_FE_WAIT_FENCE_HEADER_UNK16__SHIFT: u32 = 0x00000010;
pub const fn VIV_FE_WAIT_FENCE_HEADER_UNK16(x: u32) -> u32 { (x << VIV_FE_WAIT_FENCE_HEADER_UNK16__SHIFT) & VIV_FE_WAIT_FENCE_HEADER_UNK16__MASK }
pub const VIV_FE_WAIT_FENCE_HEADER_WAITCOUNT__MASK: u32 = 0x0000ffff;
pub const VIV_FE_WAIT_FENCE_HEADER_WAITCOUNT__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_WAIT_FENCE_HEADER_WAITCOUNT(x: u32) -> u32 { (x << VIV_FE_WAIT_FENCE_HEADER_WAITCOUNT__SHIFT) & VIV_FE_WAIT_FENCE_HEADER_WAITCOUNT__MASK }
pub const VIV_FE_WAIT_FENCE_ADDRESS: u32 = 0x00000004;
pub const VIV_FE_DRAW_INDIRECT: u32 = 0x00000000;
pub const VIV_FE_DRAW_INDIRECT_HEADER: u32 = 0x00000000;
pub const VIV_FE_DRAW_INDIRECT_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_DRAW_INDIRECT_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_DRAW_INDIRECT_HEADER_OP_DRAW_INDIRECT: u32 = 0x80000000;
pub const VIV_FE_DRAW_INDIRECT_HEADER_UNK8: u32 = 0x00000100;
pub const VIV_FE_DRAW_INDIRECT_HEADER_TYPE__MASK: u32 = 0x0000000f;
pub const VIV_FE_DRAW_INDIRECT_HEADER_TYPE__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_DRAW_INDIRECT_HEADER_TYPE(x: u32) -> u32 { (x << VIV_FE_DRAW_INDIRECT_HEADER_TYPE__SHIFT) & VIV_FE_DRAW_INDIRECT_HEADER_TYPE__MASK }
pub const VIV_FE_DRAW_INDIRECT_ADDRESS: u32 = 0x00000004;
pub const VIV_FE_SNAP_PAGES: u32 = 0x00000000;
pub const VIV_FE_SNAP_PAGES_HEADER: u32 = 0x00000000;
pub const VIV_FE_SNAP_PAGES_HEADER_OP__MASK: u32 = 0xf8000000;
pub const VIV_FE_SNAP_PAGES_HEADER_OP__SHIFT: u32 = 0x0000001b;
pub const VIV_FE_SNAP_PAGES_HEADER_OP_SNAP_PAGES: u32 = 0x98000000;
pub const VIV_FE_SNAP_PAGES_HEADER_UNK0__MASK: u32 = 0x0000001f;
pub const VIV_FE_SNAP_PAGES_HEADER_UNK0__SHIFT: u32 = 0x00000000;
pub const fn VIV_FE_SNAP_PAGES_HEADER_UNK0(x: u32) -> u32 { (x << VIV_FE_SNAP_PAGES_HEADER_UNK0__SHIFT) & VIV_FE_SNAP_PAGES_HEADER_UNK0__MASK }
pub const VARYING_COMPONENT_USE_UNUSED: u32 = 0x00000000;
pub const VARYING_COMPONENT_USE_USED: u32 = 0x00000001;
pub const VARYING_COMPONENT_USE_POINTCOORD_X: u32 = 0x00000002;
pub const VARYING_COMPONENT_USE_POINTCOORD_Y: u32 = 0x00000003;
pub const FE_DATA_TYPE_BYTE: u32 = 0x00000000;
pub const FE_DATA_TYPE_UNSIGNED_BYTE: u32 = 0x00000001;
pub const FE_DATA_TYPE_SHORT: u32 = 0x00000002;
pub const FE_DATA_TYPE_UNSIGNED_SHORT: u32 = 0x00000003;
pub const FE_DATA_TYPE_INT: u32 = 0x00000004;
pub const FE_DATA_TYPE_UNSIGNED_INT: u32 = 0x00000005;
pub const FE_DATA_TYPE_INT_2_10_10_10_REV: u32 = 0x00000006;
pub const FE_DATA_TYPE_UNSIGNED_INT_2_10_10_10_REV: u32 = 0x00000007;
pub const FE_DATA_TYPE_FLOAT: u32 = 0x00000008;
pub const FE_DATA_TYPE_HALF_FLOAT: u32 = 0x00000009;
pub const FE_DATA_TYPE_FIXED: u32 = 0x0000000b;
pub const FE_DATA_TYPE_INT_10_10_10_2: u32 = 0x0000000c;
pub const FE_DATA_TYPE_UNSIGNED_INT_10_10_10_2: u32 = 0x0000000d;
pub const FE_DATA_TYPE_BYTE_I: u32 = 0x0000000e;
pub const FE_DATA_TYPE_SHORT_I: u32 = 0x0000000f;
pub const FE_VERTEX_STREAM_CONTROL_VERTEX_STRIDE__MASK: u32 = 0x000000ff;
pub const FE_VERTEX_STREAM_CONTROL_VERTEX_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn FE_VERTEX_STREAM_CONTROL_VERTEX_STRIDE(x: u32) -> u32 { (x << FE_VERTEX_STREAM_CONTROL_VERTEX_STRIDE__SHIFT) & FE_VERTEX_STREAM_CONTROL_VERTEX_STRIDE__MASK }
pub const FE_VERTEX_STREAM_CONTROL_VERTEX_DIVISOR__MASK: u32 = 0x00ff0000;
pub const FE_VERTEX_STREAM_CONTROL_VERTEX_DIVISOR__SHIFT: u32 = 0x00000010;
pub const fn FE_VERTEX_STREAM_CONTROL_VERTEX_DIVISOR(x: u32) -> u32 { (x << FE_VERTEX_STREAM_CONTROL_VERTEX_DIVISOR__SHIFT) & FE_VERTEX_STREAM_CONTROL_VERTEX_DIVISOR__MASK }
pub const VIVS_FE: u32 = 0x00000000;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG__ESIZE: u32 = 0x00000004;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG__LEN: u32 = 0x00000010;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_TYPE__MASK: u32 = 0x0000000f;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_TYPE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_FE_VERTEX_ELEMENT_CONFIG_TYPE(x: u32) -> u32 { (x << VIVS_FE_VERTEX_ELEMENT_CONFIG_TYPE__SHIFT) & VIVS_FE_VERTEX_ELEMENT_CONFIG_TYPE__MASK }
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_ENDIAN__MASK: u32 = 0x00000030;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_ENDIAN__SHIFT: u32 = 0x00000004;
pub const fn VIVS_FE_VERTEX_ELEMENT_CONFIG_ENDIAN(x: u32) -> u32 { (x << VIVS_FE_VERTEX_ELEMENT_CONFIG_ENDIAN__SHIFT) & VIVS_FE_VERTEX_ELEMENT_CONFIG_ENDIAN__MASK }
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_NONCONSECUTIVE: u32 = 0x00000080;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_STREAM__MASK: u32 = 0x00000700;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_STREAM__SHIFT: u32 = 0x00000008;
pub const fn VIVS_FE_VERTEX_ELEMENT_CONFIG_STREAM(x: u32) -> u32 { (x << VIVS_FE_VERTEX_ELEMENT_CONFIG_STREAM__SHIFT) & VIVS_FE_VERTEX_ELEMENT_CONFIG_STREAM__MASK }
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_NUM__MASK: u32 = 0x00003000;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_NUM__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_FE_VERTEX_ELEMENT_CONFIG_NUM(x: u32) -> u32 { (x << VIVS_FE_VERTEX_ELEMENT_CONFIG_NUM__SHIFT) & VIVS_FE_VERTEX_ELEMENT_CONFIG_NUM__MASK }
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_NORMALIZE__MASK: u32 = 0x0000c000;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_NORMALIZE__SHIFT: u32 = 0x0000000e;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_NORMALIZE_OFF: u32 = 0x00000000;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_NORMALIZE_SIGN_EXTEND: u32 = 0x00004000;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_NORMALIZE_ON: u32 = 0x00008000;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_START__MASK: u32 = 0x00ff0000;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_START__SHIFT: u32 = 0x00000010;
pub const fn VIVS_FE_VERTEX_ELEMENT_CONFIG_START(x: u32) -> u32 { (x << VIVS_FE_VERTEX_ELEMENT_CONFIG_START__SHIFT) & VIVS_FE_VERTEX_ELEMENT_CONFIG_START__MASK }
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_END__MASK: u32 = 0xff000000;
pub const VIVS_FE_VERTEX_ELEMENT_CONFIG_END__SHIFT: u32 = 0x00000018;
pub const fn VIVS_FE_VERTEX_ELEMENT_CONFIG_END(x: u32) -> u32 { (x << VIVS_FE_VERTEX_ELEMENT_CONFIG_END__SHIFT) & VIVS_FE_VERTEX_ELEMENT_CONFIG_END__MASK }
pub const VIVS_FE_CMD_STREAM_BASE_ADDR: u32 = 0x00000640;
pub const VIVS_FE_INDEX_STREAM_BASE_ADDR: u32 = 0x00000644;
pub const VIVS_FE_INDEX_STREAM_CONTROL: u32 = 0x00000648;
pub const VIVS_FE_INDEX_STREAM_CONTROL_TYPE__MASK: u32 = 0x00000003;
pub const VIVS_FE_INDEX_STREAM_CONTROL_TYPE__SHIFT: u32 = 0x00000000;
pub const VIVS_FE_INDEX_STREAM_CONTROL_TYPE_UNSIGNED_CHAR: u32 = 0x00000000;
pub const VIVS_FE_INDEX_STREAM_CONTROL_TYPE_UNSIGNED_SHORT: u32 = 0x00000001;
pub const VIVS_FE_INDEX_STREAM_CONTROL_TYPE_UNSIGNED_INT: u32 = 0x00000002;
pub const VIVS_FE_INDEX_STREAM_CONTROL_PRIMITIVE_RESTART: u32 = 0x00000100;
pub const VIVS_FE_VERTEX_STREAM_BASE_ADDR: u32 = 0x0000064c;
pub const VIVS_FE_VERTEX_STREAM_CONTROL: u32 = 0x00000650;
pub const VIVS_FE_COMMAND_ADDRESS: u32 = 0x00000654;
pub const VIVS_FE_COMMAND_CONTROL: u32 = 0x00000658;
pub const VIVS_FE_COMMAND_CONTROL_PREFETCH__MASK: u32 = 0x0000ffff;
pub const VIVS_FE_COMMAND_CONTROL_PREFETCH__SHIFT: u32 = 0x00000000;
pub const fn VIVS_FE_COMMAND_CONTROL_PREFETCH(x: u32) -> u32 { (x << VIVS_FE_COMMAND_CONTROL_PREFETCH__SHIFT) & VIVS_FE_COMMAND_CONTROL_PREFETCH__MASK }
pub const VIVS_FE_COMMAND_CONTROL_ENABLE: u32 = 0x00010000;
pub const VIVS_FE_DMA_STATUS: u32 = 0x0000065c;
pub const VIVS_FE_DMA_DEBUG_STATE: u32 = 0x00000660;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE__MASK: u32 = 0x0000001f;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE__SHIFT: u32 = 0x00000000;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_IDLE: u32 = 0x00000000;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_DEC: u32 = 0x00000001;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_ADR0: u32 = 0x00000002;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_LOAD0: u32 = 0x00000003;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_ADR1: u32 = 0x00000004;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_LOAD1: u32 = 0x00000005;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_3DADR: u32 = 0x00000006;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_3DCMD: u32 = 0x00000007;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_3DCNTL: u32 = 0x00000008;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_3DIDXCNTL: u32 = 0x00000009;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_INITREQDMA: u32 = 0x0000000a;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_DRAWIDX: u32 = 0x0000000b;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_DRAW: u32 = 0x0000000c;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_2DRECT0: u32 = 0x0000000d;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_2DRECT1: u32 = 0x0000000e;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_2DDATA0: u32 = 0x0000000f;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_2DDATA1: u32 = 0x00000010;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_WAITFIFO: u32 = 0x00000011;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_WAIT: u32 = 0x00000012;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_LINK: u32 = 0x00000013;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_END: u32 = 0x00000014;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_STATE_STALL: u32 = 0x00000015;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_DMA_STATE__MASK: u32 = 0x00000300;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_DMA_STATE__SHIFT: u32 = 0x00000008;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_DMA_STATE_IDLE: u32 = 0x00000000;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_DMA_STATE_START: u32 = 0x00000100;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_DMA_STATE_REQ: u32 = 0x00000200;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_DMA_STATE_END: u32 = 0x00000300;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_FETCH_STATE__MASK: u32 = 0x00000c00;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_FETCH_STATE__SHIFT: u32 = 0x0000000a;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_FETCH_STATE_IDLE: u32 = 0x00000000;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_FETCH_STATE_RAMVALID: u32 = 0x00000400;
pub const VIVS_FE_DMA_DEBUG_STATE_CMD_FETCH_STATE_VALID: u32 = 0x00000800;
pub const VIVS_FE_DMA_DEBUG_STATE_REQ_DMA_STATE__MASK: u32 = 0x00003000;
pub const VIVS_FE_DMA_DEBUG_STATE_REQ_DMA_STATE__SHIFT: u32 = 0x0000000c;
pub const VIVS_FE_DMA_DEBUG_STATE_REQ_DMA_STATE_IDLE: u32 = 0x00000000;
pub const VIVS_FE_DMA_DEBUG_STATE_REQ_DMA_STATE_WAITIDX: u32 = 0x00001000;
pub const VIVS_FE_DMA_DEBUG_STATE_REQ_DMA_STATE_CAL: u32 = 0x00002000;
pub const VIVS_FE_DMA_DEBUG_STATE_CAL_STATE__MASK: u32 = 0x0000c000;
pub const VIVS_FE_DMA_DEBUG_STATE_CAL_STATE__SHIFT: u32 = 0x0000000e;
pub const VIVS_FE_DMA_DEBUG_STATE_CAL_STATE_IDLE: u32 = 0x00000000;
pub const VIVS_FE_DMA_DEBUG_STATE_CAL_STATE_LDADR: u32 = 0x00004000;
pub const VIVS_FE_DMA_DEBUG_STATE_CAL_STATE_IDXCALC: u32 = 0x00008000;
pub const VIVS_FE_DMA_DEBUG_STATE_VE_REQ_STATE__MASK: u32 = 0x00030000;
pub const VIVS_FE_DMA_DEBUG_STATE_VE_REQ_STATE__SHIFT: u32 = 0x00000010;
pub const VIVS_FE_DMA_DEBUG_STATE_VE_REQ_STATE_IDLE: u32 = 0x00000000;
pub const VIVS_FE_DMA_DEBUG_STATE_VE_REQ_STATE_CKCACHE: u32 = 0x00010000;
pub const VIVS_FE_DMA_DEBUG_STATE_VE_REQ_STATE_MISS: u32 = 0x00020000;
pub const VIVS_FE_DMA_ADDRESS: u32 = 0x00000664;
pub const VIVS_FE_DMA_LOW: u32 = 0x00000668;
pub const VIVS_FE_DMA_HIGH: u32 = 0x0000066c;
pub const VIVS_FE_AUTO_FLUSH: u32 = 0x00000670;
pub const VIVS_FE_PRIMITIVE_RESTART_INDEX: u32 = 0x00000674;
pub const VIVS_FE_UNK00678: u32 = 0x00000678;
pub const VIVS_FE_UNK0067C: u32 = 0x0000067c;
pub const VIVS_FE_VERTEX_STREAMS__ESIZE: u32 = 0x00000004;
pub const VIVS_FE_VERTEX_STREAMS__LEN: u32 = 0x00000008;
pub const VIVS_FE_GENERIC_ATTRIB__ESIZE: u32 = 0x00000004;
pub const VIVS_FE_GENERIC_ATTRIB__LEN: u32 = 0x00000010;
pub const VIVS_FE_HALTI5_ID_CONFIG: u32 = 0x000007c4;
pub const VIVS_FE_HALTI5_ID_CONFIG_VERTEX_ID_ENABLE: u32 = 0x00000001;
pub const VIVS_FE_HALTI5_ID_CONFIG_INSTANCE_ID_ENABLE: u32 = 0x00000002;
pub const VIVS_FE_HALTI5_ID_CONFIG_VERTEX_ID_REG__MASK: u32 = 0x0000ff00;
pub const VIVS_FE_HALTI5_ID_CONFIG_VERTEX_ID_REG__SHIFT: u32 = 0x00000008;
pub const fn VIVS_FE_HALTI5_ID_CONFIG_VERTEX_ID_REG(x: u32) -> u32 { (x << VIVS_FE_HALTI5_ID_CONFIG_VERTEX_ID_REG__SHIFT) & VIVS_FE_HALTI5_ID_CONFIG_VERTEX_ID_REG__MASK }
pub const VIVS_FE_HALTI5_ID_CONFIG_INSTANCE_ID_REG__MASK: u32 = 0x00ff0000;
pub const VIVS_FE_HALTI5_ID_CONFIG_INSTANCE_ID_REG__SHIFT: u32 = 0x00000010;
pub const fn VIVS_FE_HALTI5_ID_CONFIG_INSTANCE_ID_REG(x: u32) -> u32 { (x << VIVS_FE_HALTI5_ID_CONFIG_INSTANCE_ID_REG__SHIFT) & VIVS_FE_HALTI5_ID_CONFIG_INSTANCE_ID_REG__MASK }
pub const VIVS_FE_HALTI5_UNK007D0__ESIZE: u32 = 0x00000004;
pub const VIVS_FE_HALTI5_UNK007D0__LEN: u32 = 0x00000002;
pub const VIVS_FE_HALTI5_UNK007D8: u32 = 0x000007d8;
pub const VIVS_FE_DESC_START: u32 = 0x000007dc;
pub const VIVS_FE_DESC_END: u32 = 0x000007e0;
pub const VIVS_FE_DESC_AVAIL: u32 = 0x000007e4;
pub const VIVS_FE_DESC_AVAIL_COUNT__MASK: u32 = 0x0000007f;
pub const VIVS_FE_DESC_AVAIL_COUNT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_FE_DESC_AVAIL_COUNT(x: u32) -> u32 { (x << VIVS_FE_DESC_AVAIL_COUNT__SHIFT) & VIVS_FE_DESC_AVAIL_COUNT__MASK }
pub const VIVS_FE_FENCE_WAIT_DATA_LOW: u32 = 0x000007e8;
pub const VIVS_FE_FENCE_WAIT_DATA_HIGH: u32 = 0x000007f4;
pub const VIVS_FE_ROBUSTNESS_UNK007F8: u32 = 0x000007f8;
pub const VIVS_GL: u32 = 0x00000000;
pub const VIVS_GL_PIPE_SELECT: u32 = 0x00003800;
pub const VIVS_GL_PIPE_SELECT_PIPE__MASK: u32 = 0x00000001;
pub const VIVS_GL_PIPE_SELECT_PIPE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_GL_PIPE_SELECT_PIPE(x: u32) -> u32 { (x << VIVS_GL_PIPE_SELECT_PIPE__SHIFT) & VIVS_GL_PIPE_SELECT_PIPE__MASK }
pub const VIVS_GL_EVENT: u32 = 0x00003804;
pub const VIVS_GL_EVENT_EVENT_ID__MASK: u32 = 0x0000001f;
pub const VIVS_GL_EVENT_EVENT_ID__SHIFT: u32 = 0x00000000;
pub const fn VIVS_GL_EVENT_EVENT_ID(x: u32) -> u32 { (x << VIVS_GL_EVENT_EVENT_ID__SHIFT) & VIVS_GL_EVENT_EVENT_ID__MASK }
pub const VIVS_GL_EVENT_FROM_FE: u32 = 0x00000020;
pub const VIVS_GL_EVENT_FROM_PE: u32 = 0x00000040;
pub const VIVS_GL_EVENT_FROM_BLT: u32 = 0x00000080;
pub const VIVS_GL_EVENT_SOURCE__MASK: u32 = 0x00001f00;
pub const VIVS_GL_EVENT_SOURCE__SHIFT: u32 = 0x00000008;
pub const fn VIVS_GL_EVENT_SOURCE(x: u32) -> u32 { (x << VIVS_GL_EVENT_SOURCE__SHIFT) & VIVS_GL_EVENT_SOURCE__MASK }
pub const VIVS_GL_SEMAPHORE_TOKEN: u32 = 0x00003808;
pub const VIVS_GL_SEMAPHORE_TOKEN_FROM__MASK: u32 = 0x0000001f;
pub const VIVS_GL_SEMAPHORE_TOKEN_FROM__SHIFT: u32 = 0x00000000;
pub const fn VIVS_GL_SEMAPHORE_TOKEN_FROM(x: u32) -> u32 { (x << VIVS_GL_SEMAPHORE_TOKEN_FROM__SHIFT) & VIVS_GL_SEMAPHORE_TOKEN_FROM__MASK }
pub const VIVS_GL_SEMAPHORE_TOKEN_TO__MASK: u32 = 0x00001f00;
pub const VIVS_GL_SEMAPHORE_TOKEN_TO__SHIFT: u32 = 0x00000008;
pub const fn VIVS_GL_SEMAPHORE_TOKEN_TO(x: u32) -> u32 { (x << VIVS_GL_SEMAPHORE_TOKEN_TO__SHIFT) & VIVS_GL_SEMAPHORE_TOKEN_TO__MASK }
pub const VIVS_GL_SEMAPHORE_TOKEN_UNK28__MASK: u32 = 0x30000000;
pub const VIVS_GL_SEMAPHORE_TOKEN_UNK28__SHIFT: u32 = 0x0000001c;
pub const fn VIVS_GL_SEMAPHORE_TOKEN_UNK28(x: u32) -> u32 { (x << VIVS_GL_SEMAPHORE_TOKEN_UNK28__SHIFT) & VIVS_GL_SEMAPHORE_TOKEN_UNK28__MASK }
pub const VIVS_GL_FLUSH_CACHE: u32 = 0x0000380c;
pub const VIVS_GL_FLUSH_CACHE_DEPTH: u32 = 0x00000001;
pub const VIVS_GL_FLUSH_CACHE_COLOR: u32 = 0x00000002;
pub const VIVS_GL_FLUSH_CACHE_TEXTURE: u32 = 0x00000004;
pub const VIVS_GL_FLUSH_CACHE_PE2D: u32 = 0x00000008;
pub const VIVS_GL_FLUSH_CACHE_TEXTUREVS: u32 = 0x00000010;
pub const VIVS_GL_FLUSH_CACHE_SHADER_L1: u32 = 0x00000020;
pub const VIVS_GL_FLUSH_CACHE_SHADER_L2: u32 = 0x00000040;
pub const VIVS_GL_FLUSH_CACHE_UNK10: u32 = 0x00000400;
pub const VIVS_GL_FLUSH_CACHE_UNK11: u32 = 0x00000800;
pub const VIVS_GL_FLUSH_CACHE_DESCRIPTOR_UNK12: u32 = 0x00001000;
pub const VIVS_GL_FLUSH_CACHE_DESCRIPTOR_UNK13: u32 = 0x00002000;
pub const VIVS_GL_FLUSH_MMU: u32 = 0x00003810;
pub const VIVS_GL_FLUSH_MMU_FLUSH_FEMMU: u32 = 0x00000001;
pub const VIVS_GL_FLUSH_MMU_FLUSH_UNK1: u32 = 0x00000002;
pub const VIVS_GL_FLUSH_MMU_FLUSH_UNK2: u32 = 0x00000004;
pub const VIVS_GL_FLUSH_MMU_FLUSH_PEMMU: u32 = 0x00000008;
pub const VIVS_GL_FLUSH_MMU_FLUSH_UNK4: u32 = 0x00000010;
pub const VIVS_GL_VERTEX_ELEMENT_CONFIG: u32 = 0x00003814;
pub const VIVS_GL_VERTEX_ELEMENT_CONFIG_UNK0: u32 = 0x00000001;
pub const VIVS_GL_VERTEX_ELEMENT_CONFIG_REUSE: u32 = 0x00000010;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG: u32 = 0x00003818;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_SAMPLES__MASK: u32 = 0x00000003;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_SAMPLES__SHIFT: u32 = 0x00000000;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_SAMPLES_NONE: u32 = 0x00000000;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_SAMPLES_2X: u32 = 0x00000001;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_SAMPLES_4X: u32 = 0x00000002;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_SAMPLES_MASK: u32 = 0x00000008;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_ENABLES__MASK: u32 = 0x000000f0;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_ENABLES__SHIFT: u32 = 0x00000004;
pub const fn VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_ENABLES(x: u32) -> u32 { (x << VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_ENABLES__SHIFT) & VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_ENABLES__MASK }
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_MSAA_ENABLES_MASK: u32 = 0x00000100;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_UNK12__MASK: u32 = 0x00007000;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_UNK12__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_GL_MULTI_SAMPLE_CONFIG_UNK12(x: u32) -> u32 { (x << VIVS_GL_MULTI_SAMPLE_CONFIG_UNK12__SHIFT) & VIVS_GL_MULTI_SAMPLE_CONFIG_UNK12__MASK }
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_UNK12_MASK: u32 = 0x00008000;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_UNK16__MASK: u32 = 0x00030000;
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_UNK16__SHIFT: u32 = 0x00000010;
pub const fn VIVS_GL_MULTI_SAMPLE_CONFIG_UNK16(x: u32) -> u32 { (x << VIVS_GL_MULTI_SAMPLE_CONFIG_UNK16__SHIFT) & VIVS_GL_MULTI_SAMPLE_CONFIG_UNK16__MASK }
pub const VIVS_GL_MULTI_SAMPLE_CONFIG_UNK16_MASK: u32 = 0x00080000;
pub const VIVS_GL_VARYING_TOTAL_COMPONENTS: u32 = 0x0000381c;
pub const VIVS_GL_VARYING_TOTAL_COMPONENTS_NUM__MASK: u32 = 0x000000ff;
pub const VIVS_GL_VARYING_TOTAL_COMPONENTS_NUM__SHIFT: u32 = 0x00000000;
pub const fn VIVS_GL_VARYING_TOTAL_COMPONENTS_NUM(x: u32) -> u32 { (x << VIVS_GL_VARYING_TOTAL_COMPONENTS_NUM__SHIFT) & VIVS_GL_VARYING_TOTAL_COMPONENTS_NUM__MASK }
pub const VIVS_GL_VARYING_NUM_COMPONENTS: u32 = 0x00003820;
pub const VIVS_GL_OCCLUSION_QUERY_ADDR: u32 = 0x00003824;
pub const VIVS_GL_VARYING_COMPONENT_USE__ESIZE: u32 = 0x00000004;
pub const VIVS_GL_VARYING_COMPONENT_USE__LEN: u32 = 0x00000002;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP0__MASK: u32 = 0x00000003;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP0__SHIFT: u32 = 0x00000000;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP0(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP0__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP0__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP1__MASK: u32 = 0x0000000c;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP1__SHIFT: u32 = 0x00000002;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP1(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP1__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP1__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP2__MASK: u32 = 0x00000030;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP2__SHIFT: u32 = 0x00000004;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP2(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP2__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP2__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP3__MASK: u32 = 0x000000c0;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP3__SHIFT: u32 = 0x00000006;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP3(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP3__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP3__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP4__MASK: u32 = 0x00000300;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP4__SHIFT: u32 = 0x00000008;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP4(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP4__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP4__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP5__MASK: u32 = 0x00000c00;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP5__SHIFT: u32 = 0x0000000a;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP5(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP5__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP5__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP6__MASK: u32 = 0x00003000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP6__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP6(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP6__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP6__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP7__MASK: u32 = 0x0000c000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP7__SHIFT: u32 = 0x0000000e;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP7(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP7__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP7__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP8__MASK: u32 = 0x00030000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP8__SHIFT: u32 = 0x00000010;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP8(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP8__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP8__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP9__MASK: u32 = 0x000c0000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP9__SHIFT: u32 = 0x00000012;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP9(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP9__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP9__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP10__MASK: u32 = 0x00300000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP10__SHIFT: u32 = 0x00000014;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP10(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP10__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP10__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP11__MASK: u32 = 0x00c00000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP11__SHIFT: u32 = 0x00000016;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP11(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP11__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP11__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP12__MASK: u32 = 0x03000000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP12__SHIFT: u32 = 0x00000018;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP12(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP12__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP12__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP13__MASK: u32 = 0x0c000000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP13__SHIFT: u32 = 0x0000001a;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP13(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP13__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP13__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP14__MASK: u32 = 0x30000000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP14__SHIFT: u32 = 0x0000001c;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP14(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP14__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP14__MASK }
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP15__MASK: u32 = 0xc0000000;
pub const VIVS_GL_VARYING_COMPONENT_USE_COMP15__SHIFT: u32 = 0x0000001e;
pub const fn VIVS_GL_VARYING_COMPONENT_USE_COMP15(x: u32) -> u32 { (x << VIVS_GL_VARYING_COMPONENT_USE_COMP15__SHIFT) & VIVS_GL_VARYING_COMPONENT_USE_COMP15__MASK }
pub const VIVS_GL_UNK0382C: u32 = 0x0000382c;
pub const VIVS_GL_OCCLUSION_QUERY_CONTROL: u32 = 0x00003830;
pub const VIVS_GL_VARYING_NUM_COMPONENTS2: u32 = 0x00003834;
pub const VIVS_GL_UNK03838: u32 = 0x00003838;
pub const VIVS_GL_API_MODE: u32 = 0x0000384c;
pub const VIVS_GL_API_MODE_OPENGL: u32 = 0x00000000;
pub const VIVS_GL_API_MODE_OPENVG: u32 = 0x00000001;
pub const VIVS_GL_API_MODE_OPENCL: u32 = 0x00000002;
pub const VIVS_GL_CONTEXT_POINTER: u32 = 0x00003850;
pub const VIVS_GL_UNK03854: u32 = 0x00003854;
pub const VIVS_GL_BUG_FIXES: u32 = 0x00003860;
pub const VIVS_GL_FENCE_OUT_ADDRESS: u32 = 0x00003868;
pub const VIVS_GL_FENCE_OUT_DATA_LOW: u32 = 0x0000386c;
pub const VIVS_GL_USC_CONTROL: u32 = 0x00003884;
pub const VIVS_GL_USC_CONTROL_L1_CACHE_RATIO__MASK: u32 = 0x00000007;
pub const VIVS_GL_USC_CONTROL_L1_CACHE_RATIO__SHIFT: u32 = 0x00000000;
pub const fn VIVS_GL_USC_CONTROL_L1_CACHE_RATIO(x: u32) -> u32 { (x << VIVS_GL_USC_CONTROL_L1_CACHE_RATIO__SHIFT) & VIVS_GL_USC_CONTROL_L1_CACHE_RATIO__MASK }
pub const VIVS_GL_USC_CONTROL_ATTRIB_CACHE_RATIO__MASK: u32 = 0x00000f00;
pub const VIVS_GL_USC_CONTROL_ATTRIB_CACHE_RATIO__SHIFT: u32 = 0x00000008;
pub const fn VIVS_GL_USC_CONTROL_ATTRIB_CACHE_RATIO(x: u32) -> u32 { (x << VIVS_GL_USC_CONTROL_ATTRIB_CACHE_RATIO__SHIFT) & VIVS_GL_USC_CONTROL_ATTRIB_CACHE_RATIO__MASK }
pub const VIVS_GL_USC_CONTROL_UNK16__MASK: u32 = 0x001f0000;
pub const VIVS_GL_USC_CONTROL_UNK16__SHIFT: u32 = 0x00000010;
pub const fn VIVS_GL_USC_CONTROL_UNK16(x: u32) -> u32 { (x << VIVS_GL_USC_CONTROL_UNK16__SHIFT) & VIVS_GL_USC_CONTROL_UNK16__MASK }
pub const VIVS_GL_HALTI5_SH_SPECIALS: u32 = 0x00003888;
pub const VIVS_GL_HALTI5_SH_SPECIALS_VS_PSIZE_OUT__MASK: u32 = 0x0000007f;
pub const VIVS_GL_HALTI5_SH_SPECIALS_VS_PSIZE_OUT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_GL_HALTI5_SH_SPECIALS_VS_PSIZE_OUT(x: u32) -> u32 { (x << VIVS_GL_HALTI5_SH_SPECIALS_VS_PSIZE_OUT__SHIFT) & VIVS_GL_HALTI5_SH_SPECIALS_VS_PSIZE_OUT__MASK }
pub const VIVS_GL_HALTI5_SH_SPECIALS_PS_PCOORD_IN__MASK: u32 = 0x00007f00;
pub const VIVS_GL_HALTI5_SH_SPECIALS_PS_PCOORD_IN__SHIFT: u32 = 0x00000008;
pub const fn VIVS_GL_HALTI5_SH_SPECIALS_PS_PCOORD_IN(x: u32) -> u32 { (x << VIVS_GL_HALTI5_SH_SPECIALS_PS_PCOORD_IN__SHIFT) & VIVS_GL_HALTI5_SH_SPECIALS_PS_PCOORD_IN__MASK }
pub const VIVS_GL_HALTI5_SH_SPECIALS_UNK16__MASK: u32 = 0x007f0000;
pub const VIVS_GL_HALTI5_SH_SPECIALS_UNK16__SHIFT: u32 = 0x00000010;
pub const fn VIVS_GL_HALTI5_SH_SPECIALS_UNK16(x: u32) -> u32 { (x << VIVS_GL_HALTI5_SH_SPECIALS_UNK16__SHIFT) & VIVS_GL_HALTI5_SH_SPECIALS_UNK16__MASK }
pub const VIVS_GL_HALTI5_SH_SPECIALS_UNK24__MASK: u32 = 0xff000000;
pub const VIVS_GL_HALTI5_SH_SPECIALS_UNK24__SHIFT: u32 = 0x00000018;
pub const fn VIVS_GL_HALTI5_SH_SPECIALS_UNK24(x: u32) -> u32 { (x << VIVS_GL_HALTI5_SH_SPECIALS_UNK24__SHIFT) & VIVS_GL_HALTI5_SH_SPECIALS_UNK24__MASK }
pub const VIVS_GL_GS_UNK0388C: u32 = 0x0000388c;
pub const VIVS_GL_FENCE_OUT_DATA_HIGH: u32 = 0x00003898;
pub const VIVS_GL_SHADER_INDEX: u32 = 0x0000389c;
pub const VIVS_GL_GS_UNK038A0__ESIZE: u32 = 0x00000004;
pub const VIVS_GL_GS_UNK038A0__LEN: u32 = 0x00000008;
pub const VIVS_GL_HALTI5_UNK038C0__ESIZE: u32 = 0x00000004;
pub const VIVS_GL_HALTI5_UNK038C0__LEN: u32 = 0x00000010;
pub const VIVS_GL_SECURITY_UNK3900: u32 = 0x00003900;
pub const VIVS_GL_SECURITY_UNK3904: u32 = 0x00003904;
pub const VIVS_GL_NN_CONFIG: u32 = 0x00003930;
pub const VIVS_GL_NN_CONFIG_UNK0__MASK: u32 = 0x00000003;
pub const VIVS_GL_NN_CONFIG_UNK0__SHIFT: u32 = 0x00000000;
pub const fn VIVS_GL_NN_CONFIG_UNK0(x: u32) -> u32 { (x << VIVS_GL_NN_CONFIG_UNK0__SHIFT) & VIVS_GL_NN_CONFIG_UNK0__MASK }
pub const VIVS_GL_NN_CONFIG_DISABLE_ZDPN: u32 = 0x00000004;
pub const VIVS_GL_NN_CONFIG_DISABLE_SWTILING: u32 = 0x00000008;
pub const VIVS_GL_NN_CONFIG_SMALL_BATCH: u32 = 0x00000010;
pub const VIVS_GL_NN_CONFIG_DDR_BURST_SIZE__MASK: u32 = 0x00000060;
pub const VIVS_GL_NN_CONFIG_DDR_BURST_SIZE__SHIFT: u32 = 0x00000005;
pub const fn VIVS_GL_NN_CONFIG_DDR_BURST_SIZE(x: u32) -> u32 { (x << VIVS_GL_NN_CONFIG_DDR_BURST_SIZE__SHIFT) & VIVS_GL_NN_CONFIG_DDR_BURST_SIZE__MASK }
pub const VIVS_GL_NN_CONFIG_UNK7: u32 = 0x00000080;
pub const VIVS_GL_NN_CONFIG_NN_CORE_COUNT__MASK: u32 = 0x00000f00;
pub const VIVS_GL_NN_CONFIG_NN_CORE_COUNT__SHIFT: u32 = 0x00000008;
pub const fn VIVS_GL_NN_CONFIG_NN_CORE_COUNT(x: u32) -> u32 { (x << VIVS_GL_NN_CONFIG_NN_CORE_COUNT__SHIFT) & VIVS_GL_NN_CONFIG_NN_CORE_COUNT__MASK }
pub const VIVS_GL_NN_CONFIG_UNK12: u32 = 0x00001000;
pub const VIVS_GL_SRAM_REMAP_ADDRESS: u32 = 0x00003938;
pub const VIVS_GL_OCB_REMAP_START: u32 = 0x0000393c;
pub const VIVS_GL_OCB_REMAP_END: u32 = 0x00003940;
pub const VIVS_GL_TP_CONFIG: u32 = 0x0000394c;
pub const VIVS_GL_UNK03A00: u32 = 0x00003a00;
pub const VIVS_GL_UNK03A04: u32 = 0x00003a04;
pub const VIVS_GL_UNK03A08: u32 = 0x00003a08;
pub const VIVS_GL_UNK03A0C: u32 = 0x00003a0c;
pub const VIVS_GL_UNK03A10: u32 = 0x00003a10;
pub const VIVS_GL_STALL_TOKEN: u32 = 0x00003c00;
pub const VIVS_GL_STALL_TOKEN_FROM__MASK: u32 = 0x0000001f;
pub const VIVS_GL_STALL_TOKEN_FROM__SHIFT: u32 = 0x00000000;
pub const fn VIVS_GL_STALL_TOKEN_FROM(x: u32) -> u32 { (x << VIVS_GL_STALL_TOKEN_FROM__SHIFT) & VIVS_GL_STALL_TOKEN_FROM__MASK }
pub const VIVS_GL_STALL_TOKEN_TO__MASK: u32 = 0x00001f00;
pub const VIVS_GL_STALL_TOKEN_TO__SHIFT: u32 = 0x00000008;
pub const fn VIVS_GL_STALL_TOKEN_TO(x: u32) -> u32 { (x << VIVS_GL_STALL_TOKEN_TO__SHIFT) & VIVS_GL_STALL_TOKEN_TO__MASK }
pub const VIVS_GL_STALL_TOKEN_FLIP0: u32 = 0x40000000;
pub const VIVS_GL_STALL_TOKEN_FLIP1: u32 = 0x80000000;
pub const VIVS_NFE: u32 = 0x00000000;
pub const VIVS_NFE_VERTEX_STREAMS__ESIZE: u32 = 0x00000004;
pub const VIVS_NFE_VERTEX_STREAMS__LEN: u32 = 0x00000010;
pub const VIVS_NFE_GENERIC_ATTRIB__ESIZE: u32 = 0x00000004;
pub const VIVS_NFE_GENERIC_ATTRIB__LEN: u32 = 0x00000020;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_TYPE__MASK: u32 = 0x0000000f;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_TYPE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_NFE_GENERIC_ATTRIB_CONFIG0_TYPE(x: u32) -> u32 { (x << VIVS_NFE_GENERIC_ATTRIB_CONFIG0_TYPE__SHIFT) & VIVS_NFE_GENERIC_ATTRIB_CONFIG0_TYPE__MASK }
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_ENDIAN__MASK: u32 = 0x00000030;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_ENDIAN__SHIFT: u32 = 0x00000004;
pub const fn VIVS_NFE_GENERIC_ATTRIB_CONFIG0_ENDIAN(x: u32) -> u32 { (x << VIVS_NFE_GENERIC_ATTRIB_CONFIG0_ENDIAN__SHIFT) & VIVS_NFE_GENERIC_ATTRIB_CONFIG0_ENDIAN__MASK }
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_STREAM__MASK: u32 = 0x00000700;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_STREAM__SHIFT: u32 = 0x00000008;
pub const fn VIVS_NFE_GENERIC_ATTRIB_CONFIG0_STREAM(x: u32) -> u32 { (x << VIVS_NFE_GENERIC_ATTRIB_CONFIG0_STREAM__SHIFT) & VIVS_NFE_GENERIC_ATTRIB_CONFIG0_STREAM__MASK }
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_NUM__MASK: u32 = 0x00003000;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_NUM__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_NFE_GENERIC_ATTRIB_CONFIG0_NUM(x: u32) -> u32 { (x << VIVS_NFE_GENERIC_ATTRIB_CONFIG0_NUM__SHIFT) & VIVS_NFE_GENERIC_ATTRIB_CONFIG0_NUM__MASK }
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_NORMALIZE__MASK: u32 = 0x0000c000;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_NORMALIZE__SHIFT: u32 = 0x0000000e;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_NORMALIZE_OFF: u32 = 0x00000000;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_NORMALIZE_ON: u32 = 0x00008000;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_START__MASK: u32 = 0x00ff0000;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG0_START__SHIFT: u32 = 0x00000010;
pub const fn VIVS_NFE_GENERIC_ATTRIB_CONFIG0_START(x: u32) -> u32 { (x << VIVS_NFE_GENERIC_ATTRIB_CONFIG0_START__SHIFT) & VIVS_NFE_GENERIC_ATTRIB_CONFIG0_START__MASK }
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG1_END__MASK: u32 = 0x000000ff;
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG1_END__SHIFT: u32 = 0x00000000;
pub const fn VIVS_NFE_GENERIC_ATTRIB_CONFIG1_END(x: u32) -> u32 { (x << VIVS_NFE_GENERIC_ATTRIB_CONFIG1_END__SHIFT) & VIVS_NFE_GENERIC_ATTRIB_CONFIG1_END__MASK }
pub const VIVS_NFE_GENERIC_ATTRIB_CONFIG1_NONCONSECUTIVE: u32 = 0x00000800;
pub const VIVS_DUMMY: u32 = 0x00000000;
pub const VIVS_DUMMY_DUMMY: u32 = 0x0003fffc;
pub const DE_FORMAT_X4R4G4B4: u32 = 0x00000000;
pub const DE_FORMAT_A4R4G4B4: u32 = 0x00000001;
pub const DE_FORMAT_X1R5G5B5: u32 = 0x00000002;
pub const DE_FORMAT_A1R5G5B5: u32 = 0x00000003;
pub const DE_FORMAT_R5G6B5: u32 = 0x00000004;
pub const DE_FORMAT_X8R8G8B8: u32 = 0x00000005;
pub const DE_FORMAT_A8R8G8B8: u32 = 0x00000006;
pub const DE_FORMAT_YUY2: u32 = 0x00000007;
pub const DE_FORMAT_UYVY: u32 = 0x00000008;
pub const DE_FORMAT_INDEX8: u32 = 0x00000009;
pub const DE_FORMAT_MONOCHROME: u32 = 0x0000000a;
pub const DE_FORMAT_YV12: u32 = 0x0000000f;
pub const DE_FORMAT_A8: u32 = 0x00000010;
pub const DE_FORMAT_NV12: u32 = 0x00000011;
pub const DE_FORMAT_NV16: u32 = 0x00000012;
pub const DE_FORMAT_RG16: u32 = 0x00000013;
pub const DE_SWIZZLE_ARGB: u32 = 0x00000000;
pub const DE_SWIZZLE_RGBA: u32 = 0x00000001;
pub const DE_SWIZZLE_ABGR: u32 = 0x00000002;
pub const DE_SWIZZLE_BGRA: u32 = 0x00000003;
pub const DE_BLENDMODE_ZERO: u32 = 0x00000000;
pub const DE_BLENDMODE_ONE: u32 = 0x00000001;
pub const DE_BLENDMODE_NORMAL: u32 = 0x00000002;
pub const DE_BLENDMODE_INVERSED: u32 = 0x00000003;
pub const DE_BLENDMODE_COLOR: u32 = 0x00000004;
pub const DE_BLENDMODE_COLOR_INVERSED: u32 = 0x00000005;
pub const DE_BLENDMODE_SATURATED_ALPHA: u32 = 0x00000006;
pub const DE_BLENDMODE_SATURATED_DEST_ALPHA: u32 = 0x00000007;
pub const DE_COMPONENT_BLUE: u32 = 0x00000000;
pub const DE_COMPONENT_GREEN: u32 = 0x00000001;
pub const DE_COMPONENT_RED: u32 = 0x00000002;
pub const DE_COMPONENT_ALPHA: u32 = 0x00000003;
pub const DE_ROT_MODE_ROT0: u32 = 0x00000000;
pub const DE_ROT_MODE_FLIP_X: u32 = 0x00000001;
pub const DE_ROT_MODE_FLIP_Y: u32 = 0x00000002;
pub const DE_ROT_MODE_ROT90: u32 = 0x00000004;
pub const DE_ROT_MODE_ROT180: u32 = 0x00000005;
pub const DE_ROT_MODE_ROT270: u32 = 0x00000006;
pub const DE_MIRROR_MODE_NONE: u32 = 0x00000000;
pub const DE_MIRROR_MODE_MIRROR_X: u32 = 0x00000001;
pub const DE_MIRROR_MODE_MIRROR_Y: u32 = 0x00000002;
pub const DE_MIRROR_MODE_MIRROR_XY: u32 = 0x00000003;
pub const DE_COLOR_BLUE__MASK: u32 = 0x000000ff;
pub const DE_COLOR_BLUE__SHIFT: u32 = 0x00000000;
pub const fn DE_COLOR_BLUE(x: u32) -> u32 { (x << DE_COLOR_BLUE__SHIFT) & DE_COLOR_BLUE__MASK }
pub const DE_COLOR_GREEN__MASK: u32 = 0x0000ff00;
pub const DE_COLOR_GREEN__SHIFT: u32 = 0x00000008;
pub const fn DE_COLOR_GREEN(x: u32) -> u32 { (x << DE_COLOR_GREEN__SHIFT) & DE_COLOR_GREEN__MASK }
pub const DE_COLOR_RED__MASK: u32 = 0x00ff0000;
pub const DE_COLOR_RED__SHIFT: u32 = 0x00000010;
pub const fn DE_COLOR_RED(x: u32) -> u32 { (x << DE_COLOR_RED__SHIFT) & DE_COLOR_RED__MASK }
pub const DE_COLOR_ALPHA__MASK: u32 = 0xff000000;
pub const DE_COLOR_ALPHA__SHIFT: u32 = 0x00000018;
pub const fn DE_COLOR_ALPHA(x: u32) -> u32 { (x << DE_COLOR_ALPHA__SHIFT) & DE_COLOR_ALPHA__MASK }
pub const VIVS_DE: u32 = 0x00000000;
pub const VIVS_DE_SRC_ADDRESS: u32 = 0x00001200;
pub const VIVS_DE_SRC_STRIDE: u32 = 0x00001204;
pub const VIVS_DE_SRC_STRIDE_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_SRC_STRIDE_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_SRC_STRIDE_STRIDE(x: u32) -> u32 { (x << VIVS_DE_SRC_STRIDE_STRIDE__SHIFT) & VIVS_DE_SRC_STRIDE_STRIDE__MASK }
pub const VIVS_DE_SRC_ROTATION_CONFIG: u32 = 0x00001208;
pub const VIVS_DE_SRC_ROTATION_CONFIG_WIDTH__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_SRC_ROTATION_CONFIG_WIDTH__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_SRC_ROTATION_CONFIG_WIDTH(x: u32) -> u32 { (x << VIVS_DE_SRC_ROTATION_CONFIG_WIDTH__SHIFT) & VIVS_DE_SRC_ROTATION_CONFIG_WIDTH__MASK }
pub const VIVS_DE_SRC_ROTATION_CONFIG_ROTATION__MASK: u32 = 0x00010000;
pub const VIVS_DE_SRC_ROTATION_CONFIG_ROTATION__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_SRC_ROTATION_CONFIG_ROTATION_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_SRC_ROTATION_CONFIG_ROTATION_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_SRC_CONFIG: u32 = 0x0000120c;
pub const VIVS_DE_SRC_CONFIG_PE10_SOURCE_FORMAT__MASK: u32 = 0x0000000f;
pub const VIVS_DE_SRC_CONFIG_PE10_SOURCE_FORMAT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_SRC_CONFIG_PE10_SOURCE_FORMAT(x: u32) -> u32 { (x << VIVS_DE_SRC_CONFIG_PE10_SOURCE_FORMAT__SHIFT) & VIVS_DE_SRC_CONFIG_PE10_SOURCE_FORMAT__MASK }
pub const VIVS_DE_SRC_CONFIG_TRANSPARENCY__MASK: u32 = 0x00000030;
pub const VIVS_DE_SRC_CONFIG_TRANSPARENCY__SHIFT: u32 = 0x00000004;
pub const fn VIVS_DE_SRC_CONFIG_TRANSPARENCY(x: u32) -> u32 { (x << VIVS_DE_SRC_CONFIG_TRANSPARENCY__SHIFT) & VIVS_DE_SRC_CONFIG_TRANSPARENCY__MASK }
pub const VIVS_DE_SRC_CONFIG_SRC_RELATIVE__MASK: u32 = 0x00000040;
pub const VIVS_DE_SRC_CONFIG_SRC_RELATIVE__SHIFT: u32 = 0x00000006;
pub const VIVS_DE_SRC_CONFIG_SRC_RELATIVE_ABSOLUTE: u32 = 0x00000000;
pub const VIVS_DE_SRC_CONFIG_SRC_RELATIVE_RELATIVE: u32 = 0x00000040;
pub const VIVS_DE_SRC_CONFIG_TILED__MASK: u32 = 0x00000080;
pub const VIVS_DE_SRC_CONFIG_TILED__SHIFT: u32 = 0x00000007;
pub const VIVS_DE_SRC_CONFIG_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_SRC_CONFIG_TILED_ENABLE: u32 = 0x00000080;
pub const VIVS_DE_SRC_CONFIG_LOCATION__MASK: u32 = 0x00000100;
pub const VIVS_DE_SRC_CONFIG_LOCATION__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_SRC_CONFIG_LOCATION_MEMORY: u32 = 0x00000000;
pub const VIVS_DE_SRC_CONFIG_LOCATION_STREAM: u32 = 0x00000100;
pub const VIVS_DE_SRC_CONFIG_PACK__MASK: u32 = 0x00003000;
pub const VIVS_DE_SRC_CONFIG_PACK__SHIFT: u32 = 0x0000000c;
pub const VIVS_DE_SRC_CONFIG_PACK_PACKED8: u32 = 0x00000000;
pub const VIVS_DE_SRC_CONFIG_PACK_PACKED16: u32 = 0x00001000;
pub const VIVS_DE_SRC_CONFIG_PACK_PACKED32: u32 = 0x00002000;
pub const VIVS_DE_SRC_CONFIG_PACK_UNPACKED: u32 = 0x00003000;
pub const VIVS_DE_SRC_CONFIG_MONO_TRANSPARENCY__MASK: u32 = 0x00008000;
pub const VIVS_DE_SRC_CONFIG_MONO_TRANSPARENCY__SHIFT: u32 = 0x0000000f;
pub const VIVS_DE_SRC_CONFIG_MONO_TRANSPARENCY_BACKGROUND: u32 = 0x00000000;
pub const VIVS_DE_SRC_CONFIG_MONO_TRANSPARENCY_FOREGROUND: u32 = 0x00008000;
pub const VIVS_DE_SRC_CONFIG_UNK16: u32 = 0x00010000;
pub const VIVS_DE_SRC_CONFIG_SWIZZLE__MASK: u32 = 0x00300000;
pub const VIVS_DE_SRC_CONFIG_SWIZZLE__SHIFT: u32 = 0x00000014;
pub const fn VIVS_DE_SRC_CONFIG_SWIZZLE(x: u32) -> u32 { (x << VIVS_DE_SRC_CONFIG_SWIZZLE__SHIFT) & VIVS_DE_SRC_CONFIG_SWIZZLE__MASK }
pub const VIVS_DE_SRC_CONFIG_SOURCE_FORMAT__MASK: u32 = 0x1f000000;
pub const VIVS_DE_SRC_CONFIG_SOURCE_FORMAT__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_SRC_CONFIG_SOURCE_FORMAT(x: u32) -> u32 { (x << VIVS_DE_SRC_CONFIG_SOURCE_FORMAT__SHIFT) & VIVS_DE_SRC_CONFIG_SOURCE_FORMAT__MASK }
pub const VIVS_DE_SRC_CONFIG_DISABLE420_L2_CACHE: u32 = 0x20000000;
pub const VIVS_DE_SRC_CONFIG_ENDIAN_CONTROL__MASK: u32 = 0xc0000000;
pub const VIVS_DE_SRC_CONFIG_ENDIAN_CONTROL__SHIFT: u32 = 0x0000001e;
pub const fn VIVS_DE_SRC_CONFIG_ENDIAN_CONTROL(x: u32) -> u32 { (x << VIVS_DE_SRC_CONFIG_ENDIAN_CONTROL__SHIFT) & VIVS_DE_SRC_CONFIG_ENDIAN_CONTROL__MASK }
pub const VIVS_DE_SRC_ORIGIN: u32 = 0x00001210;
pub const VIVS_DE_SRC_ORIGIN_X__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_SRC_ORIGIN_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_SRC_ORIGIN_X(x: u32) -> u32 { (x << VIVS_DE_SRC_ORIGIN_X__SHIFT) & VIVS_DE_SRC_ORIGIN_X__MASK }
pub const VIVS_DE_SRC_ORIGIN_Y__MASK: u32 = 0xffff0000;
pub const VIVS_DE_SRC_ORIGIN_Y__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_SRC_ORIGIN_Y(x: u32) -> u32 { (x << VIVS_DE_SRC_ORIGIN_Y__SHIFT) & VIVS_DE_SRC_ORIGIN_Y__MASK }
pub const VIVS_DE_SRC_SIZE: u32 = 0x00001214;
pub const VIVS_DE_SRC_SIZE_X__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_SRC_SIZE_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_SRC_SIZE_X(x: u32) -> u32 { (x << VIVS_DE_SRC_SIZE_X__SHIFT) & VIVS_DE_SRC_SIZE_X__MASK }
pub const VIVS_DE_SRC_SIZE_Y__MASK: u32 = 0xffff0000;
pub const VIVS_DE_SRC_SIZE_Y__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_SRC_SIZE_Y(x: u32) -> u32 { (x << VIVS_DE_SRC_SIZE_Y__SHIFT) & VIVS_DE_SRC_SIZE_Y__MASK }
pub const VIVS_DE_SRC_COLOR_BG: u32 = 0x00001218;
pub const VIVS_DE_SRC_COLOR_FG: u32 = 0x0000121c;
pub const VIVS_DE_STRETCH_FACTOR_LOW: u32 = 0x00001220;
pub const VIVS_DE_STRETCH_FACTOR_LOW_X__MASK: u32 = 0x7fffffff;
pub const VIVS_DE_STRETCH_FACTOR_LOW_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_STRETCH_FACTOR_LOW_X(x: u32) -> u32 { (x << VIVS_DE_STRETCH_FACTOR_LOW_X__SHIFT) & VIVS_DE_STRETCH_FACTOR_LOW_X__MASK }
pub const VIVS_DE_STRETCH_FACTOR_HIGH: u32 = 0x00001224;
pub const VIVS_DE_STRETCH_FACTOR_HIGH_Y__MASK: u32 = 0x7fffffff;
pub const VIVS_DE_STRETCH_FACTOR_HIGH_Y__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_STRETCH_FACTOR_HIGH_Y(x: u32) -> u32 { (x << VIVS_DE_STRETCH_FACTOR_HIGH_Y__SHIFT) & VIVS_DE_STRETCH_FACTOR_HIGH_Y__MASK }
pub const VIVS_DE_DEST_ADDRESS: u32 = 0x00001228;
pub const VIVS_DE_DEST_STRIDE: u32 = 0x0000122c;
pub const VIVS_DE_DEST_STRIDE_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_DEST_STRIDE_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_DEST_STRIDE_STRIDE(x: u32) -> u32 { (x << VIVS_DE_DEST_STRIDE_STRIDE__SHIFT) & VIVS_DE_DEST_STRIDE_STRIDE__MASK }
pub const VIVS_DE_DEST_ROTATION_CONFIG: u32 = 0x00001230;
pub const VIVS_DE_DEST_ROTATION_CONFIG_WIDTH__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_DEST_ROTATION_CONFIG_WIDTH__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_DEST_ROTATION_CONFIG_WIDTH(x: u32) -> u32 { (x << VIVS_DE_DEST_ROTATION_CONFIG_WIDTH__SHIFT) & VIVS_DE_DEST_ROTATION_CONFIG_WIDTH__MASK }
pub const VIVS_DE_DEST_ROTATION_CONFIG_ROTATION__MASK: u32 = 0x00010000;
pub const VIVS_DE_DEST_ROTATION_CONFIG_ROTATION__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_DEST_ROTATION_CONFIG_ROTATION_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_DEST_ROTATION_CONFIG_ROTATION_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_DEST_CONFIG: u32 = 0x00001234;
pub const VIVS_DE_DEST_CONFIG_FORMAT__MASK: u32 = 0x0000001f;
pub const VIVS_DE_DEST_CONFIG_FORMAT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_DEST_CONFIG_FORMAT(x: u32) -> u32 { (x << VIVS_DE_DEST_CONFIG_FORMAT__SHIFT) & VIVS_DE_DEST_CONFIG_FORMAT__MASK }
pub const VIVS_DE_DEST_CONFIG_TILED__MASK: u32 = 0x00000100;
pub const VIVS_DE_DEST_CONFIG_TILED__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_DEST_CONFIG_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_DEST_CONFIG_TILED_ENABLE: u32 = 0x00000100;
pub const VIVS_DE_DEST_CONFIG_COMMAND__MASK: u32 = 0x0000f000;
pub const VIVS_DE_DEST_CONFIG_COMMAND__SHIFT: u32 = 0x0000000c;
pub const VIVS_DE_DEST_CONFIG_COMMAND_CLEAR: u32 = 0x00000000;
pub const VIVS_DE_DEST_CONFIG_COMMAND_LINE: u32 = 0x00001000;
pub const VIVS_DE_DEST_CONFIG_COMMAND_BIT_BLT: u32 = 0x00002000;
pub const VIVS_DE_DEST_CONFIG_COMMAND_BIT_BLT_REVERSED: u32 = 0x00003000;
pub const VIVS_DE_DEST_CONFIG_COMMAND_STRETCH_BLT: u32 = 0x00004000;
pub const VIVS_DE_DEST_CONFIG_COMMAND_HOR_FILTER_BLT: u32 = 0x00005000;
pub const VIVS_DE_DEST_CONFIG_COMMAND_VER_FILTER_BLT: u32 = 0x00006000;
pub const VIVS_DE_DEST_CONFIG_COMMAND_ONE_PASS_FILTER_BLT: u32 = 0x00007000;
pub const VIVS_DE_DEST_CONFIG_COMMAND_MULTI_SOURCE_BLT: u32 = 0x00008000;
pub const VIVS_DE_DEST_CONFIG_SWIZZLE__MASK: u32 = 0x00030000;
pub const VIVS_DE_DEST_CONFIG_SWIZZLE__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_DEST_CONFIG_SWIZZLE(x: u32) -> u32 { (x << VIVS_DE_DEST_CONFIG_SWIZZLE__SHIFT) & VIVS_DE_DEST_CONFIG_SWIZZLE__MASK }
pub const VIVS_DE_DEST_CONFIG_ENDIAN_CONTROL__MASK: u32 = 0x00300000;
pub const VIVS_DE_DEST_CONFIG_ENDIAN_CONTROL__SHIFT: u32 = 0x00000014;
pub const fn VIVS_DE_DEST_CONFIG_ENDIAN_CONTROL(x: u32) -> u32 { (x << VIVS_DE_DEST_CONFIG_ENDIAN_CONTROL__SHIFT) & VIVS_DE_DEST_CONFIG_ENDIAN_CONTROL__MASK }
pub const VIVS_DE_DEST_CONFIG_GDI_STRE__MASK: u32 = 0x01000000;
pub const VIVS_DE_DEST_CONFIG_GDI_STRE__SHIFT: u32 = 0x00000018;
pub const VIVS_DE_DEST_CONFIG_GDI_STRE_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_DEST_CONFIG_GDI_STRE_ENABLE: u32 = 0x01000000;
pub const VIVS_DE_DEST_CONFIG_INTER_TILE_PER_FIX__MASK: u32 = 0x02000000;
pub const VIVS_DE_DEST_CONFIG_INTER_TILE_PER_FIX__SHIFT: u32 = 0x00000019;
pub const VIVS_DE_DEST_CONFIG_INTER_TILE_PER_FIX_DISABLED: u32 = 0x02000000;
pub const VIVS_DE_DEST_CONFIG_INTER_TILE_PER_FIX_ENABLED: u32 = 0x00000000;
pub const VIVS_DE_DEST_CONFIG_MINOR_TILED__MASK: u32 = 0x04000000;
pub const VIVS_DE_DEST_CONFIG_MINOR_TILED__SHIFT: u32 = 0x0000001a;
pub const VIVS_DE_DEST_CONFIG_MINOR_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_DEST_CONFIG_MINOR_TILED_ENABLE: u32 = 0x04000000;
pub const VIVS_DE_PATTERN_ADDRESS: u32 = 0x00001238;
pub const VIVS_DE_PATTERN_CONFIG: u32 = 0x0000123c;
pub const VIVS_DE_PATTERN_CONFIG_FORMAT__MASK: u32 = 0x0000000f;
pub const VIVS_DE_PATTERN_CONFIG_FORMAT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_PATTERN_CONFIG_FORMAT(x: u32) -> u32 { (x << VIVS_DE_PATTERN_CONFIG_FORMAT__SHIFT) & VIVS_DE_PATTERN_CONFIG_FORMAT__MASK }
pub const VIVS_DE_PATTERN_CONFIG_TYPE__MASK: u32 = 0x00000010;
pub const VIVS_DE_PATTERN_CONFIG_TYPE__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_PATTERN_CONFIG_TYPE_SOLID_COLOR: u32 = 0x00000000;
pub const VIVS_DE_PATTERN_CONFIG_TYPE_PATTERN: u32 = 0x00000010;
pub const VIVS_DE_PATTERN_CONFIG_COLOR_CONVERT__MASK: u32 = 0x00000020;
pub const VIVS_DE_PATTERN_CONFIG_COLOR_CONVERT__SHIFT: u32 = 0x00000005;
pub const VIVS_DE_PATTERN_CONFIG_COLOR_CONVERT_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_PATTERN_CONFIG_COLOR_CONVERT_ENABLE: u32 = 0x00000020;
pub const VIVS_DE_PATTERN_CONFIG_INIT_TRIGGER__MASK: u32 = 0x000000c0;
pub const VIVS_DE_PATTERN_CONFIG_INIT_TRIGGER__SHIFT: u32 = 0x00000006;
pub const fn VIVS_DE_PATTERN_CONFIG_INIT_TRIGGER(x: u32) -> u32 { (x << VIVS_DE_PATTERN_CONFIG_INIT_TRIGGER__SHIFT) & VIVS_DE_PATTERN_CONFIG_INIT_TRIGGER__MASK }
pub const VIVS_DE_PATTERN_CONFIG_ORIGIN_X__MASK: u32 = 0x00070000;
pub const VIVS_DE_PATTERN_CONFIG_ORIGIN_X__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_PATTERN_CONFIG_ORIGIN_X(x: u32) -> u32 { (x << VIVS_DE_PATTERN_CONFIG_ORIGIN_X__SHIFT) & VIVS_DE_PATTERN_CONFIG_ORIGIN_X__MASK }
pub const VIVS_DE_PATTERN_CONFIG_ORIGIN_Y__MASK: u32 = 0x00700000;
pub const VIVS_DE_PATTERN_CONFIG_ORIGIN_Y__SHIFT: u32 = 0x00000014;
pub const fn VIVS_DE_PATTERN_CONFIG_ORIGIN_Y(x: u32) -> u32 { (x << VIVS_DE_PATTERN_CONFIG_ORIGIN_Y__SHIFT) & VIVS_DE_PATTERN_CONFIG_ORIGIN_Y__MASK }
pub const VIVS_DE_PATTERN_LOW: u32 = 0x00001240;
pub const VIVS_DE_PATTERN_HIGH: u32 = 0x00001244;
pub const VIVS_DE_PATTERN_MASK_LOW: u32 = 0x00001248;
pub const VIVS_DE_PATTERN_MASK_HIGH: u32 = 0x0000124c;
pub const VIVS_DE_PATTERN_BG_COLOR: u32 = 0x00001250;
pub const VIVS_DE_PATTERN_FG_COLOR: u32 = 0x00001254;
pub const VIVS_DE_ROP: u32 = 0x0000125c;
pub const VIVS_DE_ROP_ROP_FG__MASK: u32 = 0x000000ff;
pub const VIVS_DE_ROP_ROP_FG__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_ROP_ROP_FG(x: u32) -> u32 { (x << VIVS_DE_ROP_ROP_FG__SHIFT) & VIVS_DE_ROP_ROP_FG__MASK }
pub const VIVS_DE_ROP_ROP_BG__MASK: u32 = 0x0000ff00;
pub const VIVS_DE_ROP_ROP_BG__SHIFT: u32 = 0x00000008;
pub const fn VIVS_DE_ROP_ROP_BG(x: u32) -> u32 { (x << VIVS_DE_ROP_ROP_BG__SHIFT) & VIVS_DE_ROP_ROP_BG__MASK }
pub const VIVS_DE_ROP_TYPE__MASK: u32 = 0x00300000;
pub const VIVS_DE_ROP_TYPE__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_ROP_TYPE_ROP2_PATTERN: u32 = 0x00000000;
pub const VIVS_DE_ROP_TYPE_ROP2_SOURCE: u32 = 0x00100000;
pub const VIVS_DE_ROP_TYPE_ROP3: u32 = 0x00200000;
pub const VIVS_DE_ROP_TYPE_ROP4: u32 = 0x00300000;
pub const VIVS_DE_CLIP_TOP_LEFT: u32 = 0x00001260;
pub const VIVS_DE_CLIP_TOP_LEFT_X__MASK: u32 = 0x00007fff;
pub const VIVS_DE_CLIP_TOP_LEFT_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_CLIP_TOP_LEFT_X(x: u32) -> u32 { (x << VIVS_DE_CLIP_TOP_LEFT_X__SHIFT) & VIVS_DE_CLIP_TOP_LEFT_X__MASK }
pub const VIVS_DE_CLIP_TOP_LEFT_Y__MASK: u32 = 0x7fff0000;
pub const VIVS_DE_CLIP_TOP_LEFT_Y__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_CLIP_TOP_LEFT_Y(x: u32) -> u32 { (x << VIVS_DE_CLIP_TOP_LEFT_Y__SHIFT) & VIVS_DE_CLIP_TOP_LEFT_Y__MASK }
pub const VIVS_DE_CLIP_BOTTOM_RIGHT: u32 = 0x00001264;
pub const VIVS_DE_CLIP_BOTTOM_RIGHT_X__MASK: u32 = 0x00007fff;
pub const VIVS_DE_CLIP_BOTTOM_RIGHT_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_CLIP_BOTTOM_RIGHT_X(x: u32) -> u32 { (x << VIVS_DE_CLIP_BOTTOM_RIGHT_X__SHIFT) & VIVS_DE_CLIP_BOTTOM_RIGHT_X__MASK }
pub const VIVS_DE_CLIP_BOTTOM_RIGHT_Y__MASK: u32 = 0x7fff0000;
pub const VIVS_DE_CLIP_BOTTOM_RIGHT_Y__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_CLIP_BOTTOM_RIGHT_Y(x: u32) -> u32 { (x << VIVS_DE_CLIP_BOTTOM_RIGHT_Y__SHIFT) & VIVS_DE_CLIP_BOTTOM_RIGHT_Y__MASK }
pub const VIVS_DE_CLEAR_BYTE_MASK: u32 = 0x00001268;
pub const VIVS_DE_CONFIG: u32 = 0x0000126c;
pub const VIVS_DE_CONFIG_MIRROR_BLT_ENABLE__MASK: u32 = 0x00000001;
pub const VIVS_DE_CONFIG_MIRROR_BLT_ENABLE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_CONFIG_MIRROR_BLT_ENABLE_OFF: u32 = 0x00000000;
pub const VIVS_DE_CONFIG_MIRROR_BLT_ENABLE_ON: u32 = 0x00000001;
pub const VIVS_DE_CONFIG_MIRROR_BLT_MODE__MASK: u32 = 0x00000030;
pub const VIVS_DE_CONFIG_MIRROR_BLT_MODE__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_CONFIG_MIRROR_BLT_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_CONFIG_MIRROR_BLT_MODE_HMIRROR: u32 = 0x00000010;
pub const VIVS_DE_CONFIG_MIRROR_BLT_MODE_VMIRROR: u32 = 0x00000020;
pub const VIVS_DE_CONFIG_MIRROR_BLT_MODE_FULL_MIRROR: u32 = 0x00000030;
pub const VIVS_DE_CONFIG_SOURCE_SELECT__MASK: u32 = 0x00070000;
pub const VIVS_DE_CONFIG_SOURCE_SELECT__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_CONFIG_SOURCE_SELECT(x: u32) -> u32 { (x << VIVS_DE_CONFIG_SOURCE_SELECT__SHIFT) & VIVS_DE_CONFIG_SOURCE_SELECT__MASK }
pub const VIVS_DE_CONFIG_DESTINATION_SELECT__MASK: u32 = 0x00300000;
pub const VIVS_DE_CONFIG_DESTINATION_SELECT__SHIFT: u32 = 0x00000014;
pub const fn VIVS_DE_CONFIG_DESTINATION_SELECT(x: u32) -> u32 { (x << VIVS_DE_CONFIG_DESTINATION_SELECT__SHIFT) & VIVS_DE_CONFIG_DESTINATION_SELECT__MASK }
pub const VIVS_DE_CLEAR_PIXEL_VALUE_LOW: u32 = 0x00001270;
pub const VIVS_DE_CLEAR_PIXEL_VALUE_HIGH: u32 = 0x00001274;
pub const VIVS_DE_SRC_ORIGIN_FRACTION: u32 = 0x00001278;
pub const VIVS_DE_SRC_ORIGIN_FRACTION_X__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_SRC_ORIGIN_FRACTION_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_SRC_ORIGIN_FRACTION_X(x: u32) -> u32 { (x << VIVS_DE_SRC_ORIGIN_FRACTION_X__SHIFT) & VIVS_DE_SRC_ORIGIN_FRACTION_X__MASK }
pub const VIVS_DE_SRC_ORIGIN_FRACTION_Y__MASK: u32 = 0xffff0000;
pub const VIVS_DE_SRC_ORIGIN_FRACTION_Y__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_SRC_ORIGIN_FRACTION_Y(x: u32) -> u32 { (x << VIVS_DE_SRC_ORIGIN_FRACTION_Y__SHIFT) & VIVS_DE_SRC_ORIGIN_FRACTION_Y__MASK }
pub const VIVS_DE_ALPHA_CONTROL: u32 = 0x0000127c;
pub const VIVS_DE_ALPHA_CONTROL_ENABLE__MASK: u32 = 0x00000001;
pub const VIVS_DE_ALPHA_CONTROL_ENABLE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_CONTROL_ENABLE_OFF: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_CONTROL_ENABLE_ON: u32 = 0x00000001;
pub const VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__MASK: u32 = 0x00ff0000;
pub const VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA(x: u32) -> u32 { (x << VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__SHIFT) & VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__MASK }
pub const VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__MASK: u32 = 0xff000000;
pub const VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA(x: u32) -> u32 { (x << VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__SHIFT) & VIVS_DE_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__MASK }
pub const VIVS_DE_ALPHA_MODES: u32 = 0x00001280;
pub const VIVS_DE_ALPHA_MODES_SRC_ALPHA_MODE__MASK: u32 = 0x00000001;
pub const VIVS_DE_ALPHA_MODES_SRC_ALPHA_MODE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_MODES_SRC_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_MODES_SRC_ALPHA_MODE_INVERSED: u32 = 0x00000001;
pub const VIVS_DE_ALPHA_MODES_DST_ALPHA_MODE__MASK: u32 = 0x00000010;
pub const VIVS_DE_ALPHA_MODES_DST_ALPHA_MODE__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_ALPHA_MODES_DST_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_MODES_DST_ALPHA_MODE_INVERSED: u32 = 0x00000010;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE__MASK: u32 = 0x00000300;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_GLOBAL: u32 = 0x00000100;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_SCALED: u32 = 0x00000200;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE__MASK: u32 = 0x00003000;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE__SHIFT: u32 = 0x0000000c;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE_GLOBAL: u32 = 0x00001000;
pub const VIVS_DE_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE_SCALED: u32 = 0x00002000;
pub const VIVS_DE_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY__MASK: u32 = 0x00010000;
pub const VIVS_DE_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY__MASK: u32 = 0x00100000;
pub const VIVS_DE_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY_ENABLE: u32 = 0x00100000;
pub const VIVS_DE_ALPHA_MODES_SRC_BLENDING_MODE__MASK: u32 = 0x07000000;
pub const VIVS_DE_ALPHA_MODES_SRC_BLENDING_MODE__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_ALPHA_MODES_SRC_BLENDING_MODE(x: u32) -> u32 { (x << VIVS_DE_ALPHA_MODES_SRC_BLENDING_MODE__SHIFT) & VIVS_DE_ALPHA_MODES_SRC_BLENDING_MODE__MASK }
pub const VIVS_DE_ALPHA_MODES_SRC_ALPHA_FACTOR__MASK: u32 = 0x08000000;
pub const VIVS_DE_ALPHA_MODES_SRC_ALPHA_FACTOR__SHIFT: u32 = 0x0000001b;
pub const VIVS_DE_ALPHA_MODES_SRC_ALPHA_FACTOR_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_MODES_SRC_ALPHA_FACTOR_ENABLE: u32 = 0x08000000;
pub const VIVS_DE_ALPHA_MODES_DST_BLENDING_MODE__MASK: u32 = 0x70000000;
pub const VIVS_DE_ALPHA_MODES_DST_BLENDING_MODE__SHIFT: u32 = 0x0000001c;
pub const fn VIVS_DE_ALPHA_MODES_DST_BLENDING_MODE(x: u32) -> u32 { (x << VIVS_DE_ALPHA_MODES_DST_BLENDING_MODE__SHIFT) & VIVS_DE_ALPHA_MODES_DST_BLENDING_MODE__MASK }
pub const VIVS_DE_ALPHA_MODES_DST_ALPHA_FACTOR__MASK: u32 = 0x80000000;
pub const VIVS_DE_ALPHA_MODES_DST_ALPHA_FACTOR__SHIFT: u32 = 0x0000001f;
pub const VIVS_DE_ALPHA_MODES_DST_ALPHA_FACTOR_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_ALPHA_MODES_DST_ALPHA_FACTOR_ENABLE: u32 = 0x80000000;
pub const VIVS_DE_UPLANE_ADDRESS: u32 = 0x00001284;
pub const VIVS_DE_UPLANE_STRIDE: u32 = 0x00001288;
pub const VIVS_DE_UPLANE_STRIDE_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_UPLANE_STRIDE_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_UPLANE_STRIDE_STRIDE(x: u32) -> u32 { (x << VIVS_DE_UPLANE_STRIDE_STRIDE__SHIFT) & VIVS_DE_UPLANE_STRIDE_STRIDE__MASK }
pub const VIVS_DE_VPLANE_ADDRESS: u32 = 0x0000128c;
pub const VIVS_DE_VPLANE_STRIDE: u32 = 0x00001290;
pub const VIVS_DE_VPLANE_STRIDE_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_VPLANE_STRIDE_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_VPLANE_STRIDE_STRIDE(x: u32) -> u32 { (x << VIVS_DE_VPLANE_STRIDE_STRIDE__SHIFT) & VIVS_DE_VPLANE_STRIDE_STRIDE__MASK }
pub const VIVS_DE_VR_CONFIG: u32 = 0x00001294;
pub const VIVS_DE_VR_CONFIG_START__MASK: u32 = 0x00000003;
pub const VIVS_DE_VR_CONFIG_START__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_VR_CONFIG_START_HORIZONTAL_BLIT: u32 = 0x00000000;
pub const VIVS_DE_VR_CONFIG_START_VERTICAL_BLIT: u32 = 0x00000001;
pub const VIVS_DE_VR_CONFIG_START_ONE_PASS_BLIT: u32 = 0x00000002;
pub const VIVS_DE_VR_CONFIG_START_MASK: u32 = 0x00000008;
pub const VIVS_DE_VR_SOURCE_IMAGE_LOW: u32 = 0x00001298;
pub const VIVS_DE_VR_SOURCE_IMAGE_LOW_LEFT__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_VR_SOURCE_IMAGE_LOW_LEFT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_VR_SOURCE_IMAGE_LOW_LEFT(x: u32) -> u32 { (x << VIVS_DE_VR_SOURCE_IMAGE_LOW_LEFT__SHIFT) & VIVS_DE_VR_SOURCE_IMAGE_LOW_LEFT__MASK }
pub const VIVS_DE_VR_SOURCE_IMAGE_LOW_TOP__MASK: u32 = 0xffff0000;
pub const VIVS_DE_VR_SOURCE_IMAGE_LOW_TOP__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_VR_SOURCE_IMAGE_LOW_TOP(x: u32) -> u32 { (x << VIVS_DE_VR_SOURCE_IMAGE_LOW_TOP__SHIFT) & VIVS_DE_VR_SOURCE_IMAGE_LOW_TOP__MASK }
pub const VIVS_DE_VR_SOURCE_IMAGE_HIGH: u32 = 0x0000129c;
pub const VIVS_DE_VR_SOURCE_IMAGE_HIGH_RIGHT__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_VR_SOURCE_IMAGE_HIGH_RIGHT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_VR_SOURCE_IMAGE_HIGH_RIGHT(x: u32) -> u32 { (x << VIVS_DE_VR_SOURCE_IMAGE_HIGH_RIGHT__SHIFT) & VIVS_DE_VR_SOURCE_IMAGE_HIGH_RIGHT__MASK }
pub const VIVS_DE_VR_SOURCE_IMAGE_HIGH_BOTTOM__MASK: u32 = 0xffff0000;
pub const VIVS_DE_VR_SOURCE_IMAGE_HIGH_BOTTOM__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_VR_SOURCE_IMAGE_HIGH_BOTTOM(x: u32) -> u32 { (x << VIVS_DE_VR_SOURCE_IMAGE_HIGH_BOTTOM__SHIFT) & VIVS_DE_VR_SOURCE_IMAGE_HIGH_BOTTOM__MASK }
pub const VIVS_DE_VR_SOURCE_ORIGIN_LOW: u32 = 0x000012a0;
pub const VIVS_DE_VR_SOURCE_ORIGIN_LOW_X__MASK: u32 = 0xffffffff;
pub const VIVS_DE_VR_SOURCE_ORIGIN_LOW_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_VR_SOURCE_ORIGIN_LOW_X(x: u32) -> u32 { (x << VIVS_DE_VR_SOURCE_ORIGIN_LOW_X__SHIFT) & VIVS_DE_VR_SOURCE_ORIGIN_LOW_X__MASK }
pub const VIVS_DE_VR_SOURCE_ORIGIN_HIGH: u32 = 0x000012a4;
pub const VIVS_DE_VR_SOURCE_ORIGIN_HIGH_Y__MASK: u32 = 0xffffffff;
pub const VIVS_DE_VR_SOURCE_ORIGIN_HIGH_Y__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_VR_SOURCE_ORIGIN_HIGH_Y(x: u32) -> u32 { (x << VIVS_DE_VR_SOURCE_ORIGIN_HIGH_Y__SHIFT) & VIVS_DE_VR_SOURCE_ORIGIN_HIGH_Y__MASK }
pub const VIVS_DE_VR_TARGET_WINDOW_LOW: u32 = 0x000012a8;
pub const VIVS_DE_VR_TARGET_WINDOW_LOW_LEFT__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_VR_TARGET_WINDOW_LOW_LEFT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_VR_TARGET_WINDOW_LOW_LEFT(x: u32) -> u32 { (x << VIVS_DE_VR_TARGET_WINDOW_LOW_LEFT__SHIFT) & VIVS_DE_VR_TARGET_WINDOW_LOW_LEFT__MASK }
pub const VIVS_DE_VR_TARGET_WINDOW_LOW_TOP__MASK: u32 = 0xffff0000;
pub const VIVS_DE_VR_TARGET_WINDOW_LOW_TOP__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_VR_TARGET_WINDOW_LOW_TOP(x: u32) -> u32 { (x << VIVS_DE_VR_TARGET_WINDOW_LOW_TOP__SHIFT) & VIVS_DE_VR_TARGET_WINDOW_LOW_TOP__MASK }
pub const VIVS_DE_VR_TARGET_WINDOW_HIGH: u32 = 0x000012ac;
pub const VIVS_DE_VR_TARGET_WINDOW_HIGH_RIGHT__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_VR_TARGET_WINDOW_HIGH_RIGHT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_VR_TARGET_WINDOW_HIGH_RIGHT(x: u32) -> u32 { (x << VIVS_DE_VR_TARGET_WINDOW_HIGH_RIGHT__SHIFT) & VIVS_DE_VR_TARGET_WINDOW_HIGH_RIGHT__MASK }
pub const VIVS_DE_VR_TARGET_WINDOW_HIGH_BOTTOM__MASK: u32 = 0xffff0000;
pub const VIVS_DE_VR_TARGET_WINDOW_HIGH_BOTTOM__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_VR_TARGET_WINDOW_HIGH_BOTTOM(x: u32) -> u32 { (x << VIVS_DE_VR_TARGET_WINDOW_HIGH_BOTTOM__SHIFT) & VIVS_DE_VR_TARGET_WINDOW_HIGH_BOTTOM__MASK }
pub const VIVS_DE_PE_CONFIG: u32 = 0x000012b0;
pub const VIVS_DE_PE_CONFIG_DESTINATION_FETCH__MASK: u32 = 0x00000003;
pub const VIVS_DE_PE_CONFIG_DESTINATION_FETCH__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_PE_CONFIG_DESTINATION_FETCH_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_PE_CONFIG_DESTINATION_FETCH_DEFAULT: u32 = 0x00000001;
pub const VIVS_DE_PE_CONFIG_DESTINATION_FETCH_ALWAYS: u32 = 0x00000002;
pub const VIVS_DE_PE_CONFIG_DESTINATION_FETCH_MASK: u32 = 0x00000008;
pub const VIVS_DE_DEST_ROTATION_HEIGHT: u32 = 0x000012b4;
pub const VIVS_DE_DEST_ROTATION_HEIGHT_HEIGHT__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_DEST_ROTATION_HEIGHT_HEIGHT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_DEST_ROTATION_HEIGHT_HEIGHT(x: u32) -> u32 { (x << VIVS_DE_DEST_ROTATION_HEIGHT_HEIGHT__SHIFT) & VIVS_DE_DEST_ROTATION_HEIGHT_HEIGHT__MASK }
pub const VIVS_DE_SRC_ROTATION_HEIGHT: u32 = 0x000012b8;
pub const VIVS_DE_SRC_ROTATION_HEIGHT_HEIGHT__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_SRC_ROTATION_HEIGHT_HEIGHT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_SRC_ROTATION_HEIGHT_HEIGHT(x: u32) -> u32 { (x << VIVS_DE_SRC_ROTATION_HEIGHT_HEIGHT__SHIFT) & VIVS_DE_SRC_ROTATION_HEIGHT_HEIGHT__MASK }
pub const VIVS_DE_ROT_ANGLE: u32 = 0x000012bc;
pub const VIVS_DE_ROT_ANGLE_SRC__MASK: u32 = 0x00000007;
pub const VIVS_DE_ROT_ANGLE_SRC__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_ROT_ANGLE_SRC(x: u32) -> u32 { (x << VIVS_DE_ROT_ANGLE_SRC__SHIFT) & VIVS_DE_ROT_ANGLE_SRC__MASK }
pub const VIVS_DE_ROT_ANGLE_DST__MASK: u32 = 0x00000038;
pub const VIVS_DE_ROT_ANGLE_DST__SHIFT: u32 = 0x00000003;
pub const fn VIVS_DE_ROT_ANGLE_DST(x: u32) -> u32 { (x << VIVS_DE_ROT_ANGLE_DST__SHIFT) & VIVS_DE_ROT_ANGLE_DST__MASK }
pub const VIVS_DE_ROT_ANGLE_SRC_MASK: u32 = 0x00000100;
pub const VIVS_DE_ROT_ANGLE_DST_MASK: u32 = 0x00000200;
pub const VIVS_DE_ROT_ANGLE_SRC_MIRROR__MASK: u32 = 0x00003000;
pub const VIVS_DE_ROT_ANGLE_SRC_MIRROR__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_DE_ROT_ANGLE_SRC_MIRROR(x: u32) -> u32 { (x << VIVS_DE_ROT_ANGLE_SRC_MIRROR__SHIFT) & VIVS_DE_ROT_ANGLE_SRC_MIRROR__MASK }
pub const VIVS_DE_ROT_ANGLE_SRC_MIRROR_MASK: u32 = 0x00008000;
pub const VIVS_DE_ROT_ANGLE_DST_MIRROR__MASK: u32 = 0x00030000;
pub const VIVS_DE_ROT_ANGLE_DST_MIRROR__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_ROT_ANGLE_DST_MIRROR(x: u32) -> u32 { (x << VIVS_DE_ROT_ANGLE_DST_MIRROR__SHIFT) & VIVS_DE_ROT_ANGLE_DST_MIRROR__MASK }
pub const VIVS_DE_ROT_ANGLE_DST_MIRROR_MASK: u32 = 0x00080000;
pub const VIVS_DE_CLEAR_PIXEL_VALUE32: u32 = 0x000012c0;
pub const VIVS_DE_DEST_COLOR_KEY: u32 = 0x000012c4;
pub const VIVS_DE_GLOBAL_SRC_COLOR: u32 = 0x000012c8;
pub const VIVS_DE_GLOBAL_DEST_COLOR: u32 = 0x000012cc;
pub const VIVS_DE_COLOR_MULTIPLY_MODES: u32 = 0x000012d0;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY__MASK: u32 = 0x00000001;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY_ENABLE: u32 = 0x00000001;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY__MASK: u32 = 0x00000010;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY_ENABLE: u32 = 0x00000010;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY__MASK: u32 = 0x00000300;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_ALPHA: u32 = 0x00000100;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_COLOR: u32 = 0x00000200;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY__MASK: u32 = 0x00100000;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY_ENABLE: u32 = 0x00100000;
pub const VIVS_DE_PE_TRANSPARENCY: u32 = 0x000012d4;
pub const VIVS_DE_PE_TRANSPARENCY_SOURCE__MASK: u32 = 0x00000003;
pub const VIVS_DE_PE_TRANSPARENCY_SOURCE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_PE_TRANSPARENCY_SOURCE_OPAQUE: u32 = 0x00000000;
pub const VIVS_DE_PE_TRANSPARENCY_SOURCE_MASK: u32 = 0x00000001;
pub const VIVS_DE_PE_TRANSPARENCY_SOURCE_KEY: u32 = 0x00000002;
pub const VIVS_DE_PE_TRANSPARENCY_PATTERN__MASK: u32 = 0x00000030;
pub const VIVS_DE_PE_TRANSPARENCY_PATTERN__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_PE_TRANSPARENCY_PATTERN_OPAQUE: u32 = 0x00000000;
pub const VIVS_DE_PE_TRANSPARENCY_PATTERN_MASK: u32 = 0x00000010;
pub const VIVS_DE_PE_TRANSPARENCY_PATTERN_KEY: u32 = 0x00000020;
pub const VIVS_DE_PE_TRANSPARENCY_DESTINATION__MASK: u32 = 0x00000300;
pub const VIVS_DE_PE_TRANSPARENCY_DESTINATION__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_PE_TRANSPARENCY_DESTINATION_OPAQUE: u32 = 0x00000000;
pub const VIVS_DE_PE_TRANSPARENCY_DESTINATION_MASK: u32 = 0x00000100;
pub const VIVS_DE_PE_TRANSPARENCY_DESTINATION_KEY: u32 = 0x00000200;
pub const VIVS_DE_PE_TRANSPARENCY_TRANSPARENCY_MASK: u32 = 0x00001000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_SRC_OVERRIDE__MASK: u32 = 0x00030000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_SRC_OVERRIDE__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_PE_TRANSPARENCY_USE_SRC_OVERRIDE_DEFAULT: u32 = 0x00000000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_SRC_OVERRIDE_USE_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_SRC_OVERRIDE_USE_DISABLE: u32 = 0x00020000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_PAT_OVERRIDE__MASK: u32 = 0x00300000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_PAT_OVERRIDE__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_PE_TRANSPARENCY_USE_PAT_OVERRIDE_DEFAULT: u32 = 0x00000000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_PAT_OVERRIDE_USE_ENABLE: u32 = 0x00100000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_PAT_OVERRIDE_USE_DISABLE: u32 = 0x00200000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_DST_OVERRIDE__MASK: u32 = 0x03000000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_DST_OVERRIDE__SHIFT: u32 = 0x00000018;
pub const VIVS_DE_PE_TRANSPARENCY_USE_DST_OVERRIDE_DEFAULT: u32 = 0x00000000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_DST_OVERRIDE_USE_ENABLE: u32 = 0x01000000;
pub const VIVS_DE_PE_TRANSPARENCY_USE_DST_OVERRIDE_USE_DISABLE: u32 = 0x02000000;
pub const VIVS_DE_PE_TRANSPARENCY_RESOURCE_OVERRIDE_MASK: u32 = 0x10000000;
pub const VIVS_DE_PE_TRANSPARENCY_DFB_COLOR_KEY__MASK: u32 = 0x20000000;
pub const VIVS_DE_PE_TRANSPARENCY_DFB_COLOR_KEY__SHIFT: u32 = 0x0000001d;
pub const VIVS_DE_PE_TRANSPARENCY_DFB_COLOR_KEY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_PE_TRANSPARENCY_DFB_COLOR_KEY_ENABLE: u32 = 0x20000000;
pub const VIVS_DE_PE_TRANSPARENCY_DFB_COLOR_KEY_MASK: u32 = 0x80000000;
pub const VIVS_DE_PE_CONTROL: u32 = 0x000012d8;
pub const VIVS_DE_PE_CONTROL_YUV__MASK: u32 = 0x00000001;
pub const VIVS_DE_PE_CONTROL_YUV__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_PE_CONTROL_YUV_601: u32 = 0x00000000;
pub const VIVS_DE_PE_CONTROL_YUV_709: u32 = 0x00000001;
pub const VIVS_DE_PE_CONTROL_YUV_MASK: u32 = 0x00000008;
pub const VIVS_DE_PE_CONTROL_UV_SWIZZLE__MASK: u32 = 0x00000010;
pub const VIVS_DE_PE_CONTROL_UV_SWIZZLE__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_PE_CONTROL_UV_SWIZZLE_UV: u32 = 0x00000000;
pub const VIVS_DE_PE_CONTROL_UV_SWIZZLE_VU: u32 = 0x00000010;
pub const VIVS_DE_PE_CONTROL_UV_SWIZZLE_MASK: u32 = 0x00000080;
pub const VIVS_DE_PE_CONTROL_YUVRGB__MASK: u32 = 0x00000100;
pub const VIVS_DE_PE_CONTROL_YUVRGB__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_PE_CONTROL_YUVRGB_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_PE_CONTROL_YUVRGB_ENABLE: u32 = 0x00000100;
pub const VIVS_DE_PE_CONTROL_YUVRGB_MASK: u32 = 0x00000800;
pub const VIVS_DE_SRC_COLOR_KEY_HIGH: u32 = 0x000012dc;
pub const VIVS_DE_DEST_COLOR_KEY_HIGH: u32 = 0x000012e0;
pub const VIVS_DE_VR_CONFIG_EX: u32 = 0x000012e4;
pub const VIVS_DE_VR_CONFIG_EX_VERTICAL_LINE_WIDTH__MASK: u32 = 0x00000003;
pub const VIVS_DE_VR_CONFIG_EX_VERTICAL_LINE_WIDTH__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_VR_CONFIG_EX_VERTICAL_LINE_WIDTH_AUTO: u32 = 0x00000000;
pub const VIVS_DE_VR_CONFIG_EX_VERTICAL_LINE_WIDTH_PIXELS16: u32 = 0x00000001;
pub const VIVS_DE_VR_CONFIG_EX_VERTICAL_LINE_WIDTH_PIXELS32: u32 = 0x00000002;
pub const VIVS_DE_VR_CONFIG_EX_VERTICAL_LINE_WIDTH_MASK: u32 = 0x00000008;
pub const VIVS_DE_VR_CONFIG_EX_FILTER_TAP__MASK: u32 = 0x000000f0;
pub const VIVS_DE_VR_CONFIG_EX_FILTER_TAP__SHIFT: u32 = 0x00000004;
pub const fn VIVS_DE_VR_CONFIG_EX_FILTER_TAP(x: u32) -> u32 { (x << VIVS_DE_VR_CONFIG_EX_FILTER_TAP__SHIFT) & VIVS_DE_VR_CONFIG_EX_FILTER_TAP__MASK }
pub const VIVS_DE_VR_CONFIG_EX_FILTER_TAP_MASK: u32 = 0x00000100;
pub const VIVS_DE_PE_DITHER_LOW: u32 = 0x000012e8;
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y0__MASK: u32 = 0x0000000f;
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y0__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y0(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y0__SHIFT) & VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y0__MASK }
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y0__MASK: u32 = 0x000000f0;
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y0__SHIFT: u32 = 0x00000004;
pub const fn VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y0(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y0__SHIFT) & VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y0__MASK }
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y0__MASK: u32 = 0x00000f00;
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y0__SHIFT: u32 = 0x00000008;
pub const fn VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y0(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y0__SHIFT) & VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y0__MASK }
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y0__MASK: u32 = 0x0000f000;
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y0__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y0(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y0__SHIFT) & VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y0__MASK }
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y1__MASK: u32 = 0x000f0000;
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y1__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y1(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y1__SHIFT) & VIVS_DE_PE_DITHER_LOW_PIXEL_X0_Y1__MASK }
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y1__MASK: u32 = 0x00f00000;
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y1__SHIFT: u32 = 0x00000014;
pub const fn VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y1(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y1__SHIFT) & VIVS_DE_PE_DITHER_LOW_PIXEL_X1_Y1__MASK }
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y1__MASK: u32 = 0x0f000000;
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y1__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y1(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y1__SHIFT) & VIVS_DE_PE_DITHER_LOW_PIXEL_X2_Y1__MASK }
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y1__MASK: u32 = 0xf0000000;
pub const VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y1__SHIFT: u32 = 0x0000001c;
pub const fn VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y1(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y1__SHIFT) & VIVS_DE_PE_DITHER_LOW_PIXEL_X3_Y1__MASK }
pub const VIVS_DE_PE_DITHER_HIGH: u32 = 0x000012ec;
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y2__MASK: u32 = 0x0000000f;
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y2__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y2(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y2__SHIFT) & VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y2__MASK }
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y2__MASK: u32 = 0x000000f0;
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y2__SHIFT: u32 = 0x00000004;
pub const fn VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y2(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y2__SHIFT) & VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y2__MASK }
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y2__MASK: u32 = 0x00000f00;
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y2__SHIFT: u32 = 0x00000008;
pub const fn VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y2(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y2__SHIFT) & VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y2__MASK }
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y2__MASK: u32 = 0x0000f000;
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y2__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y2(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y2__SHIFT) & VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y2__MASK }
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y3__MASK: u32 = 0x000f0000;
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y3__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y3(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y3__SHIFT) & VIVS_DE_PE_DITHER_HIGH_PIXEL_X0_Y3__MASK }
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y3__MASK: u32 = 0x00f00000;
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y3__SHIFT: u32 = 0x00000014;
pub const fn VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y3(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y3__SHIFT) & VIVS_DE_PE_DITHER_HIGH_PIXEL_X1_Y3__MASK }
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y3__MASK: u32 = 0x0f000000;
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y3__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y3(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y3__SHIFT) & VIVS_DE_PE_DITHER_HIGH_PIXEL_X2_Y3__MASK }
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y3__MASK: u32 = 0xf0000000;
pub const VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y3__SHIFT: u32 = 0x0000001c;
pub const fn VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y3(x: u32) -> u32 { (x << VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y3__SHIFT) & VIVS_DE_PE_DITHER_HIGH_PIXEL_X3_Y3__MASK }
pub const VIVS_DE_BW_CONFIG: u32 = 0x000012f0;
pub const VIVS_DE_BW_CONFIG_BLOCK_CONFIG__MASK: u32 = 0x00000001;
pub const VIVS_DE_BW_CONFIG_BLOCK_CONFIG__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BW_CONFIG_BLOCK_CONFIG_AUTO: u32 = 0x00000000;
pub const VIVS_DE_BW_CONFIG_BLOCK_CONFIG_CUSTOMIZE: u32 = 0x00000001;
pub const VIVS_DE_BW_CONFIG_BLOCK_CONFIG_MASK: u32 = 0x00000008;
pub const VIVS_DE_BW_CONFIG_BLOCK_WALK_DIRECTION__MASK: u32 = 0x00000010;
pub const VIVS_DE_BW_CONFIG_BLOCK_WALK_DIRECTION__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_BW_CONFIG_BLOCK_WALK_DIRECTION_RIGHT_BOTTOM: u32 = 0x00000000;
pub const VIVS_DE_BW_CONFIG_BLOCK_WALK_DIRECTION_BOTTOM_RIGHT: u32 = 0x00000010;
pub const VIVS_DE_BW_CONFIG_BLOCK_WALK_DIRECTION_MASK: u32 = 0x00000080;
pub const VIVS_DE_BW_CONFIG_TILE_WALK_DIRECTION__MASK: u32 = 0x00000100;
pub const VIVS_DE_BW_CONFIG_TILE_WALK_DIRECTION__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BW_CONFIG_TILE_WALK_DIRECTION_RIGHT_BOTTOM: u32 = 0x00000000;
pub const VIVS_DE_BW_CONFIG_TILE_WALK_DIRECTION_BOTTOM_RIGHT: u32 = 0x00000100;
pub const VIVS_DE_BW_CONFIG_TILE_WALK_DIRECTION_MASK: u32 = 0x00000800;
pub const VIVS_DE_BW_CONFIG_PIXEL_WALK_DIRECTION__MASK: u32 = 0x00001000;
pub const VIVS_DE_BW_CONFIG_PIXEL_WALK_DIRECTION__SHIFT: u32 = 0x0000000c;
pub const VIVS_DE_BW_CONFIG_PIXEL_WALK_DIRECTION_RIGHT_BOTTOM: u32 = 0x00000000;
pub const VIVS_DE_BW_CONFIG_PIXEL_WALK_DIRECTION_BOTTOM_RIGHT: u32 = 0x00001000;
pub const VIVS_DE_BW_CONFIG_PIXEL_WALK_DIRECTION_MASK: u32 = 0x00008000;
pub const VIVS_DE_BW_BLOCK_SIZE: u32 = 0x000012f4;
pub const VIVS_DE_BW_BLOCK_SIZE_WIDTH__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BW_BLOCK_SIZE_WIDTH__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BW_BLOCK_SIZE_WIDTH(x: u32) -> u32 { (x << VIVS_DE_BW_BLOCK_SIZE_WIDTH__SHIFT) & VIVS_DE_BW_BLOCK_SIZE_WIDTH__MASK }
pub const VIVS_DE_BW_BLOCK_SIZE_HEIGHT__MASK: u32 = 0xffff0000;
pub const VIVS_DE_BW_BLOCK_SIZE_HEIGHT__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BW_BLOCK_SIZE_HEIGHT(x: u32) -> u32 { (x << VIVS_DE_BW_BLOCK_SIZE_HEIGHT__SHIFT) & VIVS_DE_BW_BLOCK_SIZE_HEIGHT__MASK }
pub const VIVS_DE_BW_TILE_SIZE: u32 = 0x000012f8;
pub const VIVS_DE_BW_TILE_SIZE_WIDTH__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BW_TILE_SIZE_WIDTH__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BW_TILE_SIZE_WIDTH(x: u32) -> u32 { (x << VIVS_DE_BW_TILE_SIZE_WIDTH__SHIFT) & VIVS_DE_BW_TILE_SIZE_WIDTH__MASK }
pub const VIVS_DE_BW_TILE_SIZE_HEIGHT__MASK: u32 = 0xffff0000;
pub const VIVS_DE_BW_TILE_SIZE_HEIGHT__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BW_TILE_SIZE_HEIGHT(x: u32) -> u32 { (x << VIVS_DE_BW_TILE_SIZE_HEIGHT__SHIFT) & VIVS_DE_BW_TILE_SIZE_HEIGHT__MASK }
pub const VIVS_DE_BW_BLOCK_MASK: u32 = 0x000012fc;
pub const VIVS_DE_BW_BLOCK_MASK_HORIZONTAL__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BW_BLOCK_MASK_HORIZONTAL__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BW_BLOCK_MASK_HORIZONTAL(x: u32) -> u32 { (x << VIVS_DE_BW_BLOCK_MASK_HORIZONTAL__SHIFT) & VIVS_DE_BW_BLOCK_MASK_HORIZONTAL__MASK }
pub const VIVS_DE_BW_BLOCK_MASK_VERTICAL__MASK: u32 = 0xffff0000;
pub const VIVS_DE_BW_BLOCK_MASK_VERTICAL__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BW_BLOCK_MASK_VERTICAL(x: u32) -> u32 { (x << VIVS_DE_BW_BLOCK_MASK_VERTICAL__SHIFT) & VIVS_DE_BW_BLOCK_MASK_VERTICAL__MASK }
pub const VIVS_DE_SRC_EX_CONFIG: u32 = 0x00001300;
pub const VIVS_DE_SRC_EX_CONFIG_MULTI_TILED__MASK: u32 = 0x00000001;
pub const VIVS_DE_SRC_EX_CONFIG_MULTI_TILED__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_SRC_EX_CONFIG_MULTI_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_SRC_EX_CONFIG_MULTI_TILED_ENABLE: u32 = 0x00000001;
pub const VIVS_DE_SRC_EX_CONFIG_SUPER_TILED__MASK: u32 = 0x00000008;
pub const VIVS_DE_SRC_EX_CONFIG_SUPER_TILED__SHIFT: u32 = 0x00000003;
pub const VIVS_DE_SRC_EX_CONFIG_SUPER_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_SRC_EX_CONFIG_SUPER_TILED_ENABLE: u32 = 0x00000008;
pub const VIVS_DE_SRC_EX_CONFIG_MINOR_TILED__MASK: u32 = 0x00000100;
pub const VIVS_DE_SRC_EX_CONFIG_MINOR_TILED__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_SRC_EX_CONFIG_MINOR_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_SRC_EX_CONFIG_MINOR_TILED_ENABLE: u32 = 0x00000100;
pub const VIVS_DE_SRC_EX_ADDRESS: u32 = 0x00001304;
pub const VIVS_DE_DE_MULTI_SOURCE: u32 = 0x00001308;
pub const VIVS_DE_DE_MULTI_SOURCE_MAX_SOURCE__MASK: u32 = 0x00000007;
pub const VIVS_DE_DE_MULTI_SOURCE_MAX_SOURCE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_DE_MULTI_SOURCE_MAX_SOURCE(x: u32) -> u32 { (x << VIVS_DE_DE_MULTI_SOURCE_MAX_SOURCE__SHIFT) & VIVS_DE_DE_MULTI_SOURCE_MAX_SOURCE__MASK }
pub const VIVS_DE_DE_MULTI_SOURCE_HORIZONTAL_BLOCK__MASK: u32 = 0x00000700;
pub const VIVS_DE_DE_MULTI_SOURCE_HORIZONTAL_BLOCK__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_DE_MULTI_SOURCE_HORIZONTAL_BLOCK_PIXEL16: u32 = 0x00000000;
pub const VIVS_DE_DE_MULTI_SOURCE_HORIZONTAL_BLOCK_PIXEL32: u32 = 0x00000100;
pub const VIVS_DE_DE_MULTI_SOURCE_HORIZONTAL_BLOCK_PIXEL64: u32 = 0x00000200;
pub const VIVS_DE_DE_MULTI_SOURCE_HORIZONTAL_BLOCK_PIXEL128: u32 = 0x00000300;
pub const VIVS_DE_DE_MULTI_SOURCE_HORIZONTAL_BLOCK_PIXEL256: u32 = 0x00000400;
pub const VIVS_DE_DE_MULTI_SOURCE_HORIZONTAL_BLOCK_PIXEL512: u32 = 0x00000500;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK__MASK: u32 = 0x00070000;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK_LINE1: u32 = 0x00000000;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK_LINE2: u32 = 0x00010000;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK_LINE4: u32 = 0x00020000;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK_LINE8: u32 = 0x00030000;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK_LINE16: u32 = 0x00040000;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK_LINE32: u32 = 0x00050000;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK_LINE64: u32 = 0x00060000;
pub const VIVS_DE_DE_MULTI_SOURCE_VERTICAL_BLOCK_LINE128: u32 = 0x00070000;
pub const VIVS_DE_DEYUV_CONVERSION: u32 = 0x0000130c;
pub const VIVS_DE_DEYUV_CONVERSION_ENABLE__MASK: u32 = 0x00000003;
pub const VIVS_DE_DEYUV_CONVERSION_ENABLE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_DEYUV_CONVERSION_ENABLE_OFF: u32 = 0x00000000;
pub const VIVS_DE_DEYUV_CONVERSION_ENABLE_PLANE1: u32 = 0x00000001;
pub const VIVS_DE_DEYUV_CONVERSION_ENABLE_PLANE2: u32 = 0x00000002;
pub const VIVS_DE_DEYUV_CONVERSION_ENABLE_PLANE3: u32 = 0x00000003;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_COUNT__MASK: u32 = 0x0000000c;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_COUNT__SHIFT: u32 = 0x00000002;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE1_COUNT(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE1_COUNT__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE1_COUNT__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_COUNT__MASK: u32 = 0x00000030;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_COUNT__SHIFT: u32 = 0x00000004;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE2_COUNT(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE2_COUNT__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE2_COUNT__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_COUNT__MASK: u32 = 0x000000c0;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_COUNT__SHIFT: u32 = 0x00000006;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE3_COUNT(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE3_COUNT__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE3_COUNT__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_B__MASK: u32 = 0x00000300;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_B__SHIFT: u32 = 0x00000008;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_B(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_B__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_B__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_G__MASK: u32 = 0x00000c00;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_G__SHIFT: u32 = 0x0000000a;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_G(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_G__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_G__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_R__MASK: u32 = 0x00003000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_R__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_R(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_R__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_R__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_A__MASK: u32 = 0x0000c000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_A__SHIFT: u32 = 0x0000000e;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_A(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_A__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE1_SWIZZLE_A__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_B__MASK: u32 = 0x00030000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_B__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_B(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_B__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_B__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_G__MASK: u32 = 0x000c0000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_G__SHIFT: u32 = 0x00000012;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_G(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_G__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_G__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_R__MASK: u32 = 0x00300000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_R__SHIFT: u32 = 0x00000014;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_R(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_R__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_R__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_A__MASK: u32 = 0x00c00000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_A__SHIFT: u32 = 0x00000016;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_A(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_A__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE2_SWIZZLE_A__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_B__MASK: u32 = 0x03000000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_B__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_B(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_B__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_B__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_G__MASK: u32 = 0x0c000000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_G__SHIFT: u32 = 0x0000001a;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_G(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_G__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_G__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_R__MASK: u32 = 0x30000000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_R__SHIFT: u32 = 0x0000001c;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_R(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_R__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_R__MASK }
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_A__MASK: u32 = 0xc0000000;
pub const VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_A__SHIFT: u32 = 0x0000001e;
pub const fn VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_A(x: u32) -> u32 { (x << VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_A__SHIFT) & VIVS_DE_DEYUV_CONVERSION_PLANE3_SWIZZLE_A__MASK }
pub const VIVS_DE_DE_PLANE2_ADDRESS: u32 = 0x00001310;
pub const VIVS_DE_DE_PLANE2_STRIDE: u32 = 0x00001314;
pub const VIVS_DE_DE_PLANE2_STRIDE_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_DE_PLANE2_STRIDE_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_DE_PLANE2_STRIDE_STRIDE(x: u32) -> u32 { (x << VIVS_DE_DE_PLANE2_STRIDE_STRIDE__SHIFT) & VIVS_DE_DE_PLANE2_STRIDE_STRIDE__MASK }
pub const VIVS_DE_DE_PLANE3_ADDRESS: u32 = 0x00001318;
pub const VIVS_DE_DE_PLANE3_STRIDE: u32 = 0x0000131c;
pub const VIVS_DE_DE_PLANE3_STRIDE_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_DE_PLANE3_STRIDE_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_DE_PLANE3_STRIDE_STRIDE(x: u32) -> u32 { (x << VIVS_DE_DE_PLANE3_STRIDE_STRIDE__SHIFT) & VIVS_DE_DE_PLANE3_STRIDE_STRIDE__MASK }
pub const VIVS_DE_DE_STALL_DE: u32 = 0x00001320;
pub const VIVS_DE_DE_STALL_DE_ENABLE__MASK: u32 = 0x00000001;
pub const VIVS_DE_DE_STALL_DE_ENABLE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_DE_STALL_DE_ENABLE_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_DE_STALL_DE_ENABLE_ENABLE: u32 = 0x00000001;
pub const VIVS_DE_TILE_CONFIG: u32 = 0x00001328;
pub const VIVS_DE_TILE_CONFIG_SUPERTILE_VERSION__MASK: u32 = 0x00000003;
pub const VIVS_DE_TILE_CONFIG_SUPERTILE_VERSION__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_TILE_CONFIG_SUPERTILE_VERSION(x: u32) -> u32 { (x << VIVS_DE_TILE_CONFIG_SUPERTILE_VERSION__SHIFT) & VIVS_DE_TILE_CONFIG_SUPERTILE_VERSION__MASK }
pub const VIVS_DE_FILTER_KERNEL__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_FILTER_KERNEL__LEN: u32 = 0x00000080;
pub const VIVS_DE_FILTER_KERNEL_COEFFICIENT0__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_FILTER_KERNEL_COEFFICIENT0__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_FILTER_KERNEL_COEFFICIENT0(x: u32) -> u32 { (x << VIVS_DE_FILTER_KERNEL_COEFFICIENT0__SHIFT) & VIVS_DE_FILTER_KERNEL_COEFFICIENT0__MASK }
pub const VIVS_DE_FILTER_KERNEL_COEFFICIENT1__MASK: u32 = 0xffff0000;
pub const VIVS_DE_FILTER_KERNEL_COEFFICIENT1__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_FILTER_KERNEL_COEFFICIENT1(x: u32) -> u32 { (x << VIVS_DE_FILTER_KERNEL_COEFFICIENT1__SHIFT) & VIVS_DE_FILTER_KERNEL_COEFFICIENT1__MASK }
pub const VIVS_DE_INDEX_COLOR_TABLE__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_INDEX_COLOR_TABLE__LEN: u32 = 0x00000100;
pub const VIVS_DE_HORI_FILTER_KERNEL__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_HORI_FILTER_KERNEL__LEN: u32 = 0x00000080;
pub const VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT0__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT0__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT0(x: u32) -> u32 { (x << VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT0__SHIFT) & VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT0__MASK }
pub const VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT1__MASK: u32 = 0xffff0000;
pub const VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT1__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT1(x: u32) -> u32 { (x << VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT1__SHIFT) & VIVS_DE_HORI_FILTER_KERNEL_COEFFICIENT1__MASK }
pub const VIVS_DE_VERTI_FILTER_KERNEL__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_VERTI_FILTER_KERNEL__LEN: u32 = 0x00000080;
pub const VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT0__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT0__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT0(x: u32) -> u32 { (x << VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT0__SHIFT) & VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT0__MASK }
pub const VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT1__MASK: u32 = 0xffff0000;
pub const VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT1__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT1(x: u32) -> u32 { (x << VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT1__SHIFT) & VIVS_DE_VERTI_FILTER_KERNEL_COEFFICIENT1__MASK }
pub const VIVS_DE_INDEX_COLOR_TABLE32__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_INDEX_COLOR_TABLE32__LEN: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_ADDRESS__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_ADDRESS__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_STRIDE__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_STRIDE__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_STRIDE_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_BLOCK4_SRC_STRIDE_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_SRC_STRIDE_STRIDE(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_STRIDE_STRIDE__SHIFT) & VIVS_DE_BLOCK4_SRC_STRIDE_STRIDE__MASK }
pub const VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG_WIDTH__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG_WIDTH__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG_WIDTH(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG_WIDTH__SHIFT) & VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG_WIDTH__MASK }
pub const VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG_ROTATION__MASK: u32 = 0x00010000;
pub const VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG_ROTATION__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG_ROTATION_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_ROTATION_CONFIG_ROTATION_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_CONFIG__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_PE10_SOURCE_FORMAT__MASK: u32 = 0x0000000f;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_PE10_SOURCE_FORMAT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_SRC_CONFIG_PE10_SOURCE_FORMAT(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_CONFIG_PE10_SOURCE_FORMAT__SHIFT) & VIVS_DE_BLOCK4_SRC_CONFIG_PE10_SOURCE_FORMAT__MASK }
pub const VIVS_DE_BLOCK4_SRC_CONFIG_TRANSPARENCY__MASK: u32 = 0x00000030;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_TRANSPARENCY__SHIFT: u32 = 0x00000004;
pub const fn VIVS_DE_BLOCK4_SRC_CONFIG_TRANSPARENCY(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_CONFIG_TRANSPARENCY__SHIFT) & VIVS_DE_BLOCK4_SRC_CONFIG_TRANSPARENCY__MASK }
pub const VIVS_DE_BLOCK4_SRC_CONFIG_SRC_RELATIVE__MASK: u32 = 0x00000040;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_SRC_RELATIVE__SHIFT: u32 = 0x00000006;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_SRC_RELATIVE_ABSOLUTE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_SRC_RELATIVE_RELATIVE: u32 = 0x00000040;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_TILED__MASK: u32 = 0x00000080;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_TILED__SHIFT: u32 = 0x00000007;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_TILED_ENABLE: u32 = 0x00000080;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_LOCATION__MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_LOCATION__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_LOCATION_MEMORY: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_LOCATION_STREAM: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_PACK__MASK: u32 = 0x00003000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_PACK__SHIFT: u32 = 0x0000000c;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_PACK_PACKED8: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_PACK_PACKED16: u32 = 0x00001000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_PACK_PACKED32: u32 = 0x00002000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_PACK_UNPACKED: u32 = 0x00003000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_MONO_TRANSPARENCY__MASK: u32 = 0x00008000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_MONO_TRANSPARENCY__SHIFT: u32 = 0x0000000f;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_MONO_TRANSPARENCY_BACKGROUND: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_MONO_TRANSPARENCY_FOREGROUND: u32 = 0x00008000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_UNK16: u32 = 0x00010000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_SWIZZLE__MASK: u32 = 0x00300000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_SWIZZLE__SHIFT: u32 = 0x00000014;
pub const fn VIVS_DE_BLOCK4_SRC_CONFIG_SWIZZLE(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_CONFIG_SWIZZLE__SHIFT) & VIVS_DE_BLOCK4_SRC_CONFIG_SWIZZLE__MASK }
pub const VIVS_DE_BLOCK4_SRC_CONFIG_SOURCE_FORMAT__MASK: u32 = 0x1f000000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_SOURCE_FORMAT__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_BLOCK4_SRC_CONFIG_SOURCE_FORMAT(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_CONFIG_SOURCE_FORMAT__SHIFT) & VIVS_DE_BLOCK4_SRC_CONFIG_SOURCE_FORMAT__MASK }
pub const VIVS_DE_BLOCK4_SRC_CONFIG_DISABLE420_L2_CACHE: u32 = 0x20000000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_ENDIAN_CONTROL__MASK: u32 = 0xc0000000;
pub const VIVS_DE_BLOCK4_SRC_CONFIG_ENDIAN_CONTROL__SHIFT: u32 = 0x0000001e;
pub const fn VIVS_DE_BLOCK4_SRC_CONFIG_ENDIAN_CONTROL(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_CONFIG_ENDIAN_CONTROL__SHIFT) & VIVS_DE_BLOCK4_SRC_CONFIG_ENDIAN_CONTROL__MASK }
pub const VIVS_DE_BLOCK4_SRC_ORIGIN__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_ORIGIN__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_ORIGIN_X__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BLOCK4_SRC_ORIGIN_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_SRC_ORIGIN_X(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_ORIGIN_X__SHIFT) & VIVS_DE_BLOCK4_SRC_ORIGIN_X__MASK }
pub const VIVS_DE_BLOCK4_SRC_ORIGIN_Y__MASK: u32 = 0xffff0000;
pub const VIVS_DE_BLOCK4_SRC_ORIGIN_Y__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BLOCK4_SRC_ORIGIN_Y(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_ORIGIN_Y__SHIFT) & VIVS_DE_BLOCK4_SRC_ORIGIN_Y__MASK }
pub const VIVS_DE_BLOCK4_SRC_SIZE__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_SIZE__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_SIZE_X__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BLOCK4_SRC_SIZE_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_SRC_SIZE_X(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_SIZE_X__SHIFT) & VIVS_DE_BLOCK4_SRC_SIZE_X__MASK }
pub const VIVS_DE_BLOCK4_SRC_SIZE_Y__MASK: u32 = 0xffff0000;
pub const VIVS_DE_BLOCK4_SRC_SIZE_Y__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BLOCK4_SRC_SIZE_Y(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_SIZE_Y__SHIFT) & VIVS_DE_BLOCK4_SRC_SIZE_Y__MASK }
pub const VIVS_DE_BLOCK4_SRC_COLOR_BG__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_COLOR_BG__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ROP__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ROP__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ROP_ROP_FG__MASK: u32 = 0x000000ff;
pub const VIVS_DE_BLOCK4_ROP_ROP_FG__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_ROP_ROP_FG(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ROP_ROP_FG__SHIFT) & VIVS_DE_BLOCK4_ROP_ROP_FG__MASK }
pub const VIVS_DE_BLOCK4_ROP_ROP_BG__MASK: u32 = 0x0000ff00;
pub const VIVS_DE_BLOCK4_ROP_ROP_BG__SHIFT: u32 = 0x00000008;
pub const fn VIVS_DE_BLOCK4_ROP_ROP_BG(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ROP_ROP_BG__SHIFT) & VIVS_DE_BLOCK4_ROP_ROP_BG__MASK }
pub const VIVS_DE_BLOCK4_ROP_TYPE__MASK: u32 = 0x00300000;
pub const VIVS_DE_BLOCK4_ROP_TYPE__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_BLOCK4_ROP_TYPE_ROP2_PATTERN: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ROP_TYPE_ROP2_SOURCE: u32 = 0x00100000;
pub const VIVS_DE_BLOCK4_ROP_TYPE_ROP3: u32 = 0x00200000;
pub const VIVS_DE_BLOCK4_ROP_TYPE_ROP4: u32 = 0x00300000;
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL_ENABLE__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL_ENABLE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL_ENABLE_OFF: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL_ENABLE_ON: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__MASK: u32 = 0x00ff0000;
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__SHIFT) & VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__MASK }
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__MASK: u32 = 0xff000000;
pub const VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__SHIFT) & VIVS_DE_BLOCK4_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__MASK }
pub const VIVS_DE_BLOCK4_ALPHA_MODES__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ALPHA_MODES__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_ALPHA_MODE__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_ALPHA_MODE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_ALPHA_MODE_INVERSED: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_ALPHA_MODE__MASK: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_ALPHA_MODE__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_ALPHA_MODE_INVERSED: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE__MASK: u32 = 0x00000300;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_GLOBAL: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_SCALED: u32 = 0x00000200;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE__MASK: u32 = 0x00003000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE__SHIFT: u32 = 0x0000000c;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE_GLOBAL: u32 = 0x00001000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE_SCALED: u32 = 0x00002000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY__MASK: u32 = 0x00010000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY__MASK: u32 = 0x00100000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY_ENABLE: u32 = 0x00100000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_BLENDING_MODE__MASK: u32 = 0x07000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_BLENDING_MODE__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_BLOCK4_ALPHA_MODES_SRC_BLENDING_MODE(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ALPHA_MODES_SRC_BLENDING_MODE__SHIFT) & VIVS_DE_BLOCK4_ALPHA_MODES_SRC_BLENDING_MODE__MASK }
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_ALPHA_FACTOR__MASK: u32 = 0x08000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_ALPHA_FACTOR__SHIFT: u32 = 0x0000001b;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_ALPHA_FACTOR_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_SRC_ALPHA_FACTOR_ENABLE: u32 = 0x08000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_BLENDING_MODE__MASK: u32 = 0x70000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_BLENDING_MODE__SHIFT: u32 = 0x0000001c;
pub const fn VIVS_DE_BLOCK4_ALPHA_MODES_DST_BLENDING_MODE(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ALPHA_MODES_DST_BLENDING_MODE__SHIFT) & VIVS_DE_BLOCK4_ALPHA_MODES_DST_BLENDING_MODE__MASK }
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_ALPHA_FACTOR__MASK: u32 = 0x80000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_ALPHA_FACTOR__SHIFT: u32 = 0x0000001f;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_ALPHA_FACTOR_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_ALPHA_MODES_DST_ALPHA_FACTOR_ENABLE: u32 = 0x80000000;
pub const VIVS_DE_BLOCK4_ADDRESS_U__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ADDRESS_U__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_STRIDE_U__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_STRIDE_U__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_STRIDE_U_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_BLOCK4_STRIDE_U_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_STRIDE_U_STRIDE(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_STRIDE_U_STRIDE__SHIFT) & VIVS_DE_BLOCK4_STRIDE_U_STRIDE__MASK }
pub const VIVS_DE_BLOCK4_ADDRESS_V__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ADDRESS_V__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_STRIDE_V__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_STRIDE_V__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_STRIDE_V_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_BLOCK4_STRIDE_V_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_STRIDE_V_STRIDE(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_STRIDE_V_STRIDE__SHIFT) & VIVS_DE_BLOCK4_STRIDE_V_STRIDE__MASK }
pub const VIVS_DE_BLOCK4_SRC_ROTATION_HEIGHT__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_ROTATION_HEIGHT__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_ROTATION_HEIGHT_HEIGHT__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BLOCK4_SRC_ROTATION_HEIGHT_HEIGHT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_SRC_ROTATION_HEIGHT_HEIGHT(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_SRC_ROTATION_HEIGHT_HEIGHT__SHIFT) & VIVS_DE_BLOCK4_SRC_ROTATION_HEIGHT_HEIGHT__MASK }
pub const VIVS_DE_BLOCK4_ROT_ANGLE__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ROT_ANGLE__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_ROT_ANGLE_SRC__MASK: u32 = 0x00000007;
pub const VIVS_DE_BLOCK4_ROT_ANGLE_SRC__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK4_ROT_ANGLE_SRC(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ROT_ANGLE_SRC__SHIFT) & VIVS_DE_BLOCK4_ROT_ANGLE_SRC__MASK }
pub const VIVS_DE_BLOCK4_ROT_ANGLE_DST__MASK: u32 = 0x00000038;
pub const VIVS_DE_BLOCK4_ROT_ANGLE_DST__SHIFT: u32 = 0x00000003;
pub const fn VIVS_DE_BLOCK4_ROT_ANGLE_DST(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ROT_ANGLE_DST__SHIFT) & VIVS_DE_BLOCK4_ROT_ANGLE_DST__MASK }
pub const VIVS_DE_BLOCK4_ROT_ANGLE_SRC_MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_ROT_ANGLE_DST_MASK: u32 = 0x00000200;
pub const VIVS_DE_BLOCK4_ROT_ANGLE_SRC_MIRROR__MASK: u32 = 0x00003000;
pub const VIVS_DE_BLOCK4_ROT_ANGLE_SRC_MIRROR__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_DE_BLOCK4_ROT_ANGLE_SRC_MIRROR(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ROT_ANGLE_SRC_MIRROR__SHIFT) & VIVS_DE_BLOCK4_ROT_ANGLE_SRC_MIRROR__MASK }
pub const VIVS_DE_BLOCK4_ROT_ANGLE_SRC_MIRROR_MASK: u32 = 0x00008000;
pub const VIVS_DE_BLOCK4_ROT_ANGLE_DST_MIRROR__MASK: u32 = 0x00030000;
pub const VIVS_DE_BLOCK4_ROT_ANGLE_DST_MIRROR__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BLOCK4_ROT_ANGLE_DST_MIRROR(x: u32) -> u32 { (x << VIVS_DE_BLOCK4_ROT_ANGLE_DST_MIRROR__SHIFT) & VIVS_DE_BLOCK4_ROT_ANGLE_DST_MIRROR__MASK }
pub const VIVS_DE_BLOCK4_ROT_ANGLE_DST_MIRROR_MASK: u32 = 0x00080000;
pub const VIVS_DE_BLOCK4_GLOBAL_SRC_COLOR__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_GLOBAL_SRC_COLOR__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_GLOBAL_DEST_COLOR__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_GLOBAL_DEST_COLOR__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY_ENABLE: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY__MASK: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY_ENABLE: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY__MASK: u32 = 0x00000300;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_ALPHA: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_COLOR: u32 = 0x00000200;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY__MASK: u32 = 0x00100000;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY_ENABLE: u32 = 0x00100000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_TRANSPARENCY__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_SOURCE__MASK: u32 = 0x00000003;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_SOURCE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_SOURCE_OPAQUE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_SOURCE_MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_SOURCE_KEY: u32 = 0x00000002;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_PATTERN__MASK: u32 = 0x00000030;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_PATTERN__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_PATTERN_OPAQUE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_PATTERN_MASK: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_PATTERN_KEY: u32 = 0x00000020;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DESTINATION__MASK: u32 = 0x00000300;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DESTINATION__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DESTINATION_OPAQUE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DESTINATION_MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DESTINATION_KEY: u32 = 0x00000200;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_TRANSPARENCY_MASK: u32 = 0x00001000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_SRC_OVERRIDE__MASK: u32 = 0x00030000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_SRC_OVERRIDE__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_SRC_OVERRIDE_DEFAULT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_SRC_OVERRIDE_USE_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_SRC_OVERRIDE_USE_DISABLE: u32 = 0x00020000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_PAT_OVERRIDE__MASK: u32 = 0x00300000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_PAT_OVERRIDE__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_PAT_OVERRIDE_DEFAULT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_PAT_OVERRIDE_USE_ENABLE: u32 = 0x00100000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_PAT_OVERRIDE_USE_DISABLE: u32 = 0x00200000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_DST_OVERRIDE__MASK: u32 = 0x03000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_DST_OVERRIDE__SHIFT: u32 = 0x00000018;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_DST_OVERRIDE_DEFAULT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_DST_OVERRIDE_USE_ENABLE: u32 = 0x01000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_USE_DST_OVERRIDE_USE_DISABLE: u32 = 0x02000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_RESOURCE_OVERRIDE_MASK: u32 = 0x10000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DFB_COLOR_KEY__MASK: u32 = 0x20000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DFB_COLOR_KEY__SHIFT: u32 = 0x0000001d;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DFB_COLOR_KEY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DFB_COLOR_KEY_ENABLE: u32 = 0x20000000;
pub const VIVS_DE_BLOCK4_TRANSPARENCY_DFB_COLOR_KEY_MASK: u32 = 0x80000000;
pub const VIVS_DE_BLOCK4_CONTROL__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_CONTROL__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_CONTROL_YUV__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_CONTROL_YUV__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_CONTROL_YUV_601: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_CONTROL_YUV_709: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_CONTROL_YUV_MASK: u32 = 0x00000008;
pub const VIVS_DE_BLOCK4_CONTROL_UV_SWIZZLE__MASK: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_CONTROL_UV_SWIZZLE__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_CONTROL_UV_SWIZZLE_UV: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_CONTROL_UV_SWIZZLE_VU: u32 = 0x00000010;
pub const VIVS_DE_BLOCK4_CONTROL_UV_SWIZZLE_MASK: u32 = 0x00000080;
pub const VIVS_DE_BLOCK4_CONTROL_YUVRGB__MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_CONTROL_YUVRGB__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK4_CONTROL_YUVRGB_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_CONTROL_YUVRGB_ENABLE: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_CONTROL_YUVRGB_MASK: u32 = 0x00000800;
pub const VIVS_DE_BLOCK4_SRC_COLOR_KEY_HIGH__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_COLOR_KEY_HIGH__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_MULTI_TILED__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_MULTI_TILED__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_MULTI_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_MULTI_TILED_ENABLE: u32 = 0x00000001;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_SUPER_TILED__MASK: u32 = 0x00000008;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_SUPER_TILED__SHIFT: u32 = 0x00000003;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_SUPER_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_SUPER_TILED_ENABLE: u32 = 0x00000008;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_MINOR_TILED__MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_MINOR_TILED__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_MINOR_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK4_SRC_EX_CONFIG_MINOR_TILED_ENABLE: u32 = 0x00000100;
pub const VIVS_DE_BLOCK4_SRC_EX_ADDRESS__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK4_SRC_EX_ADDRESS__LEN: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_ADDRESS__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_ADDRESS__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_STRIDE__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_STRIDE__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_STRIDE_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_BLOCK8_SRC_STRIDE_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_SRC_STRIDE_STRIDE(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_STRIDE_STRIDE__SHIFT) & VIVS_DE_BLOCK8_SRC_STRIDE_STRIDE__MASK }
pub const VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG_WIDTH__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG_WIDTH__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG_WIDTH(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG_WIDTH__SHIFT) & VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG_WIDTH__MASK }
pub const VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG_ROTATION__MASK: u32 = 0x00010000;
pub const VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG_ROTATION__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG_ROTATION_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_ROTATION_CONFIG_ROTATION_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_CONFIG__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_PE10_SOURCE_FORMAT__MASK: u32 = 0x0000000f;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_PE10_SOURCE_FORMAT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_SRC_CONFIG_PE10_SOURCE_FORMAT(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_CONFIG_PE10_SOURCE_FORMAT__SHIFT) & VIVS_DE_BLOCK8_SRC_CONFIG_PE10_SOURCE_FORMAT__MASK }
pub const VIVS_DE_BLOCK8_SRC_CONFIG_TRANSPARENCY__MASK: u32 = 0x00000030;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_TRANSPARENCY__SHIFT: u32 = 0x00000004;
pub const fn VIVS_DE_BLOCK8_SRC_CONFIG_TRANSPARENCY(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_CONFIG_TRANSPARENCY__SHIFT) & VIVS_DE_BLOCK8_SRC_CONFIG_TRANSPARENCY__MASK }
pub const VIVS_DE_BLOCK8_SRC_CONFIG_SRC_RELATIVE__MASK: u32 = 0x00000040;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_SRC_RELATIVE__SHIFT: u32 = 0x00000006;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_SRC_RELATIVE_ABSOLUTE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_SRC_RELATIVE_RELATIVE: u32 = 0x00000040;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_TILED__MASK: u32 = 0x00000080;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_TILED__SHIFT: u32 = 0x00000007;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_TILED_ENABLE: u32 = 0x00000080;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_LOCATION__MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_LOCATION__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_LOCATION_MEMORY: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_LOCATION_STREAM: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_PACK__MASK: u32 = 0x00003000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_PACK__SHIFT: u32 = 0x0000000c;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_PACK_PACKED8: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_PACK_PACKED16: u32 = 0x00001000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_PACK_PACKED32: u32 = 0x00002000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_PACK_UNPACKED: u32 = 0x00003000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_MONO_TRANSPARENCY__MASK: u32 = 0x00008000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_MONO_TRANSPARENCY__SHIFT: u32 = 0x0000000f;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_MONO_TRANSPARENCY_BACKGROUND: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_MONO_TRANSPARENCY_FOREGROUND: u32 = 0x00008000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_UNK16: u32 = 0x00010000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_SWIZZLE__MASK: u32 = 0x00300000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_SWIZZLE__SHIFT: u32 = 0x00000014;
pub const fn VIVS_DE_BLOCK8_SRC_CONFIG_SWIZZLE(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_CONFIG_SWIZZLE__SHIFT) & VIVS_DE_BLOCK8_SRC_CONFIG_SWIZZLE__MASK }
pub const VIVS_DE_BLOCK8_SRC_CONFIG_SOURCE_FORMAT__MASK: u32 = 0x1f000000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_SOURCE_FORMAT__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_BLOCK8_SRC_CONFIG_SOURCE_FORMAT(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_CONFIG_SOURCE_FORMAT__SHIFT) & VIVS_DE_BLOCK8_SRC_CONFIG_SOURCE_FORMAT__MASK }
pub const VIVS_DE_BLOCK8_SRC_CONFIG_DISABLE420_L2_CACHE: u32 = 0x20000000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_ENDIAN_CONTROL__MASK: u32 = 0xc0000000;
pub const VIVS_DE_BLOCK8_SRC_CONFIG_ENDIAN_CONTROL__SHIFT: u32 = 0x0000001e;
pub const fn VIVS_DE_BLOCK8_SRC_CONFIG_ENDIAN_CONTROL(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_CONFIG_ENDIAN_CONTROL__SHIFT) & VIVS_DE_BLOCK8_SRC_CONFIG_ENDIAN_CONTROL__MASK }
pub const VIVS_DE_BLOCK8_SRC_ORIGIN__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_ORIGIN__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_ORIGIN_X__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BLOCK8_SRC_ORIGIN_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_SRC_ORIGIN_X(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_ORIGIN_X__SHIFT) & VIVS_DE_BLOCK8_SRC_ORIGIN_X__MASK }
pub const VIVS_DE_BLOCK8_SRC_ORIGIN_Y__MASK: u32 = 0xffff0000;
pub const VIVS_DE_BLOCK8_SRC_ORIGIN_Y__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BLOCK8_SRC_ORIGIN_Y(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_ORIGIN_Y__SHIFT) & VIVS_DE_BLOCK8_SRC_ORIGIN_Y__MASK }
pub const VIVS_DE_BLOCK8_SRC_SIZE__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_SIZE__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_SIZE_X__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BLOCK8_SRC_SIZE_X__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_SRC_SIZE_X(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_SIZE_X__SHIFT) & VIVS_DE_BLOCK8_SRC_SIZE_X__MASK }
pub const VIVS_DE_BLOCK8_SRC_SIZE_Y__MASK: u32 = 0xffff0000;
pub const VIVS_DE_BLOCK8_SRC_SIZE_Y__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BLOCK8_SRC_SIZE_Y(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_SIZE_Y__SHIFT) & VIVS_DE_BLOCK8_SRC_SIZE_Y__MASK }
pub const VIVS_DE_BLOCK8_SRC_COLOR_BG__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_COLOR_BG__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_ROP__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_ROP__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_ROP_ROP_FG__MASK: u32 = 0x000000ff;
pub const VIVS_DE_BLOCK8_ROP_ROP_FG__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_ROP_ROP_FG(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ROP_ROP_FG__SHIFT) & VIVS_DE_BLOCK8_ROP_ROP_FG__MASK }
pub const VIVS_DE_BLOCK8_ROP_ROP_BG__MASK: u32 = 0x0000ff00;
pub const VIVS_DE_BLOCK8_ROP_ROP_BG__SHIFT: u32 = 0x00000008;
pub const fn VIVS_DE_BLOCK8_ROP_ROP_BG(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ROP_ROP_BG__SHIFT) & VIVS_DE_BLOCK8_ROP_ROP_BG__MASK }
pub const VIVS_DE_BLOCK8_ROP_TYPE__MASK: u32 = 0x00300000;
pub const VIVS_DE_BLOCK8_ROP_TYPE__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_BLOCK8_ROP_TYPE_ROP2_PATTERN: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ROP_TYPE_ROP2_SOURCE: u32 = 0x00100000;
pub const VIVS_DE_BLOCK8_ROP_TYPE_ROP3: u32 = 0x00200000;
pub const VIVS_DE_BLOCK8_ROP_TYPE_ROP4: u32 = 0x00300000;
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL_ENABLE__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL_ENABLE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL_ENABLE_OFF: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL_ENABLE_ON: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__MASK: u32 = 0x00ff0000;
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__SHIFT) & VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_SRC_ALPHA__MASK }
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__MASK: u32 = 0xff000000;
pub const VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__SHIFT) & VIVS_DE_BLOCK8_ALPHA_CONTROL_PE10_GLOBAL_DST_ALPHA__MASK }
pub const VIVS_DE_BLOCK8_ALPHA_MODES__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_ALPHA_MODES__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_ALPHA_MODE__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_ALPHA_MODE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_ALPHA_MODE_INVERSED: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_ALPHA_MODE__MASK: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_ALPHA_MODE__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_ALPHA_MODE_INVERSED: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE__MASK: u32 = 0x00000300;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_GLOBAL: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_SRC_ALPHA_MODE_SCALED: u32 = 0x00000200;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE__MASK: u32 = 0x00003000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE__SHIFT: u32 = 0x0000000c;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE_NORMAL: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE_GLOBAL: u32 = 0x00001000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_GLOBAL_DST_ALPHA_MODE_SCALED: u32 = 0x00002000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY__MASK: u32 = 0x00010000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_PE10_SRC_COLOR_MULTIPLY_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY__MASK: u32 = 0x00100000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_PE10_DST_COLOR_MULTIPLY_ENABLE: u32 = 0x00100000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_BLENDING_MODE__MASK: u32 = 0x07000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_BLENDING_MODE__SHIFT: u32 = 0x00000018;
pub const fn VIVS_DE_BLOCK8_ALPHA_MODES_SRC_BLENDING_MODE(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ALPHA_MODES_SRC_BLENDING_MODE__SHIFT) & VIVS_DE_BLOCK8_ALPHA_MODES_SRC_BLENDING_MODE__MASK }
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_ALPHA_FACTOR__MASK: u32 = 0x08000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_ALPHA_FACTOR__SHIFT: u32 = 0x0000001b;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_ALPHA_FACTOR_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_SRC_ALPHA_FACTOR_ENABLE: u32 = 0x08000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_BLENDING_MODE__MASK: u32 = 0x70000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_BLENDING_MODE__SHIFT: u32 = 0x0000001c;
pub const fn VIVS_DE_BLOCK8_ALPHA_MODES_DST_BLENDING_MODE(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ALPHA_MODES_DST_BLENDING_MODE__SHIFT) & VIVS_DE_BLOCK8_ALPHA_MODES_DST_BLENDING_MODE__MASK }
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_ALPHA_FACTOR__MASK: u32 = 0x80000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_ALPHA_FACTOR__SHIFT: u32 = 0x0000001f;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_ALPHA_FACTOR_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_ALPHA_MODES_DST_ALPHA_FACTOR_ENABLE: u32 = 0x80000000;
pub const VIVS_DE_BLOCK8_ADDRESS_U__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_ADDRESS_U__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_STRIDE_U__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_STRIDE_U__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_STRIDE_U_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_BLOCK8_STRIDE_U_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_STRIDE_U_STRIDE(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_STRIDE_U_STRIDE__SHIFT) & VIVS_DE_BLOCK8_STRIDE_U_STRIDE__MASK }
pub const VIVS_DE_BLOCK8_ADDRESS_V__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_ADDRESS_V__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_STRIDE_V__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_STRIDE_V__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_STRIDE_V_STRIDE__MASK: u32 = 0x0003ffff;
pub const VIVS_DE_BLOCK8_STRIDE_V_STRIDE__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_STRIDE_V_STRIDE(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_STRIDE_V_STRIDE__SHIFT) & VIVS_DE_BLOCK8_STRIDE_V_STRIDE__MASK }
pub const VIVS_DE_BLOCK8_SRC_ROTATION_HEIGHT__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_ROTATION_HEIGHT__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_ROTATION_HEIGHT_HEIGHT__MASK: u32 = 0x0000ffff;
pub const VIVS_DE_BLOCK8_SRC_ROTATION_HEIGHT_HEIGHT__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_SRC_ROTATION_HEIGHT_HEIGHT(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_SRC_ROTATION_HEIGHT_HEIGHT__SHIFT) & VIVS_DE_BLOCK8_SRC_ROTATION_HEIGHT_HEIGHT__MASK }
pub const VIVS_DE_BLOCK8_ROT_ANGLE__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_ROT_ANGLE__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_ROT_ANGLE_SRC__MASK: u32 = 0x00000007;
pub const VIVS_DE_BLOCK8_ROT_ANGLE_SRC__SHIFT: u32 = 0x00000000;
pub const fn VIVS_DE_BLOCK8_ROT_ANGLE_SRC(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ROT_ANGLE_SRC__SHIFT) & VIVS_DE_BLOCK8_ROT_ANGLE_SRC__MASK }
pub const VIVS_DE_BLOCK8_ROT_ANGLE_DST__MASK: u32 = 0x00000038;
pub const VIVS_DE_BLOCK8_ROT_ANGLE_DST__SHIFT: u32 = 0x00000003;
pub const fn VIVS_DE_BLOCK8_ROT_ANGLE_DST(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ROT_ANGLE_DST__SHIFT) & VIVS_DE_BLOCK8_ROT_ANGLE_DST__MASK }
pub const VIVS_DE_BLOCK8_ROT_ANGLE_SRC_MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_ROT_ANGLE_DST_MASK: u32 = 0x00000200;
pub const VIVS_DE_BLOCK8_ROT_ANGLE_SRC_MIRROR__MASK: u32 = 0x00003000;
pub const VIVS_DE_BLOCK8_ROT_ANGLE_SRC_MIRROR__SHIFT: u32 = 0x0000000c;
pub const fn VIVS_DE_BLOCK8_ROT_ANGLE_SRC_MIRROR(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ROT_ANGLE_SRC_MIRROR__SHIFT) & VIVS_DE_BLOCK8_ROT_ANGLE_SRC_MIRROR__MASK }
pub const VIVS_DE_BLOCK8_ROT_ANGLE_SRC_MIRROR_MASK: u32 = 0x00008000;
pub const VIVS_DE_BLOCK8_ROT_ANGLE_DST_MIRROR__MASK: u32 = 0x00030000;
pub const VIVS_DE_BLOCK8_ROT_ANGLE_DST_MIRROR__SHIFT: u32 = 0x00000010;
pub const fn VIVS_DE_BLOCK8_ROT_ANGLE_DST_MIRROR(x: u32) -> u32 { (x << VIVS_DE_BLOCK8_ROT_ANGLE_DST_MIRROR__SHIFT) & VIVS_DE_BLOCK8_ROT_ANGLE_DST_MIRROR__MASK }
pub const VIVS_DE_BLOCK8_ROT_ANGLE_DST_MIRROR_MASK: u32 = 0x00080000;
pub const VIVS_DE_BLOCK8_GLOBAL_SRC_COLOR__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_GLOBAL_SRC_COLOR__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_GLOBAL_DEST_COLOR__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_GLOBAL_DEST_COLOR__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_SRC_PREMULTIPLY_ENABLE: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY__MASK: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_DST_PREMULTIPLY_ENABLE: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY__MASK: u32 = 0x00000300;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_ALPHA: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_SRC_GLOBAL_PREMULTIPLY_COLOR: u32 = 0x00000200;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY__MASK: u32 = 0x00100000;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_COLOR_MULTIPLY_MODES_DST_DEMULTIPLY_ENABLE: u32 = 0x00100000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_TRANSPARENCY__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_SOURCE__MASK: u32 = 0x00000003;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_SOURCE__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_SOURCE_OPAQUE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_SOURCE_MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_SOURCE_KEY: u32 = 0x00000002;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_PATTERN__MASK: u32 = 0x00000030;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_PATTERN__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_PATTERN_OPAQUE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_PATTERN_MASK: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_PATTERN_KEY: u32 = 0x00000020;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DESTINATION__MASK: u32 = 0x00000300;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DESTINATION__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DESTINATION_OPAQUE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DESTINATION_MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DESTINATION_KEY: u32 = 0x00000200;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_TRANSPARENCY_MASK: u32 = 0x00001000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_SRC_OVERRIDE__MASK: u32 = 0x00030000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_SRC_OVERRIDE__SHIFT: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_SRC_OVERRIDE_DEFAULT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_SRC_OVERRIDE_USE_ENABLE: u32 = 0x00010000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_SRC_OVERRIDE_USE_DISABLE: u32 = 0x00020000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_PAT_OVERRIDE__MASK: u32 = 0x00300000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_PAT_OVERRIDE__SHIFT: u32 = 0x00000014;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_PAT_OVERRIDE_DEFAULT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_PAT_OVERRIDE_USE_ENABLE: u32 = 0x00100000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_PAT_OVERRIDE_USE_DISABLE: u32 = 0x00200000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_DST_OVERRIDE__MASK: u32 = 0x03000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_DST_OVERRIDE__SHIFT: u32 = 0x00000018;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_DST_OVERRIDE_DEFAULT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_DST_OVERRIDE_USE_ENABLE: u32 = 0x01000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_USE_DST_OVERRIDE_USE_DISABLE: u32 = 0x02000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_RESOURCE_OVERRIDE_MASK: u32 = 0x10000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DFB_COLOR_KEY__MASK: u32 = 0x20000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DFB_COLOR_KEY__SHIFT: u32 = 0x0000001d;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DFB_COLOR_KEY_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DFB_COLOR_KEY_ENABLE: u32 = 0x20000000;
pub const VIVS_DE_BLOCK8_TRANSPARENCY_DFB_COLOR_KEY_MASK: u32 = 0x80000000;
pub const VIVS_DE_BLOCK8_CONTROL__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_CONTROL__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_CONTROL_YUV__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_CONTROL_YUV__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_CONTROL_YUV_601: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_CONTROL_YUV_709: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_CONTROL_YUV_MASK: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_CONTROL_UV_SWIZZLE__MASK: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_CONTROL_UV_SWIZZLE__SHIFT: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_CONTROL_UV_SWIZZLE_UV: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_CONTROL_UV_SWIZZLE_VU: u32 = 0x00000010;
pub const VIVS_DE_BLOCK8_CONTROL_UV_SWIZZLE_MASK: u32 = 0x00000080;
pub const VIVS_DE_BLOCK8_CONTROL_YUVRGB__MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_CONTROL_YUVRGB__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_CONTROL_YUVRGB_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_CONTROL_YUVRGB_ENABLE: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_CONTROL_YUVRGB_MASK: u32 = 0x00000800;
pub const VIVS_DE_BLOCK8_SRC_COLOR_KEY_HIGH__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_COLOR_KEY_HIGH__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG__LEN: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_MULTI_TILED__MASK: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_MULTI_TILED__SHIFT: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_MULTI_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_MULTI_TILED_ENABLE: u32 = 0x00000001;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_SUPER_TILED__MASK: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_SUPER_TILED__SHIFT: u32 = 0x00000003;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_SUPER_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_SUPER_TILED_ENABLE: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_MINOR_TILED__MASK: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_MINOR_TILED__SHIFT: u32 = 0x00000008;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_MINOR_TILED_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_BLOCK8_SRC_EX_CONFIG_MINOR_TILED_ENABLE: u32 = 0x00000100;
pub const VIVS_DE_BLOCK8_SRC_EX_ADDRESS__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_BLOCK8_SRC_EX_ADDRESS__LEN: u32 = 0x00000008;
pub const VIVS_DE_TS: u32 = 0x00000000;
pub const VIVS_DE_TS_ADDRESS__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_TS_ADDRESS__LEN: u32 = 0x00000008;
pub const VIVS_DE_TS_CLEAR_VALUE__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_TS_CLEAR_VALUE__LEN: u32 = 0x00000008;
pub const VIVS_DE_TS_CONTROL__ESIZE: u32 = 0x00000004;
pub const VIVS_DE_TS_CONTROL__LEN: u32 = 0x00000008;
pub const VIVS_DE_TS_CONTROL_ENABLE__MASK: u32 = 0x00000004;
pub const VIVS_DE_TS_CONTROL_ENABLE__SHIFT: u32 = 0x00000002;
pub const VIVS_DE_TS_CONTROL_ENABLE_DISABLE: u32 = 0x00000000;
pub const VIVS_DE_TS_CONTROL_ENABLE_ENABLE: u32 = 0x00000004;
