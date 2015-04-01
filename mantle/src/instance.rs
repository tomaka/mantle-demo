use error;
use ffi;

use MantleObject;

use std::mem;
use std::ptr;
use std::ffi::CStr;
use std::slice::Iter;
use std::iter::Take;
use std::sync::{Once, ONCE_INIT};

static mut GPUS: [ffi::GR_PHYSICAL_GPU; ffi::GR_MAX_PHYSICAL_GPUS] = [0; ffi::GR_MAX_PHYSICAL_GPUS];
static mut GPUS_COUNT: ffi::GR_UINT = 0;

#[derive(Debug, Copy, Clone)]
pub struct Gpu {
    gpu: ffi::GR_PHYSICAL_GPU,
}

impl MantleObject for Gpu {
    type Id = ffi::GR_PHYSICAL_GPU;

    fn get_id(&self) -> &ffi::GR_PHYSICAL_GPU {
        &self.gpu
    }
}

// FIXME: cache the list of gpus
pub fn get_gpus() -> GpusIterator {
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| {
        unsafe {
            error::check_result(ffi::grDbgRegisterMsgCallback(debug_callback,
                                ptr::null_mut())).unwrap();

            let mut appinfos: ffi::GR_APPLICATION_INFO = mem::zeroed();
            appinfos.apiVersion = ffi::GR_API_VERSION;

            let result = ffi::grInitAndEnumerateGpus(&appinfos, ptr::null(), &mut GPUS_COUNT,
                                                     GPUS.as_mut_ptr());
            error::check_result(result).unwrap();
        }
    });

    GpusIterator {
        iter: unsafe { GPUS.iter().take(GPUS_COUNT as usize) },
    }
}

pub struct GpusIterator {
    iter: Take<Iter<'static, ffi::GR_PHYSICAL_GPU>>,
}

impl Iterator for GpusIterator {
    type Item = Gpu;

    fn next(&mut self) -> Option<Gpu> {
        self.iter.next().map(|&g| Gpu { gpu: g })
    }
}

unsafe extern "stdcall" fn debug_callback(_msg_type: ffi::GR_ENUM, _validation_level: ffi::GR_ENUM,
                                          _src_object: ffi::GR_BASE_OBJECT, _location: ffi::GR_SIZE,
                                          _msg_code: ffi::GR_ENUM, msg: *const ffi::GR_CHAR,
                                          _user_data: *mut ffi::GR_VOID)
{
    unsafe {
        let msg = CStr::from_ptr(msg);
        println!("Mantle debug message: {}", String::from_utf8(msg.to_bytes().to_vec()).unwrap());
    }
}
