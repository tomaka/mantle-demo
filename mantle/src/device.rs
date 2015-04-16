/*! There are two structs that each represent a Mantle context: `MainDevice` and `Device`.

Some Mantle functions are thread-safe, while some others are not. The former are
available for both `MainDevice` and `Device`, and the latter are available only on `MainDevice`.
`MainDevice` does not implement `Send` and `Sync`.
*/
use error;
use ffi;

use command_buffer::CommandBuffer;
use instance::Gpu;

use CommandBufferExt;
use DeviceExt;
use MantleObject;

use std::mem;
use std::sync::Arc;

/// Represents a Mantle context.
pub struct Device {
    device: ffi::GR_DEVICE,
    queue: ffi::GR_QUEUE,
    heaps: Vec<HeapInfos>,
}

/// Information about a memory heap.
pub struct HeapInfos {
    pub id: ffi::GR_UINT,
    pub size: usize,
    pub page_size: usize,
}

pub struct Fence<'a> {
    device: &'a Device,
    fence: ffi::GR_FENCE,
    wait: bool,
}

impl Device {
    /// Builds a new Mantle context for the given GPU.
    pub fn new(gpu: &Gpu) -> Arc<Device> {
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

        let heaps = unsafe {
            let mut heaps_count = mem::uninitialized();
            error::check_result(ffi::grGetMemoryHeapCount(device, &mut heaps_count)).unwrap();

            let mut heaps = Vec::with_capacity(heaps_count as usize);

            for id in 0 .. heaps_count {
                let mut data_size = mem::size_of::<ffi::GR_MEMORY_HEAP_PROPERTIES>() as ffi::GR_SIZE;
                let mut infos: ffi::GR_MEMORY_HEAP_PROPERTIES = mem::uninitialized();
                let infos_ptr: *mut ffi::GR_MEMORY_HEAP_PROPERTIES = &mut infos;

                let res = ffi::grGetMemoryHeapInfo(device, id,
                                                   ffi::GR_INFO_TYPE_MEMORY_HEAP_PROPERTIES,
                                                   &mut data_size, infos_ptr as *mut _);
                error::check_result(res).unwrap();

                heaps.push(HeapInfos {
                    id: id,
                    size: infos.heapSize as usize,
                    page_size: infos.pageSize as usize,
                });
            }

            heaps
        };

        Arc::new(Device {
            device: device,
            queue: queue,
            heaps: heaps,
        })
    }

    pub fn submit(&self, commands: &Arc<CommandBuffer>) -> Fence {
        let fence = unsafe {
            let fence_infos = ffi::GR_FENCE_CREATE_INFO {
                flags: 0,
            };

            let mut fence = mem::uninitialized();
            error::check_result(ffi::grCreateFence(self.device, &fence_infos,
                                                   &mut fence)).unwrap();
            fence
        };

        let mem = commands.build_memory_refs();
        let commands = [*commands.get_id()];

        error::check_result(unsafe {
            ffi::grQueueSubmit(self.queue, 1, commands.as_ptr(), mem.len() as u32,
                               mem.as_ptr(), fence)
        }).unwrap();

        Fence {
            device: self,
            fence: fence,
            wait: true,
        }
    }

    pub fn get_heaps(&self) -> &[HeapInfos] {
        &self.heaps
    }
}

impl MantleObject for Device {
    type Id = ffi::GR_DEVICE;

    fn get_id(&self) -> &ffi::GR_DEVICE {
        &self.device
    }
}

impl DeviceExt for Device {
    fn get_queue(&self) -> ffi::GR_QUEUE {
        self.queue
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        let res = unsafe { ffi::grDestroyDevice(self.device) };
        error::check_result(res).unwrap();
    }
}

impl<'a> Drop for Fence<'a> {
    fn drop(&mut self) {
        if self.wait {
            unsafe {
                let res = ffi::grWaitForFences(self.device.device, 1, [self.fence].as_ptr(),
                                               true, 5.0);
                error::check_result(res).unwrap();
            }
        }

        unsafe {
            error::check_result(ffi::grDestroyObject(self.fence)).unwrap();
        }
    }
}
