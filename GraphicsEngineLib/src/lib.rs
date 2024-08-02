use crate::game::update_game;
use crate::render::render;

mod render;
mod math;
mod game;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[no_mangle]
pub extern "C" fn update_and_render(width: usize, height: usize, delta_time: f32) -> *mut Pixel {
    let game = update_game(delta_time);
    let mut pixel_buffer = render(width, height, game);

    let pixel_buffer_ptr = pixel_buffer.as_mut_ptr();
    std::mem::forget(pixel_buffer);
    return pixel_buffer_ptr;
}

#[no_mangle]
pub extern "C" fn free_buffer(arr: *mut Pixel, length: usize) {
    if arr.is_null() {
        return;
    }
    unsafe {
        Vec::from_raw_parts(arr, length, length);
    }
}