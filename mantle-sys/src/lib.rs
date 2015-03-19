#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

// FIXME: all the definitions are total guesses
pub type GR_RESULT = libc::c_int;   // FIXME: absolutely no idea what the real type of this is
pub type GR_CHAR = libc::c_char;
pub type GR_UINT = libc::c_uint;
pub type GR_UINT32 = libc::uint32_t;
pub type GR_SIZE = libc::size_t;
pub type GR_ENUM = libc::uint32_t;        // FIXME: no idea what the real type is
pub type GR_VOID = libc::c_void;
pub type GR_PHYSICAL_GPU = *const libc::c_void;     // FIXME: no idea

// FIXME: total guess
pub const GR_MAX_PHYSICAL_GPUS: usize = 64;

// these are not guesses anymore (TODO: remove this comment)
pub type GR_ALLOC_FUNCTION = extern "stdcall" fn(GR_SIZE, GR_SIZE, GR_ENUM) -> *mut GR_VOID;
pub type GR_FREE_FUNCTION = extern "stdcall" fn(*mut GR_VOID);

#[repr(C)]
pub struct GR_APPLICATION_INFO {
    pub pAppName: *const GR_CHAR,
    pub appVersion: GR_UINT32,
    pub pEngineName: *const GR_CHAR,
    pub engineVersion: GR_UINT32,
    pub apiVersion: GR_UINT32,
}

#[repr(C)]
pub struct GR_ALLOC_CALLBACKS {
    pub pfnAlloc: GR_ALLOC_FUNCTION,
    pub pfnFree: GR_FREE_FUNCTION,
}

extern {
    pub fn grInitAndEnumerateGpus(pAppInfo: *const GR_APPLICATION_INFO,
                                  pAllocCb: *const GR_ALLOC_CALLBACKS, pGpuCount: *mut GR_UINT,
                                  gpus: *mut GR_PHYSICAL_GPU) -> GR_RESULT;    // real definition: GR_PHYSICAL_GPU gpus[GR_MAX_PHYSICAL_GPUS]
}
