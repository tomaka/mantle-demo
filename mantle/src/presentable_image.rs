use ffi;
use error;
use winapi;

use std::mem;
use std::ptr;

use device::MainDevice;
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
