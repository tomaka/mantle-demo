use ffi;

#[derive(Debug, Copy)]
pub enum MantleError {
    Unknown,
    Unavailable,
    InitializationFailed,
    OutOfMemory,
    OutOfGpuMemory,
    DeviceAlreadyCreated,
    DeviceLost,
    InvalidPointer,
    InvalidValue,
    InvalidHandle,
    InvalidOrdinal,
    InvalidMemorySize,
    InvalidExtension,
    InvalidFlags,
    InvalidAlignment,
    InvalidFormat,
    InvalidImage,
    InvalidDescriptorSetData,
    InvalidQueueType,
    InvalidObjectType,
    UnsupportedShaderIlVersion,
    BadShaderCode,
    BadPipelineData,
    TooManyMemoryReferences,
    NotMappable,
    MemoryMapFailed,
    MemoryUnmapFailed,
    IncompatibleDevice,
    IncompatibleDriver,
    IncompleteCommandBuffer,
    BuildingCommandBuffer,
    MemoryNotBound,
    IncompatibleQueue,
    NotShareable,
}

pub fn check_result(value: ffi::GR_RESULT) -> Result<(), MantleError> {
    match value {
        ffi::GR_RESULT::GR_SUCCESS => Ok(()),
        c => panic!("{:?}", c)      // FIXME: return error
    }
}
