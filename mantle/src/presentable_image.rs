use ffi;
use error;
use winapi;

use std::mem;
use std::ptr;

use device::MainDevice;
use ImageExt;
use MantleObject;
use QueuesProvider;

pub struct PresentableImage {
    device: MainDevice,
    image: ffi::GR_IMAGE,
    mem: ffi::GR_GPU_MEMORY,
}

impl PresentableImage {
    pub fn new(device: &MainDevice, width: u32, height: u32) -> PresentableImage {
        let infos = ffi::GR_WSI_WIN_PRESENTABLE_IMAGE_CREATE_INFO {
            format: ffi::GR_FORMAT {
                channelFormat: 8,
                numericFormat: 1,
            },
            usage: ffi::GR_IMAGE_USAGE_COLOR_TARGET,
            extent: ffi::GR_EXTENT2D {
                width: width as i32,
                height: height as i32,
            },
            display: 0,
            flags: 0,
        };

        let (image, image_mem) = unsafe {
            let mut image = mem::uninitialized();
            let mut mem = mem::uninitialized();
            error::check_result(ffi::grWsiWinCreatePresentableImage(*device.get_id(), &infos,
                                                                    &mut image, &mut mem)).unwrap();
            (image, mem)
        };

        // switching to `GR_WSI_WIN_IMAGE_STATE_PRESENT_WINDOWED` state
        unsafe {
            let infos = ffi::GR_CMD_BUFFER_CREATE_INFO {
                queueType: ffi::GR_QUEUE_UNIVERSAL,
                flags: 0,
            };

            let mut cmd_buffer = mem::uninitialized();
            error::check_result(ffi::grCreateCommandBuffer(*device.get_id(), &infos, &mut cmd_buffer)).unwrap();

            error::check_result(ffi::grBeginCommandBuffer(cmd_buffer, ffi::GR_CMD_BUFFER_OPTIMIZE_ONE_TIME_SUBMIT)).unwrap();

            let transition = ffi::GR_IMAGE_STATE_TRANSITION {
                image: image,
                oldState: ffi::GR_IMAGE_STATE_UNINITIALIZED,
                newState: ffi::GR_WSI_WIN_IMAGE_STATE_PRESENT_WINDOWED,
                subresourceRange: ffi::GR_IMAGE_SUBRESOURCE_RANGE {
                    aspect: ffi::GR_IMAGE_ASPECT_COLOR,
                    baseMipLevel: 0,
                    mipLevels: 1,
                    baseArraySlice: 0,
                    arraySize: 1,
                },
            };

            ffi::grCmdPrepareImages(cmd_buffer, 1, &transition);

            error::check_result(ffi::grEndCommandBuffer(cmd_buffer)).unwrap();

            let mem = ffi::GR_MEMORY_REF {
                mem: image_mem,
                flags: 0,
            };

            error::check_result(ffi::grQueueSubmit(device.get_queue(), 1, &cmd_buffer, 1, &mem, 0)).unwrap();
        }

        PresentableImage {
            device: device.clone(),
            image: image,
            mem: image_mem,
        }
    }

    /// Presents the image to the given window.
    pub fn present(&self, window: winapi::HWND) {
        unsafe {
            let infos = ffi::GR_WSI_WIN_PRESENT_INFO {
                hWndDest: window,
                srcImage: self.image,
                presentMode: ffi::GR_WSI_WIN_PRESENT_MODE_WINDOWED,
                presentInterval: 0,
                flags: 0,
            };

            error::check_result(ffi::grWsiWinQueuePresent(self.device.get_queue(), &infos)).unwrap();
        }
    }
}

impl ImageExt for PresentableImage {
    fn get_normal_state(&self) -> ffi::GR_ENUM {
        ffi::GR_WSI_WIN_IMAGE_STATE_PRESENT_WINDOWED
    }

    fn get_image(&self) -> ffi::GR_IMAGE {
        self.image
    }

    fn get_mem(&self) -> ffi::GR_GPU_MEMORY {
        self.mem
    }
}
