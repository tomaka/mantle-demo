/*! There are two structs that each represent a Mantle context: `MainDevice` and `Device`.

Some Mantle functions are thread-safe, while some others are not. The former are
available for both `MainDevice` and `Device`, and the latter are available only on `MainDevice`.
`MainDevice` does not implement `Send` and `Sync`.
*/
use error;
use ffi;

use command_buffer::CommandBuffer;
use instance::Gpu;

use MantleObject;
use QueuesProvider;

use std::mem;
use std::sync::Arc;

/// Represents a Mantle context.
#[derive(Clone)]
pub struct MainDevice {
    device: Arc<RawDevice>,
}

impl !Send for MainDevice {}
impl !Sync for MainDevice {}

#[derive(Clone)]
pub struct Device {
    device: Arc<RawDevice>,
}

pub struct RawDevice {
    device: ffi::GR_DEVICE,
    queue: ffi::GR_QUEUE,
}

pub trait AsRawDevice {
    fn as_raw_device(&self) -> &Arc<RawDevice>;
}

impl MainDevice {
    /// Builds a new Mantle context for the given GPU.
    pub fn new(gpu: &Gpu) -> MainDevice {
        unsafe {
            let ext: &'static [u8] = b"GR_WSI_WINDOWS\0";       // FIXME: 
            error::check_result(ffi::grGetExtensionSupport(*gpu.get_id(),
                                                           ext.as_ptr() as *const i8)).unwrap();
        }

        let queue_info = ffi::GR_DEVICE_QUEUE_CREATE_INFO {
            queueType: ffi::GR_QUEUE_TYPE::GR_QUEUE_UNIVERSAL,
            queueCount: 1,
        };

        let device_info = ffi::GR_DEVICE_CREATE_INFO {
            queueRecordCount: 1,
            pRequestedQueues: &queue_info,
            extensionCount: 1,
            ppEnabledExtensionNames: unsafe { mem::transmute(&b"GR_WSI_WINDOWS\0") },   // FIXME: 
            maxValidationLevel: ffi::GR_VALIDATION_LEVEL::GR_VALIDATION_LEVEL_4,
            flags: ffi::GR_DEVICE_CREATE_VALIDATION,
        };

        let device = unsafe {
            let mut device: ffi::GR_DEVICE = 0;
            let res = ffi::grCreateDevice(*gpu.get_id(), &device_info, &mut device);
            error::check_result(res).unwrap();
            device
        };

        let queue = unsafe {
            let mut queue = mem::uninitialized();
            let res = ffi::grGetDeviceQueue(device, ffi::GR_QUEUE_UNIVERSAL, 0, &mut queue);
            error::check_result(res).unwrap();
            queue
        };

        MainDevice {
            device: Arc::new(RawDevice {
                device: device,
                queue: queue,
            }),
        }
    }

    pub fn shared(&self) -> Device {
        Device {
            device: self.device.clone(),
        }
    }

    pub fn submit<L>(&self, commands: &CommandBuffer<L>) {
        unimplemented!();
    }
}

impl MantleObject for MainDevice {
    type Id = ffi::GR_DEVICE;

    fn get_id(&self) -> &ffi::GR_DEVICE {
        &self.device.device
    }
}

impl AsRawDevice for MainDevice {
    fn as_raw_device(&self) -> &Arc<RawDevice> {
        &self.device
    }
}

impl QueuesProvider for MainDevice {
    fn get_queue(&self) -> ffi::GR_QUEUE {
        self.device.queue
    }
}

impl MantleObject for Device {
    type Id = ffi::GR_DEVICE;

    fn get_id(&self) -> &ffi::GR_DEVICE {
        &self.device.device
    }
}

impl AsRawDevice for Device {
    fn as_raw_device(&self) -> &Arc<RawDevice> {
        &self.device
    }
}

impl QueuesProvider for Device {
    fn get_queue(&self) -> ffi::GR_QUEUE {
        self.device.queue
    }
}

impl QueuesProvider for RawDevice {
    fn get_queue(&self) -> ffi::GR_QUEUE {
        self.queue
    }
}

impl AsRawDevice for Arc<RawDevice> {
    fn as_raw_device(&self) -> &Arc<RawDevice> {
        self
    }
}

impl MantleObject for RawDevice {
    type Id = ffi::GR_DEVICE;

    fn get_id(&self) -> &ffi::GR_DEVICE {
        &self.device
    }
}

impl Drop for RawDevice {
    fn drop(&mut self) {
        let res = unsafe { ffi::grDestroyDevice(self.device) };
        error::check_result(res).unwrap();
    }
}
