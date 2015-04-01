use ffi;
use error;

use std::mem;
use std::sync::Arc;

use device::RawDevice;
use device::AsRawDevice;
use MantleObject;

pub struct CommandBuffer {
    device: Arc<RawDevice>,
    cmd: ffi::GR_CMD_BUFFER,
}

impl CommandBuffer {
    pub fn new<D: AsRawDevice>(device: &D) -> CommandBuffer {
        let infos = ffi::GR_CMD_BUFFER_CREATE_INFO {
            queueType: ffi::GR_QUEUE_UNIVERSAL,
            flags: 0,
        };

        let cmd_buffer = unsafe {
            let mut cmd = mem::uninitialized();
            error::check_result(ffi::grCreateCommandBuffer(*device.as_raw_device().get_id(),
                                                           &infos, &mut cmd)).unwrap();
            cmd
        };

        CommandBuffer {
            device: device.as_raw_device().clone(),
            cmd: cmd_buffer,
        }
    }


}
