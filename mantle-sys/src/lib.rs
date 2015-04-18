#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate winapi;

pub type GR_CHAR = libc::c_char;
pub type GR_INT = libc::c_int;      // FIXME: not sure with 32/64bits
pub type GR_UINT = libc::c_uint;      // FIXME: not sure with 32/64bits
pub type GR_UINT8 = libc::uint8_t;
pub type GR_UINT32 = libc::uint32_t;
pub type GR_SIZE = libc::size_t;
pub type GR_ENUM = libc::uint32_t;
pub type GR_VOID = libc::c_void;
pub type GR_FLOAT = libc::c_float;
pub type GR_BOOL = bool;            // FIXME: 
pub type GR_GPU_SIZE = libc::size_t;      // FIXME: total guess
pub type GR_FLAGS = libc::c_uint;       // FIXME: total guess
pub type GR_SAMPLE_MASK = libc::uint32_t;       // FIXME: total guess

pub type GR_PHYSICAL_GPU = libc::uint64_t;      // FIXME: not sure with 32/64bits
pub type GR_DEVICE = libc::uint64_t;      // FIXME: not sure with 32/64bits
pub type GR_WSI_WIN_DISPLAY = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_IMAGE = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_GPU_MEMORY = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_QUEUE = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_CMD_BUFFER = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_FENCE = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_BASE_OBJECT = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_OBJECT = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_SHADER = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_RASTER_STATE_OBJECT = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_VIEWPORT_STATE_OBJECT = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_COLOR_BLEND_STATE_OBJECT = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_DEPTH_STENCIL_STATE_OBJECT = libc::uint64_t;       // FIXME: not sure with 32/64bits
pub type GR_MSAA_STATE_OBJECT = libc::uint64_t;       // FIXME: not sure with 32/64bits

pub const GR_MAX_PHYSICAL_GPUS: usize = 4;
pub const GR_API_VERSION: u32 = 1;      // FIXME: this was guessed
pub const GR_MAX_MEMORY_HEAPS: usize = 8;
pub const GR_MAX_VIEWPORTS: usize = 16;     // FIXME: taken from OverV's example
pub const GR_MAX_COLOR_TARGETS: usize = 16;     // FIXME: taken from OverV's example

#[derive(Debug, Copy, Clone)]
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

//#[repr(C)]
//pub enum GR_IMAGE_USAGE_FLAGS {
pub const GR_IMAGE_USAGE_SHADER_ACCESS_READ: GR_FLAGS = 0x00000001;
pub const GR_IMAGE_USAGE_SHADER_ACCESS_WRITE: GR_FLAGS = 0x00000002;
pub const GR_IMAGE_USAGE_COLOR_TARGET: GR_FLAGS = 0x00000004;
pub const GR_IMAGE_USAGE_DEPTH_STENCIL: GR_FLAGS = 0x00000008;
//}

// GR_WSI_WIN_PRESENT_MODE
pub const GR_WSI_WIN_PRESENT_MODE_WINDOWED: GR_ENUM = 0x00200200;
pub const GR_WSI_WIN_PRESENT_MODE_FULLSCREEN: GR_ENUM = 0x00200201;

// GR_QUEUE_TYPE
pub const GR_QUEUE_UNIVERSAL: GR_ENUM = 0x1000;
pub const GR_QUEUE_COMPUTE: GR_ENUM = 0x1001;

// GR_IMAGE_ASPECT
pub const GR_IMAGE_ASPECT_COLOR: GR_ENUM = 0x1700;
pub const GR_IMAGE_ASPECT_DEPTH: GR_ENUM = 0x1701;
pub const GR_IMAGE_ASPECT_STENCIL: GR_ENUM = 0x1702;

// GR_DEVICE_CREATE_FLAGS
pub const GR_DEVICE_CREATE_VALIDATION: GR_ENUM = 0x00000001;

// GR_IMAGE_STATE
pub const GR_IMAGE_STATE_DATA_TRANSFER: GR_ENUM = 0x1300;
pub const GR_IMAGE_STATE_GRAPHICS_SHADER_READ_ONLY: GR_ENUM = 0x1301;
pub const GR_IMAGE_STATE_GRAPHICS_SHADER_WRITE_ONLY: GR_ENUM = 0x1302;
pub const GR_IMAGE_STATE_GRAPHICS_SHADER_READ_WRITE: GR_ENUM = 0x1303;
pub const GR_IMAGE_STATE_COMPUTE_SHADER_READ_ONLY: GR_ENUM = 0x1304;
pub const GR_IMAGE_STATE_COMPUTE_SHADER_WRITE_ONLY: GR_ENUM = 0x1305;
pub const GR_IMAGE_STATE_COMPUTE_SHADER_READ_WRITE: GR_ENUM = 0x1306;
pub const GR_IMAGE_STATE_MULTI_SHADER_READ_ONLY: GR_ENUM = 0x1307;
pub const GR_IMAGE_STATE_TARGET_AND_SHADER_READ_ONLY: GR_ENUM = 0x1308;
pub const GR_IMAGE_STATE_UNINITIALIZED: GR_ENUM = 0x1309;
pub const GR_IMAGE_STATE_TARGET_RENDER_ACCESS_OPTIMAL: GR_ENUM = 0x130a;
pub const GR_IMAGE_STATE_TARGET_SHADER_ACCESS_OPTIMAL: GR_ENUM = 0x130b;
pub const GR_IMAGE_STATE_CLEAR: GR_ENUM = 0x130c;
pub const GR_IMAGE_STATE_RESOLVE_SOURCE: GR_ENUM = 0x130d;
pub const GR_IMAGE_STATE_RESOLVE_DESTINATION: GR_ENUM = 0x130e;
pub const GR_IMAGE_STATE_DISCARD: GR_ENUM = 0x131f;
pub const GR_IMAGE_STATE_DATA_TRANSFER_SOURCE: GR_ENUM = 0x1310;
pub const GR_IMAGE_STATE_DATA_TRANSFER_DESTINATION: GR_ENUM = 0x1311;
pub const GR_WSI_WIN_IMAGE_STATE_PRESENT_WINDOWED: GR_ENUM = 0x00200000;
pub const GR_WSI_WIN_IMAGE_STATE_PRESENT_FULLSCREEN: GR_ENUM = 0x00200001;

pub const GR_CMD_BUFFER_OPTIMIZE_GPU_SMALL_BATCH: GR_ENUM = 0x00000001;
pub const GR_CMD_BUFFER_OPTIMIZE_PIPELINE_SWITCH: GR_ENUM = 0x00000002;
pub const GR_CMD_BUFFER_OPTIMIZE_ONE_TIME_SUBMIT: GR_ENUM = 0x00000004;
pub const GR_CMD_BUFFER_OPTIMIZE_DESCRIPTOR_SET_SWITCH: GR_ENUM = 0x00000008;

pub const GR_SHADER_CREATE_ALLOW_RE_Z: GR_FLAGS = 0x00000001;

// GR_INFO_TYPE
pub const GR_INFO_TYPE_PHYSICAL_GPU_PROPERTIES: GR_ENUM = 0x6100;
pub const GR_INFO_TYPE_PHYSICAL_GPU_PERFORMANCE: GR_ENUM = 0x6101;
pub const GR_INFO_TYPE_PHYSICAL_GPU_QUEUE_PROPERTIES: GR_ENUM = 0x6102;
pub const GR_INFO_TYPE_PHYSICAL_GPU_MEMORY_PROPERTIES: GR_ENUM = 0x6103;
pub const GR_INFO_TYPE_PHYSICAL_GPU_IMAGE_PROPERTIES: GR_ENUM = 0x6104;
pub const GR_INFO_TYPE_MEMORY_HEAP_PROPERTIES: GR_ENUM = 0x6200;
pub const GR_INFO_TYPE_FORMAT_PROPERTIES: GR_ENUM = 0x6300;
pub const GR_INFO_TYPE_SUBRESOURCE_LAYOUT: GR_ENUM = 0x6400;
pub const GR_INFO_TYPE_MEMORY_REQUIREMENTS: GR_ENUM = 0x6800;
pub const GR_INFO_TYPE_PARENT_DEVICE: GR_ENUM = 0x6801;
pub const GR_INFO_TYPE_PARENT_PHYSICAL_GPU: GR_ENUM = 0x6802;

// GR_HEAP_MEMORY_TYPE
pub const GR_HEAP_MEMORY_OTHER: GR_ENUM = 0x2f00;
pub const GR_HEAP_MEMORY_LOCAL: GR_ENUM = 0x2f01;
pub const GR_HEAP_MEMORY_REMOTE: GR_ENUM = 0x2f02;
pub const GR_HEAP_MEMORY_EMBEDDED: GR_ENUM = 0x2f03;

// GR_MEMORY_HEAP_FLAGS
pub const GR_MEMORY_HEAP_CPU_VISIBLE: GR_FLAGS = 0x00000001;
pub const GR_MEMORY_HEAP_CPU_GPU_COHERENT: GR_FLAGS = 0x00000002;
pub const GR_MEMORY_HEAP_CPU_UNCACHED: GR_FLAGS = 0x00000004;
pub const GR_MEMORY_HEAP_CPU_WRITE_COMBINED: GR_FLAGS = 0x00000008;
pub const GR_MEMORY_HEAP_HOLDS_PINNED: GR_FLAGS = 0x00000010;
pub const GR_MEMORY_HEAP_SHAREABLE: GR_FLAGS = 0x00000020;

// GR_MEMORY_PRIORITY
pub const GR_MEMORY_PRIORITY_NORMAL: GR_ENUM = 0x1100;
pub const GR_MEMORY_PRIORITY_HIGH: GR_ENUM = 0x1101;
pub const GR_MEMORY_PRIORITY_LOW: GR_ENUM = 0x1102;
pub const GR_MEMORY_PRIORITY_UNUSED: GR_ENUM = 0x1103;
pub const GR_MEMORY_PRIORITY_VERY_HIGH: GR_ENUM = 0x1104;
pub const GR_MEMORY_PRIORITY_VERY_LOW: GR_ENUM = 0x1105;

// GR_MEMORY_STATE
pub const GR_MEMORY_STATE_DATA_TRANSFER: GR_ENUM = 0x1200;
pub const GR_MEMORY_STATE_GRAPHICS_SHADER_READ_ONLY: GR_ENUM = 0x1201;
pub const GR_MEMORY_STATE_GRAPHICS_SHADER_WRITE_ONLY: GR_ENUM = 0x1202;
pub const GR_MEMORY_STATE_GRAPHICS_SHADER_READ_WRITE: GR_ENUM = 0x1203;
pub const GR_MEMORY_STATE_COMPUTE_SHADER_READ_ONLY: GR_ENUM = 0x1204;
pub const GR_MEMORY_STATE_COMPUTE_SHADER_WRITE_ONLY: GR_ENUM = 0x1205;
pub const GR_MEMORY_STATE_COMPUTE_SHADER_READ_WRITE: GR_ENUM = 0x1206;
pub const GR_MEMORY_STATE_MULTI_USE_READ_ONLY: GR_ENUM = 0x1207;
pub const GR_MEMORY_STATE_INDEX_DATA: GR_ENUM = 0x1208;
pub const GR_MEMORY_STATE_INDIRECT_ARG: GR_ENUM = 0x1209;
pub const GR_MEMORY_STATE_WRITE_TIMESTAMP: GR_ENUM = 0x120a;
pub const GR_MEMORY_STATE_QUEUE_ATOMIC: GR_ENUM = 0x120b;
pub const GR_MEMORY_STATE_DISCARD: GR_ENUM = 0x120c;
pub const GR_MEMORY_STATE_DATA_TRANSFER_SOURCE: GR_ENUM = 0x120d;
pub const GR_MEMORY_STATE_DATA_TRANSFER_DESTINATION: GR_ENUM = 0x120e;

// GR_FILL_MODE
pub const GR_FILL_SOLID: GR_ENUM = 0x2600;
pub const GR_FILL_WIREFRAME: GR_ENUM = 0x2601;

// GR_CULL_MODE
pub const GR_CULL_NONE: GR_ENUM = 0x2700;
pub const GR_CULL_FRONT: GR_ENUM = 0x2701;
pub const GR_CULL_BACK: GR_ENUM = 0x2702;

// GR_FACE_ORIENTATION
pub const GR_FRONT_FACE_CCW: GR_ENUM = 0x2800;
pub const GR_FRONT_FACE_CW: GR_ENUM = 0x2801;

// GR_BLEND
pub const GR_BLEND_ZERO: GR_ENUM = 0x2900;
pub const GR_BLEND_ONE: GR_ENUM = 0x2901;
pub const GR_BLEND_SRC_COLOR: GR_ENUM = 0x2902;
pub const GR_BLEND_ONE_MINUS_SRC_COLOR: GR_ENUM = 0x2903;
pub const GR_BLEND_DEST_COLOR: GR_ENUM = 0x2904;
pub const GR_BLEND_ONE_MINUS_DEST_COLOR: GR_ENUM = 0x2905;
pub const GR_BLEND_SRC_ALPHA: GR_ENUM = 0x2906;
pub const GR_BLEND_ONE_MINUS_SRC_ALPHA: GR_ENUM = 0x2907;
pub const GR_BLEND_DEST_ALPHA: GR_ENUM = 0x2908;
pub const GR_BLEND_ONE_MINUS_DEST_ALPHA: GR_ENUM = 0x2909;
pub const GR_BLEND_CONSTANT_COLOR: GR_ENUM = 0x290a;
pub const GR_BLEND_ONE_MINUS_CONSTANT_COLOR: GR_ENUM = 0x290b;
pub const GR_BLEND_CONSTANT_ALPHA: GR_ENUM = 0x290c;
pub const GR_BLEND_ONE_MINUS_CONSTANT_ALPHA: GR_ENUM = 0x290d;
pub const GR_BLEND_SRC_ALPHA_SATURATE: GR_ENUM = 0x290e;
pub const GR_BLEND_SRC1_COLOR: GR_ENUM = 0x290f;
pub const GR_BLEND_ONE_MINUS_SRC1_COLOR: GR_ENUM = 0x2910;
pub const GR_BLEND_SRC1_ALPHA: GR_ENUM = 0x2911;
pub const GR_BLEND_ONE_MINUS_SRC1_ALPHA: GR_ENUM = 0x2912;

// GR_BLEND_FUNC
pub const GR_BLEND_FUNC_ADD: GR_ENUM = 0x2a00;
pub const GR_BLEND_FUNC_SUBTRACT: GR_ENUM = 0x2a01;
pub const GR_BLEND_FUNC_REVERSE_SUBTRACT: GR_ENUM = 0x2a02;
pub const GR_BLEND_FUNC_MIN: GR_ENUM = 0x2a03;
pub const GR_BLEND_FUNC_MAX: GR_ENUM = 0x2a04;

// GR_COMPARE_FUNC
pub const GR_COMPARE_NEVER: GR_ENUM = 0x2500;
pub const GR_COMPARE_LESS: GR_ENUM = 0x2501;
pub const GR_COMPARE_EQUAL: GR_ENUM = 0x2502;
pub const GR_COMPARE_LESS_EQUAL: GR_ENUM = 0x2503;
pub const GR_COMPARE_GREATER: GR_ENUM = 0x2504;
pub const GR_COMPARE_NOT_EQUAL: GR_ENUM = 0x2505;
pub const GR_COMPARE_GREATER_EQUAL: GR_ENUM = 0x2506;
pub const GR_COMPARE_ALWAYS: GR_ENUM = 0x2507;

// GR_STENCIL_OP
pub const GR_STENCIL_OP_KEEP: GR_ENUM = 0x2b00;
pub const GR_STENCIL_OP_ZERO: GR_ENUM = 0x2b01;
pub const GR_STENCIL_OP_REPLACE: GR_ENUM = 0x2b02;
pub const GR_STENCIL_OP_INC_CLAMP: GR_ENUM = 0x2b03;
pub const GR_STENCIL_OP_DEC_CLAMP: GR_ENUM = 0x2b04;
pub const GR_STENCIL_OP_INVERT: GR_ENUM = 0x2b05;
pub const GR_STENCIL_OP_INC_WRAP: GR_ENUM = 0x2b06;
pub const GR_STENCIL_OP_DEC_WRAP: GR_ENUM = 0x2b07;

pub type GR_ALLOC_FUNCTION = unsafe extern "stdcall" fn(GR_SIZE, GR_SIZE, GR_ENUM) -> *mut GR_VOID;
pub type GR_FREE_FUNCTION = unsafe extern "stdcall" fn(*mut GR_VOID);
pub type GR_DBG_MSG_CALLBACK_FUNCTION = unsafe extern "stdcall" fn(GR_ENUM, GR_ENUM, GR_BASE_OBJECT,
                                                                   GR_SIZE, GR_ENUM, *const GR_CHAR,
                                                                   *mut GR_VOID);

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
    pub channelFormat: libc::uint16_t,
    pub numericFormat: libc::uint16_t,
}

#[repr(C)]
pub struct GR_EXTENT2D {
    pub width: GR_INT,
    pub height: GR_INT,
}

#[repr(C)]
pub struct GR_WSI_WIN_PRESENT_INFO {
    pub hWndDest: winapi::HWND,
    pub srcImage: GR_IMAGE,
    pub presentMode: GR_ENUM,
    pub presentInterval: GR_UINT,
    pub flags: GR_FLAGS,
}

#[repr(C)]
pub struct GR_CMD_BUFFER_CREATE_INFO {
    pub queueType: GR_ENUM,
    pub flags: GR_FLAGS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GR_MEMORY_REF {
    pub mem: GR_GPU_MEMORY,
    pub flags: GR_FLAGS,
}

#[repr(C)]
pub struct GR_IMAGE_SUBRESOURCE_RANGE {
    pub aspect: GR_ENUM,
    pub baseMipLevel: GR_UINT,
    pub mipLevels: GR_UINT,
    pub baseArraySlice: GR_UINT,
    pub arraySize: GR_UINT,
}

#[repr(C)]
pub struct GR_IMAGE_STATE_TRANSITION {
    pub image: GR_IMAGE,
    pub oldState: GR_ENUM,
    pub newState: GR_ENUM,
    pub subresourceRange: GR_IMAGE_SUBRESOURCE_RANGE,
}

#[repr(C)]
pub struct GR_FENCE_CREATE_INFO {
    pub flags: GR_FLAGS,
}

#[repr(C)]
pub struct GR_SHADER_CREATE_INFO {
    pub codeSize: GR_SIZE,
    pub pCode: *const GR_VOID,
    pub flags: GR_FLAGS,
}

#[repr(C)]
pub struct GR_MEMORY_HEAP_PROPERTIES {
    pub heapMemoryType: GR_ENUM,
    pub heapSize: GR_GPU_SIZE,
    pub pageSize: GR_GPU_SIZE,
    pub flags: GR_FLAGS,
    pub gpuReadPerfRating: GR_FLOAT,
    pub gpuWritePerfRating: GR_FLOAT,
    pub cpuReadPerfRating: GR_FLOAT,
    pub cpuWritePerfRating: GR_FLOAT,
}

#[repr(C)]
pub struct GR_MEMORY_ALLOC_INFO {
    pub size: GR_GPU_SIZE,
    pub alignment: GR_GPU_SIZE,
    pub flags: GR_FLAGS,
    pub heapCount: GR_UINT,
    pub heaps: [GR_UINT; GR_MAX_MEMORY_HEAPS],
    pub memPriority: GR_ENUM,
}

#[repr(C)]
pub struct GR_MEMORY_STATE_TRANSITION {
    pub mem: GR_GPU_MEMORY,
    pub oldState: GR_ENUM,
    pub newState: GR_ENUM,
    pub offset: GR_GPU_SIZE,
    pub regionSize: GR_GPU_SIZE,
}

#[repr(C)]
pub struct GR_RASTER_STATE_CREATE_INFO {
    pub fillMode: GR_ENUM,
    pub cullMode: GR_ENUM,
    pub frontFace: GR_ENUM,
    pub depthBias: GR_INT,
    pub depthBiasClamp: GR_FLOAT,
    pub slopeScaledDepthBias: GR_FLOAT,
}

#[repr(C)]
pub struct GR_VIEWPORT_STATE_CREATE_INFO {
    pub viewportCount: GR_UINT,
    pub scissorEnable: GR_BOOL,
    pub viewports: [GR_VIEWPORT; GR_MAX_VIEWPORTS],
    pub scissors: [GR_RECT; GR_MAX_VIEWPORTS],
}

#[repr(C)]
pub struct GR_VIEWPORT {
    pub originX: GR_FLOAT,
    pub originY: GR_FLOAT,
    pub width: GR_FLOAT,
    pub height: GR_FLOAT,
    pub minDepth: GR_FLOAT,
    pub maxDepth: GR_FLOAT,
}

#[repr(C)]
pub struct GR_RECT {
    pub offset: GR_OFFSET2D,
    pub extent: GR_EXTENT2D,
}

#[repr(C)]
pub struct GR_OFFSET2D {
    pub x: GR_INT,
    pub y: GR_INT,
}

#[repr(C)]
pub struct GR_COLOR_BLEND_STATE_CREATE_INFO {
    pub target: [GR_COLOR_TARGET_BLEND_STATE; GR_MAX_COLOR_TARGETS],
    pub blendConst: [GR_FLOAT; 4],
}

#[repr(C)]
pub struct GR_COLOR_TARGET_BLEND_STATE {
    pub blendEnable: GR_BOOL,
    pub srcBlendColor: GR_ENUM,
    pub destBlendColor: GR_ENUM,
    pub blendFuncColor: GR_ENUM,
    pub srcBlendAlpha: GR_ENUM,
    pub destBlendAlpha: GR_ENUM,
    pub blendFuncAlpha: GR_ENUM,
}

#[repr(C)]
pub struct GR_DEPTH_STENCIL_STATE_CREATE_INFO {
    pub depthEnable: GR_BOOL,
    pub depthWriteEnable: GR_BOOL,
    pub depthFunc: GR_ENUM,
    pub depthBoundsEnable: GR_BOOL,
    pub minDepth: GR_FLOAT,
    pub maxDepth: GR_FLOAT,
    pub stencilEnable: GR_BOOL,
    pub stencilReadMask: GR_UINT8,
    pub stencilWriteMask: GR_UINT8,
    pub front: GR_DEPTH_STENCIL_OP,
    pub back: GR_DEPTH_STENCIL_OP,
}

#[repr(C)]
pub struct GR_DEPTH_STENCIL_OP {
    pub stencilFailOp: GR_ENUM,
    pub stencilPassOp: GR_ENUM,
    pub stencilDepthFailOp: GR_ENUM,
    pub stencilFunc: GR_ENUM,
    pub stencilRef: GR_UINT8,
}

#[repr(C)]
pub struct GR_MSAA_STATE_CREATE_INFO {
    pub samples: GR_UINT,
    pub sampleMask: GR_SAMPLE_MASK,
}

extern {
    pub fn grInitAndEnumerateGpus(pAppInfo: *const GR_APPLICATION_INFO,
                                  pAllocCb: *const GR_ALLOC_CALLBACKS, pGpuCount: *mut GR_UINT,
                                  gpus: *mut GR_PHYSICAL_GPU) -> GR_RESULT;

    pub fn grCreateDevice(gpu: GR_PHYSICAL_GPU, pCreateInfo: *const GR_DEVICE_CREATE_INFO,
                          pDevice: *mut GR_DEVICE) -> GR_RESULT;


    pub fn grWsiWinCreatePresentableImage(device: GR_DEVICE, pCreateInfo: *const
                                          GR_WSI_WIN_PRESENTABLE_IMAGE_CREATE_INFO,
                                          pImage: *mut GR_IMAGE, pMem: *mut GR_GPU_MEMORY)
                                          -> GR_RESULT;

    pub fn grWsiWinGetDisplays(device: GR_DEVICE, pDisplayCount: *mut GR_UINT,
                               pDisplayList: *mut GR_WSI_WIN_DISPLAY) -> GR_RESULT;

    pub fn grWsiWinQueuePresent(queue: GR_QUEUE, pPresentInfo: *const GR_WSI_WIN_PRESENT_INFO)
                                -> GR_RESULT;

    pub fn grGetDeviceQueue(device: GR_DEVICE, queueType: GR_ENUM, queueId: GR_UINT,
                            queue: *mut GR_QUEUE) -> GR_RESULT;

    pub fn grCmdClearColorImage(cmdBuffer: GR_CMD_BUFFER, image: GR_IMAGE, color: *const GR_FLOAT,
                                 rangeCount: GR_UINT, pRanges: *const GR_IMAGE_SUBRESOURCE_RANGE);

    pub fn grCreateCommandBuffer(device: GR_DEVICE, pCreateInfo: *const GR_CMD_BUFFER_CREATE_INFO,
                                 pCmdBuffer: *mut GR_CMD_BUFFER) -> GR_RESULT;

    pub fn grBeginCommandBuffer(cmdBuffer: GR_CMD_BUFFER, flags: GR_FLAGS) -> GR_RESULT;

    pub fn grEndCommandBuffer(cmdBuffer: GR_CMD_BUFFER) -> GR_RESULT;

    pub fn grQueueSubmit(queue: GR_QUEUE, cmdBufferCount: GR_UINT,
                         pCmdBuffers: *const GR_CMD_BUFFER, memRefCount: GR_UINT,
                         pMemRefs: *const GR_MEMORY_REF, fence: GR_FENCE) -> GR_RESULT;

    pub fn grDbgRegisterMsgCallback(pfnMsgCallback: GR_DBG_MSG_CALLBACK_FUNCTION,
                                    pUserData: *mut GR_VOID) -> GR_RESULT;

    pub fn grCmdPrepareImages(cmdBuffer: GR_CMD_BUFFER, transitionCount: GR_UINT,
                              pStateTransitions: *const GR_IMAGE_STATE_TRANSITION);

    pub fn grGetExtensionSupport(gpu: GR_PHYSICAL_GPU, pExtName: *const GR_CHAR) -> GR_RESULT;

    pub fn grDestroyObject(object: GR_OBJECT) -> GR_RESULT;

    pub fn grDestroyDevice(device: GR_DEVICE) -> GR_RESULT;

    pub fn grCreateFence(device: GR_DEVICE, pCreateInfo: *const GR_FENCE_CREATE_INFO,
                         pFence: *mut GR_FENCE) -> GR_RESULT;

    pub fn grWaitForFences(device: GR_DEVICE, fenceCount: GR_UINT, pFences: *const GR_FENCE,
                           waitAll: GR_BOOL, timeout: GR_FLOAT) -> GR_RESULT;

    pub fn grCreateShader(device: GR_DEVICE, pCreateInfo: *const GR_SHADER_CREATE_INFO,
                          pShader: *mut GR_SHADER) -> GR_RESULT;

    pub fn grGetMemoryHeapCount(device: GR_DEVICE, pCount: *mut GR_UINT) -> GR_RESULT;

    pub fn grGetMemoryHeapInfo(device: GR_DEVICE, heapId: GR_UINT, infoType: GR_ENUM,
                               pDataSize: *mut GR_SIZE, pData: *mut GR_VOID) -> GR_RESULT;

    pub fn grAllocMemory(device: GR_DEVICE, pAllocInfo: *const GR_MEMORY_ALLOC_INFO,
                         pMem: *mut GR_GPU_MEMORY) -> GR_RESULT;

    pub fn grFreeMemory(mem: GR_GPU_MEMORY) -> GR_RESULT;

    pub fn grMapMemory(mem: GR_GPU_MEMORY, flags: GR_FLAGS, ppData: *mut *mut GR_VOID) -> GR_RESULT;

    pub fn grUnmapMemory(mem: GR_GPU_MEMORY) -> GR_RESULT;

    pub fn grCmdPrepareMemoryRegions(cmdBuffer: GR_CMD_BUFFER, transitionCount: GR_UINT,
                                     pStateTransitions: *const GR_MEMORY_STATE_TRANSITION);

    pub fn grCreateRasterState(device: GR_DEVICE, pCreateInfo: *const GR_RASTER_STATE_CREATE_INFO,
                               pState: *const GR_RASTER_STATE_OBJECT) -> GR_RESULT;

    pub fn grCreateViewportState(device: GR_DEVICE,
                                 pCreateInfo: *const GR_VIEWPORT_STATE_CREATE_INFO,
                                 pState: *const GR_VIEWPORT_STATE_OBJECT) -> GR_RESULT;

    pub fn grCreateColorBlendState(device: GR_DEVICE,
                                   pCreateInfo: *const GR_COLOR_BLEND_STATE_CREATE_INFO,
                                   pState: *const GR_COLOR_BLEND_STATE_OBJECT) -> GR_RESULT;

    pub fn grCreateDepthStencilState(device: GR_DEVICE,
                                     pCreateInfo: *const GR_DEPTH_STENCIL_STATE_CREATE_INFO,
                                     pState: *const GR_DEPTH_STENCIL_STATE_OBJECT) -> GR_RESULT;

    pub fn grCreateMsaaState(device: GR_DEVICE, pCreateInfo: *const GR_MSAA_STATE_CREATE_INFO,
                             pState: *const GR_MSAA_STATE_OBJECT) -> GR_RESULT;
}
