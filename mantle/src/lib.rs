#![feature(optin_builtin_traits)]

extern crate kernel32;
extern crate gdi32;
extern crate user32;
extern crate mantle_sys as ffi;
extern crate winapi;

pub use device::{MainDevice, Device};
pub use instance::get_gpus;

pub mod command_buffer;
pub mod device;
pub mod error;
pub mod presentable_image;

mod instance;

trait MantleObject {
    type Id;

    fn get_id(&self) -> &Self::Id;
}

trait QueuesProvider {
    fn get_queue(&self) -> ffi::GR_QUEUE;
}

/// Extra internal methods on command buffers.
trait CommandBufferExt {
    fn build_memory_refs(&self) -> Vec<ffi::GR_MEMORY_REF>;
}

/// Extra internal methods on images.
trait ImageExt {
    fn get_image(&self) -> ffi::GR_IMAGE;
    fn get_mem(&self) -> ffi::GR_GPU_MEMORY;
}
