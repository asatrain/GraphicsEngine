#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[no_mangle]
pub extern "C" fn updateAndRender(width: usize, height: usize, delta_time: f32) -> *mut Pixel {
    let orange = Pixel {
        red: 200,
        green: 100,
        blue: 0,
        alpha: 0,
    };
    let mut pixel_buffer: Vec<Pixel> = vec![orange; width * height];
    let pixel_buffer_ptr = pixel_buffer.as_mut_ptr();
    std::mem::forget(pixel_buffer);
    return pixel_buffer_ptr;
}

#[no_mangle]
pub extern "C" fn free_array(arr: *mut Pixel, length: usize) {
    if arr.is_null() {
        return;
    }
    unsafe {
        Vec::from_raw_parts(arr, length, length);
    }
}