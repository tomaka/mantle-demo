#![feature(collections)]

extern crate kernel32;
extern crate gdi32;
extern crate user32;
extern crate mantle_sys as ffi;
extern crate winapi;

extern crate mantle;

use std::mem;
use std::ptr;

fn main() {
    let device = mantle::Device::new(&mantle::get_gpus().nth(0).unwrap());

    let window = unsafe { create_window() };

    let (width, height) = unsafe {
        let mut rect: winapi::RECT = mem::uninitialized();
        user32::GetClientRect(window, &mut rect);

        ((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32)
    };

    let image = mantle::presentable_image::PresentableImage::new(&device, width, height);

    let clear_command = mantle::command_buffer::CommandBufferBuilder::new(&device)
                            .clear_image(&image, 0.0, 1.0, 0.0, 1.0)
                            .build();

    let vertex_shader = mantle::shader::Shader::new(&device, include_bytes!("vs.bin"));
    let fragment_shader = mantle::shader::Shader::new(&device, include_bytes!("ps.bin"));

    let vertex_buffer = {
        let buffer = mantle::buffer::Buffer::empty(&device, 24 * mem::size_of::<f32>());

        {
            let mut mapping = buffer.map();

            mapping[0] =  0.0; mapping[1] =  0.5; mapping[ 2] = 0.0; mapping[ 3] = 1.0;
            mapping[4] =  0.5; mapping[5] = -0.5; mapping[ 6] = 0.0; mapping[ 7] = 1.0;
            mapping[8] = -0.5; mapping[9] = -0.5; mapping[10] = 0.0; mapping[11] = 1.0;

            mapping[12] = 1.0; mapping[13] = 0.0; mapping[14] = 0.0; mapping[15] = 1.0;
            mapping[16] = 0.0; mapping[17] = 1.0; mapping[18] = 0.0; mapping[19] = 1.0;
            mapping[20] = 0.0; mapping[21] = 0.0; mapping[22] = 1.0; mapping[23] = 1.0;
        }

        buffer
    };

    loop {
        device.submit(&clear_command);
        image.present(window);

        unsafe {
            let mut msg = mem::uninitialized();
            if user32::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                break;
            }

            user32::TranslateMessage(&msg);
            user32::DispatchMessageW(&msg);
        }
    }
}

unsafe fn create_window() -> winapi::HWND {
    let class_name = register_window_class();

    let title: Vec<u16> = vec![b'M' as u16, b'a' as u16, b'n' as u16, b't' as u16,
                               b'l' as u16, b'e' as u16, 0];

    user32::CreateWindowExW(winapi::WS_EX_APPWINDOW | winapi::WS_EX_WINDOWEDGE, class_name.as_ptr(),
                            title.as_ptr() as winapi::LPCWSTR,
                            winapi::WS_OVERLAPPEDWINDOW | winapi::WS_CLIPSIBLINGS |
                            winapi::WS_VISIBLE,
                            winapi::CW_USEDEFAULT, winapi::CW_USEDEFAULT,
                            winapi::CW_USEDEFAULT, winapi::CW_USEDEFAULT,
                            ptr::null_mut(), ptr::null_mut(),
                            kernel32::GetModuleHandleW(ptr::null()),
                            ptr::null_mut())
}

unsafe fn register_window_class() -> Vec<u16> {
    let class_name: Vec<u16> = "Window Class".utf16_units().chain(Some(0).into_iter())
        .collect::<Vec<u16>>();

    let class = winapi::WNDCLASSEXW {
        cbSize: mem::size_of::<winapi::WNDCLASSEXW>() as winapi::UINT,
        style: winapi::CS_HREDRAW | winapi::CS_VREDRAW | winapi::CS_OWNDC,
        lpfnWndProc: Some(callback),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: kernel32::GetModuleHandleW(ptr::null()),
        hIcon: ptr::null_mut(),
        hCursor: ptr::null_mut(),
        hbrBackground: ptr::null_mut(),
        lpszMenuName: ptr::null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: ptr::null_mut(),
    };

    user32::RegisterClassExW(&class);
    class_name
}

unsafe extern "system" fn callback(window: winapi::HWND, msg: winapi::UINT,
                                   wparam: winapi::WPARAM, lparam: winapi::LPARAM)
                                   -> winapi::LRESULT
{
    user32::DefWindowProcW(window, msg, wparam, lparam)
}
