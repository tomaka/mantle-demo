extern crate "mantle-sys" as ffi;

use std::mem;
use std::ptr;

fn main() {
    for v in (0 .. 0xffffusize) {
        println!("{}", v);

        unsafe {
            let mut appinfos: ffi::GR_APPLICATION_INFO = mem::zeroed();
            //appInfo.apiVersion = GR_API_VERSION;
            appinfos.apiVersion = v as u32;      // FIXME: total guess

            let mut gpus = [ptr::null() ; ffi::GR_MAX_PHYSICAL_GPUS];
            let mut gpus_count = 0;

            let result = ffi::grInitAndEnumerateGpus(&appinfos, ptr::null(), &mut gpus_count,
                                                     gpus.as_mut_ptr());

            println!("{:?} {}", result, gpus_count);
        }
    }

    println!("Hello, world!");
}
