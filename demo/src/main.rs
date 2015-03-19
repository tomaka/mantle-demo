extern crate "mantle-sys" as ffi;

use std::mem;
use std::ptr;

fn main() {
    let gpus = unsafe {
        let mut appinfos: ffi::GR_APPLICATION_INFO = mem::zeroed();
        appinfos.apiVersion = ffi::GR_API_VERSION;

        let mut gpus = Vec::with_capacity(ffi::GR_MAX_PHYSICAL_GPUS);
        let mut gpus_count = 2;

        let result = ffi::grInitAndEnumerateGpus(&appinfos, ptr::null(), &mut gpus_count,
                                                 gpus.as_mut_ptr());
        check_result(result).unwrap();

        gpus.set_len(gpus_count as usize);
        gpus
    };

    let device = {
        let queue_info = ffi::GR_DEVICE_QUEUE_CREATE_INFO {
            queueType: ffi::GR_QUEUE_TYPE::GR_QUEUE_UNIVERSAL,
            queueCount: 1,
        };

        let device_info = ffi::GR_DEVICE_CREATE_INFO {
            queueRecordCount: 1,
            pRequestedQueues: &queue_info,
            extensionCount: 1,
            ppEnabledExtensionNames: unsafe { mem::transmute(&b"GR_WSI_WINDOWS\0") },
            maxValidationLevel: ffi::GR_VALIDATION_LEVEL::GR_VALIDATION_LEVEL_0,
            flags: 0,
        };

        let mut device: ffi::GR_DEVICE = 0;
        unsafe {
            check_result(ffi::grCreateDevice(gpus[0], &device_info, &mut device)).unwrap();
        }
        device
    };
}

fn check_result(value: ffi::GR_RESULT) -> Result<(), String> {
    match value {
        ffi::GR_RESULT::GR_SUCCESS => Ok(()),
        c => Err(format!("{:?}", c))
    }
}

