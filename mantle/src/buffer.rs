use ffi;
use error;

use device::Device;
use MantleObject;
use DeviceExt;

use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::slice;
use std::mem;

pub struct Buffer {
    memory: ffi::GR_GPU_MEMORY,
    size: usize,
    default_state: ffi::GR_ENUM,
}

pub struct Mapping<'a, T> {
    buffer: &'a Buffer,
    pointer: *mut T,
    size: usize,
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

/*
        // switching to `GR_WSI_WIN_IMAGE_STATE_PRESENT_WINDOWED` state
        unsafe {
            let infos = ffi::GR_CMD_BUFFER_CREATE_INFO {
                queueType: ffi::GR_QUEUE_UNIVERSAL,
                flags: 0,
            };

            let mut cmd_buffer = mem::uninitialized();
            error::check_result(ffi::grCreateCommandBuffer(*device.get_id(), &infos, &mut cmd_buffer)).unwrap();

            error::check_result(ffi::grBeginCommandBuffer(cmd_buffer, ffi::GR_CMD_BUFFER_OPTIMIZE_ONE_TIME_SUBMIT)).unwrap();

            let transition = ffi::GR_MEMORY_STATE_TRANSITION {
                mem: mem,
                oldState: ffi::GR_MEMORY_STATE_DATA_TRANSFER,
                newState: ffi::GR_MEMORY_STATE_GRAPHICS_SHADER_READ_ONLY,
                offset: 0,
                regionSize: size as ffi::GR_GPU_SIZE,
            };

            ffi::grCmdPrepareMemoryRegions(cmd_buffer, 1, &transition);

            error::check_result(ffi::grEndCommandBuffer(cmd_buffer)).unwrap();

            let mem = ffi::GR_MEMORY_REF {
                mem: mem,
                flags: 0,
            };

            error::check_result(ffi::grQueueSubmit(device.get_queue(), 1, &cmd_buffer, 1, &mem, 0)).unwrap();
        }
*/

        Arc::new(Buffer {
            memory: mem,
            size: size,
            default_state: ffi::GR_MEMORY_STATE_DATA_TRANSFER,
        })
    }

    pub fn map<T>(&self) -> Mapping<T> {
        let data = unsafe {
            let mut data = mem::uninitialized();
            error::check_result(ffi::grMapMemory(self.memory, 0, &mut data)).unwrap();
            data
        };

        Mapping {
            buffer: self,
            pointer: data as *mut _,
            size: self.size,
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            error::check_result(ffi::grFreeMemory(self.memory)).unwrap();
        }
    }
}

impl<'a, T> Deref for Mapping<'a, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts_mut(self.pointer, self.size)
        }
    }
}

impl<'a, T> DerefMut for Mapping<'a, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.pointer, self.size)
        }
    }
}

impl<'a, T> Drop for Mapping<'a, T> {
    fn drop(&mut self) {
        unsafe {
            error::check_result(ffi::grUnmapMemory(self.buffer.memory)).unwrap();
        }
    }
}
