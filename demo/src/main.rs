extern crate "mantle-sys" as ffi;

use std::mem;
use std::ptr;

fn main() {
    unsafe {
        let mut appinfos: ffi::GR_APPLICATION_INFO = mem::zeroed();
        //appInfo.apiVersion = GR_API_VERSION;
        appinfos.apiVersion = 0x1000;      // FIXME: total guess

        let mut gpus = [ptr::null() ; ffi::GR_MAX_PHYSICAL_GPUS];
        let mut gpus_count = 0;

        let result = ffi::grInitAndEnumerateGpus(&appinfos, ptr::null(), &mut gpus_count,
                                                 gpus.as_mut_ptr());
        println!("{} {}", result, gpus_count);
    }

    println!("Hello, world!");
}
