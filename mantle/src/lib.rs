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
pub mod macros;
pub mod presentable_image;

mod instance;

trait MantleObject {
    type Id;

    fn get_id(&self) -> &Self::Id;
}

trait QueuesProvider {
    fn get_queue(&self) -> ffi::GR_QUEUE;
}
