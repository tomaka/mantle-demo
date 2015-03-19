#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

pub type GR_CHAR = libc::c_char;
pub type GR_INT = libc::c_int;      // FIXME: not sure with 32/64bits
pub type GR_UINT = libc::c_uint;      // FIXME: not sure with 32/64bits
pub type GR_UINT32 = libc::uint32_t;
pub type GR_SIZE = libc::size_t;
pub type GR_ENUM = libc::uint32_t;
pub type GR_VOID = libc::c_void;
pub type GR_PHYSICAL_GPU = libc::uint64_t;      // FIXME: not sure with 32/64bits
pub type GR_DEVICE = libc::uint64_t;      // FIXME: not sure with 32/64bits
pub type GR_WSI_WIN_DISPLAY = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_IMAGE = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_GPU_MEMORY = libc::uint64_t;       // FIXME: not sure with 32/64bits

pub type GR_FLAGS = libc::c_uint;       // FIXME: total guess

pub const GR_MAX_PHYSICAL_GPUS: usize = 4;
pub const GR_API_VERSION: u32 = 1;      // FIXME: this was guessed

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

#[repr(C)]
pub enum GR_QUEUE_TYPE {
    GR_QUEUE_UNIVERSAL = 0x1000,
    GR_QUEUE_COMPUTE = 0x1001,
}

#[repr(C)]
pub enum GR_VALIDATION_LEVEL {
    GR_VALIDATION_LEVEL_0 = 0x8000,
    GR_VALIDATION_LEVEL_1 = 0x8001,
    GR_VALIDATION_LEVEL_2 = 0x8002,
    GR_VALIDATION_LEVEL_3 = 0x8003,
    GR_VALIDATION_LEVEL_4 = 0x8004,
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

#[repr(C)]
pub struct GR_DEVICE_QUEUE_CREATE_INFO {
    pub queueType: GR_QUEUE_TYPE,
    pub queueCount: GR_UINT,
}

#[repr(C)]
pub struct GR_DEVICE_CREATE_INFO {
    pub queueRecordCount: GR_UINT,
    pub pRequestedQueues: *const GR_DEVICE_QUEUE_CREATE_INFO,
    pub extensionCount: GR_UINT,
    pub ppEnabledExtensionNames: *const *const GR_CHAR,
    pub maxValidationLevel: GR_VALIDATION_LEVEL,
    pub flags: GR_FLAGS,
}

#[repr(C)]
pub struct GR_WSI_WIN_PRESENTABLE_IMAGE_CREATE_INFO {
    pub format: GR_FORMAT,
    pub usage: GR_FLAGS,
    pub extent: GR_EXTENT2D,
    pub display: GR_WSI_WIN_DISPLAY,
    pub flags: GR_FLAGS,
}

#[repr(C)]
pub struct GR_FORMAT {
    // FIXME: not sure about the order
    pub channelFormat: libc::uint16_t,
    pub numericFormat: libc::uint16_t,
}

#[repr(C)]
pub struct GR_EXTENT2D {
    pub width: GR_INT,
    pub height: GR_INT,
}

extern {
    pub fn grInitAndEnumerateGpus(pAppInfo: *const GR_APPLICATION_INFO,
                                  pAllocCb: *const GR_ALLOC_CALLBACKS, pGpuCount: *mut GR_UINT,
                                  gpus: *mut GR_PHYSICAL_GPU) -> GR_RESULT;

    pub fn grCreateDevice(gpu: GR_PHYSICAL_GPU, pCreateInfo: *const GR_DEVICE_CREATE_INFO,
                          pDevice: *mut GR_DEVICE) -> GR_RESULT;


    pub fn grWsiWinCreatePresentableImage(device: GR_DEVICE, pCreateInfo: *const
                                          GR_WSI_WIN_PRESENTABLE_IMAGE_CREATE_INFO,
                                          pImage: *mut GR_IMAGE, pMem: *mut GR_GPU_MEMORY);
}
