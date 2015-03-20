extern crate "kernel32-sys" as kernel32;
extern crate "gdi32-sys" as gdi32;
extern crate "user32-sys" as user32;
extern crate "mantle-sys" as ffi;
extern crate winapi;

extern crate mantle;

use std::mem;
use std::ptr;
use std::ffi::CStr;

fn new_main() {
    let device = mantle::MainDevice::new(&mantle::get_gpus().nth(0).unwrap());

    let window = unsafe { create_window() };

    let (width, height) = unsafe {
        let mut rect: winapi::RECT = mem::uninitialized();
        user32::GetClientRect(window, &mut rect);

        ((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32)
    };

    let image = mantle::presentable_image::PresentableImage::new(&device, width, height);

    loop {
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

fn main() {
    unsafe {
        check_result(ffi::grDbgRegisterMsgCallback(debug_callback, ptr::null_mut())).unwrap();
    }

    let window = unsafe { create_window() };

    let (width, height) = unsafe {
        let mut rect: winapi::RECT = mem::uninitialized();
        user32::GetClientRect(window, &mut rect);

        ((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32)
    };

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

    unsafe {
        let ext: &'static [u8] = b"GR_WSI_WINDOWS\0";
        check_result(ffi::grGetExtensionSupport(gpus[0], ext.as_ptr() as *const i8))
                     .unwrap();
    }

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
            maxValidationLevel: ffi::GR_VALIDATION_LEVEL::GR_VALIDATION_LEVEL_4,
            flags: ffi::GR_DEVICE_CREATE_VALIDATION,
        };

        let mut device: ffi::GR_DEVICE = 0;
        unsafe {
            check_result(ffi::grCreateDevice(gpus[0], &device_info, &mut device)).unwrap();
        }
        device
    };

    let queue = unsafe {
        let mut queue = mem::uninitialized();
        ffi::grGetDeviceQueue(device, ffi::GR_QUEUE_UNIVERSAL, 0, &mut queue);
        queue
    };

    let (image, image_mem) = unsafe {
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

        let mut image = mem::uninitialized();
        let mut mem = mem::uninitialized();
        check_result(ffi::grWsiWinCreatePresentableImage(device, &infos, &mut image,
                                                         &mut mem)).unwrap();
        (image, mem)
    };

    unsafe {
        let infos = ffi::GR_CMD_BUFFER_CREATE_INFO {
            queueType: ffi::GR_QUEUE_UNIVERSAL,
            flags: 0,
        };

        let mut cmd_buffer = mem::uninitialized();
        check_result(ffi::grCreateCommandBuffer(device, &infos, &mut cmd_buffer)).unwrap();

        check_result(ffi::grBeginCommandBuffer(cmd_buffer, 0)).unwrap();

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

        check_result(ffi::grEndCommandBuffer(cmd_buffer)).unwrap();

        let mem = ffi::GR_MEMORY_REF {
            mem: image_mem,
            flags: 0,
        };

        check_result(ffi::grQueueSubmit(queue, 1, &cmd_buffer, 1, &mem, 0)).unwrap();
    }

    let cmd_buffer = unsafe {
        let infos = ffi::GR_CMD_BUFFER_CREATE_INFO {
            queueType: ffi::GR_QUEUE_UNIVERSAL,
            flags: 0,
        };

        let mut cmd = mem::uninitialized();
        check_result(ffi::grCreateCommandBuffer(device, &infos, &mut cmd)).unwrap();
        cmd
    };

    unsafe {
        check_result(ffi::grBeginCommandBuffer(cmd_buffer, 0)).unwrap();

        let transition = ffi::GR_IMAGE_STATE_TRANSITION {
            image: image,
            oldState: ffi::GR_WSI_WIN_IMAGE_STATE_PRESENT_WINDOWED,
            newState: ffi::GR_IMAGE_STATE_CLEAR,
            subresourceRange: ffi::GR_IMAGE_SUBRESOURCE_RANGE {
                aspect: ffi::GR_IMAGE_ASPECT_COLOR,
                baseMipLevel: 0,
                mipLevels: 1,
                baseArraySlice: 0,
                arraySize: 1,
            },
        };

        ffi::grCmdPrepareImages(cmd_buffer, 1, &transition);


        let color = [0.0, 0.0, 1.0, 1.0];
        let range = ffi::GR_IMAGE_SUBRESOURCE_RANGE {
            aspect: ffi::GR_IMAGE_ASPECT_COLOR,
            baseMipLevel: 0,
            mipLevels: 1,
            baseArraySlice: 0,
            arraySize: 1,
        };

        ffi::grCmdClearColorImage(cmd_buffer, image, color.as_ptr(), 1, &range);

        let transition = ffi::GR_IMAGE_STATE_TRANSITION {
            image: image,
            oldState: ffi::GR_IMAGE_STATE_CLEAR,
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


        check_result(ffi::grEndCommandBuffer(cmd_buffer)).unwrap();
    }

    loop {
        unsafe {
            let mem = ffi::GR_MEMORY_REF {
                mem: image_mem,
                flags: 0,
            };

            check_result(ffi::grQueueSubmit(queue, 1, &cmd_buffer, 1, &mem, 0)).unwrap();
        }

        unsafe {
            let infos = ffi::GR_WSI_WIN_PRESENT_INFO {
                hWndDest: window,
                srcImage: image,
                presentMode: ffi::GR_WSI_WIN_PRESENT_MODE_WINDOWED,
                presentInterval: 0,
                flags: 0,
            };

            check_result(ffi::grWsiWinQueuePresent(queue, &infos)).unwrap();
        }

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

fn check_result(value: ffi::GR_RESULT) -> Result<(), String> {
    match value {
        ffi::GR_RESULT::GR_SUCCESS => Ok(()),
        c => Err(format!("{:?}", c))
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

extern "system" fn callback(window: winapi::HWND, msg: winapi::UINT,
                            wparam: winapi::WPARAM, lparam: winapi::LPARAM) -> winapi::LRESULT
{
    unsafe { user32::DefWindowProcW(window, msg, wparam, lparam) }
}

extern "stdcall" fn debug_callback(_msg_type: ffi::GR_ENUM, _validation_level: ffi::GR_ENUM,
                                   _src_object: ffi::GR_BASE_OBJECT, _location: ffi::GR_SIZE,
                                   _msg_code: ffi::GR_ENUM, msg: *const ffi::GR_CHAR,
                                   _user_data: *mut ffi::GR_VOID)
{
    unsafe {
        let msg = CStr::from_ptr(msg);
        println!("Mantle debug message: {}", String::from_utf8(msg.to_bytes().to_vec()).unwrap());
    }
}
