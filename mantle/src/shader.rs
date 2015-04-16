use ffi;
use error;

use std::mem;
use std::sync::Arc;

use device::Device;
use MantleObject;

pub struct Shader {
    shader: ffi::GR_SHADER,
}

impl Shader {
    pub fn new(device: &Arc<Device>, source: &[u8]) -> Arc<Shader> {
        let shader = unsafe {
            let infos = ffi::GR_SHADER_CREATE_INFO {
                codeSize: source.len() as ffi::GR_SIZE,
                pCode: source.as_ptr() as *const _,
                flags: 0,
            };

            let mut shader = mem::uninitialized();
            let res = ffi::grCreateShader(*device.get_id(), &infos, &mut shader);
            error::check_result(res).unwrap();
            shader
        };

        Arc::new(Shader {
            shader: shader,
        })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            error::check_result(ffi::grDestroyObject(self.shader)).unwrap();
        }
    }
}
