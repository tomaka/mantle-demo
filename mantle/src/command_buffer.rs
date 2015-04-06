use ffi;
use error;

use std::mem;
use std::sync::Arc;

use device::RawDevice;
use device::AsRawDevice;
use CommandBufferExt;
use ImageExt;
use MantleObject;

use presentable_image::PresentableImage;

pub struct CommandBuffer {
    device: Arc<RawDevice>,
    cmd: ffi::GR_CMD_BUFFER,
    memory_refs: Vec<ffi::GR_MEMORY_REF>,
}

pub struct CommandBufferBuilder {
    device: Arc<RawDevice>,
    cmd: Option<ffi::GR_CMD_BUFFER>,
    memory_refs: Vec<ffi::GR_MEMORY_REF>,
}

impl CommandBufferBuilder {
    pub fn new<D: AsRawDevice>(device: &D) -> CommandBufferBuilder {
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

        error::check_result(unsafe { ffi::grBeginCommandBuffer(cmd_buffer, 0) }).unwrap();

        CommandBufferBuilder {
            device: device.as_raw_device().clone(),
            cmd: Some(cmd_buffer),
            memory_refs: Vec::new(),
        }
    }

    pub fn clear_image(mut self, image: &PresentableImage, red: f32, green: f32, blue: f32, alpha: f32) -> CommandBufferBuilder {
        let (image, image_mem) = (image.get_image(), image.get_mem());

        let transition = ffi::GR_IMAGE_STATE_TRANSITION {
            image: image,
            oldState: ffi::GR_WSI_WIN_IMAGE_STATE_PRESENT_WINDOWED,
            newState: ffi::GR_IMAGE_STATE_CLEAR,
            subresourceRange: ffi::GR_IMAGE_SUBRESOURCE_RANGE {
                aspect: ffi::GR_IMAGE_ASPECT_COLOR,
                baseMipLevel: 0,
                mipLevels: 1,
                baseArraySlice: 0,
                arraySize: 1,
            },
        };

        unsafe {
            ffi::grCmdPrepareImages(self.cmd.unwrap(), 1, &transition);
        }

        let color = [red, green, blue, alpha];
        let range = ffi::GR_IMAGE_SUBRESOURCE_RANGE {
            aspect: ffi::GR_IMAGE_ASPECT_COLOR,
            baseMipLevel: 0,
            mipLevels: 1,
            baseArraySlice: 0,
            arraySize: 1,
        };

        unsafe {
            ffi::grCmdClearColorImage(self.cmd.unwrap(), image, color.as_ptr(), 1, &range);
        }

        let transition = ffi::GR_IMAGE_STATE_TRANSITION {
            image: image,
            oldState: ffi::GR_IMAGE_STATE_CLEAR,
            newState: ffi::GR_WSI_WIN_IMAGE_STATE_PRESENT_WINDOWED,
            subresourceRange: ffi::GR_IMAGE_SUBRESOURCE_RANGE {
                aspect: ffi::GR_IMAGE_ASPECT_COLOR,
                baseMipLevel: 0,
                mipLevels: 1,
                baseArraySlice: 0,
                arraySize: 1,
            },
        };

        unsafe {
            ffi::grCmdPrepareImages(self.cmd.unwrap(), 1, &transition);
        }

        self.memory_refs.push(ffi::GR_MEMORY_REF {
            mem: image_mem,
            flags: 0,
        });

        self
    }

    pub fn build(mut self) -> CommandBuffer {
        let cmd_buffer = self.cmd.take().unwrap();
        error::check_result(unsafe { ffi::grEndCommandBuffer(cmd_buffer) }).unwrap();

        CommandBuffer {
            device: self.device.clone(),
            cmd: cmd_buffer,
            memory_refs: mem::replace(&mut self.memory_refs, Vec::with_capacity(0)),
        }
    }
}

impl Drop for CommandBufferBuilder {
    fn drop(&mut self) {
        if let Some(cmd) = self.cmd {
            error::check_result(unsafe { ffi::grEndCommandBuffer(cmd) }).unwrap();
            error::check_result(unsafe { ffi::grDestroyObject(cmd) }).unwrap();
        }
    }
}

impl MantleObject for CommandBuffer {
    type Id = ffi::GR_CMD_BUFFER;

    fn get_id(&self) -> &ffi::GR_CMD_BUFFER {
        &self.cmd
    }
}

impl CommandBufferExt for CommandBuffer {
    fn build_memory_refs(&self) -> Vec<ffi::GR_MEMORY_REF> {
        self.memory_refs.clone()
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        error::check_result(unsafe { ffi::grDestroyObject(self.cmd) }).unwrap();
    }
}
