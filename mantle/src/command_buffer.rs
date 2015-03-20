use ffi;
use error;

use std::mem;
use std::sync::Arc;
use std::marker::PhantomData;

use device::RawDevice;
use device::AsRawDevice;
use MantleObject;

pub struct CommandBuffer<L> {
    device: Arc<RawDevice>,
    cmd: ffi::GR_CMD_BUFFER,
    marker: PhantomData<L>,
}

pub trait CommandsList {
    type Input;
    type Output;
}

impl<L> CommandBuffer<L> where L: CommandsList {
    pub fn new<D: AsRawDevice>(device: &D) -> CommandBuffer<L> {
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
            marker: PhantomData,
        }
    }
}
