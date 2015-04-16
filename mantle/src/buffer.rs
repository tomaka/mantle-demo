use ffi;
use error;

use device::Device;
use MantleObject;

use std::sync::Arc;
use std::mem;

pub struct Buffer {
    memory: ffi::GR_GPU_MEMORY,
}

impl Buffer {
    pub fn empty(device: &Arc<Device>, size: usize) -> Arc<Buffer> {
        let heap = &device.get_heaps()[0];

        let infos = ffi::GR_MEMORY_ALLOC_INFO {
            size: (heap.page_size * (1 + (size - 1) / heap.page_size)) as ffi::GR_GPU_SIZE,
            alignment: 0,
            flags: 0,
            heapCount: 1,
            heaps: [heap.id, 0, 0, 0, 0, 0, 0, 0],
            memPriority: ffi::GR_MEMORY_PRIORITY_NORMAL,
        };

        let mem = unsafe {
            let mut mem = mem::uninitialized();
            error::check_result(ffi::grAllocMemory(*device.get_id(), &infos, &mut mem)).unwrap();
            mem
        };

        Arc::new(Buffer {
            memory: mem,
        })
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            error::check_result(ffi::grFreeMemory(self.memory)).unwrap();
        }
    }
}
