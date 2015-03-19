#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

// FIXME: all the definitions are total guesses
pub type GR_CHAR = libc::c_char;
pub type GR_UINT = libc::c_uint;
pub type GR_UINT32 = libc::uint32_t;
pub type GR_SIZE = libc::size_t;
pub type GR_ENUM = libc::uint32_t;        // FIXME: no idea what the real type is
pub type GR_VOID = libc::c_void;
pub type GR_PHYSICAL_GPU = *const libc::c_void;     // FIXME: no idea

// FIXME: total guess
pub const GR_MAX_PHYSICAL_GPUS: usize = 64;

#[derive(Debug, Copy)]
#[repr(C)]
pub enum GR_RESULT {
    GR_SUCCESS = 0x10000,
    GR_UNSUPPORTED,
    GR_NOT_READY,
    GR_TIMEOUT,
    GR_EVENT_SET,
    GR_EVENT_RESET,

    GR_ERROR_UNKNOW = 0x11000,
    GR_ERROR_UNAVAILABLE,
    GR_ERROR_INITIALIZATION_FAILED,
    GR_ERROR_OUT_OF_MEMORY,
    GR_ERROR_OUT_OF_GPU_MEMORY,
    GR_ERROR_DEVICE_ALREADY_CREATED,
    GR_ERROR_DEVICE_LOST,
    GR_ERROR_INVALID_POINTER,
    GR_ERROR_INVALID_VALUE,
    GR_ERROR_INVALID_HANDLE,
    GR_ERROR_INVALID_ORDINAL,
    GR_ERROR_INVALID_MEMORY_SIZE,   
    GR_ERROR_INVALID_EXTENSION,
    GR_ERROR_INVALID_FLAGS,
    GR_ERROR_INVALID_ALIGNMENT,
    GR_ERROR_INVALID_FORMAT,
    GR_ERROR_INVALID_IMAGE,
    GR_ERROR_INVALID_DESCRIPTOR_SET_DATA,
    GR_ERROR_INVALID_QUEUE_TYPE,
    GR_ERROR_INVALID_OBJECT_TYPE,
    GR_ERROR_UNSUPPORTED_SHADER_IL_VERSION,
    GR_ERROR_BAD_SHADER_CODE,
    GR_ERROR_BAD_PIPELINE_DATA,
    GR_ERROR_TOO_MANY_MEMORY_REFERENCES,
    GR_ERROR_NOT_MAPPABLE,
    GR_ERROR_MEMORY_MAP_FAILED,
    GR_ERROR_MEMORY_UNMAP_FAILED,
    GR_ERROR_INCOMPATIBLE_DEVICE,
    GR_ERROR_INCOMPATIBLE_DRIVER,
    GR_ERROR_INCOMPLETE_COMMAND_BUFFER,
    GR_ERROR_BUILDING_COMMAND_BUFFER,
    GR_ERROR_MEMORY_NOT_BOUND,
    GR_ERROR_INCOMPATIBLE_QUEUE,
    GR_ERROR_NOT_SHAREABLE
}

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
