use crate::game::update_scene;
use crate::render::{render, ScreenSize};

mod render;
mod math;
mod game;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[no_mangle]
pub extern "C" fn update_and_render(width: usize, height: usize, delta_time: f32) -> *mut Color {
    let scene = update_scene(delta_time);
    let screen_size = ScreenSize { width, height };
    let mut bitmap = render(screen_size, &scene);

    let bitmap_ptr = bitmap.as_mut_ptr();
    std::mem::forget(bitmap);
    return bitmap_ptr;
}

#[no_mangle]
pub extern "C" fn free_buffer(arr: *mut Color, length: usize) {
    if arr.is_null() {
        return;
    }
    unsafe {
        Vec::from_raw_parts(arr, length, length);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        update_and_render(500, 400, 2.5);
    }
}