use std::ptr::null_mut;

use crate::game::{Scene, update_scene};
use crate::render::{render, ScreenSize};

mod render;
mod math;
mod game;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UserInput {
    w_pressed: bool,
    a_pressed: bool,
    s_pressed: bool,
    d_pressed: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

static mut SCENE: *mut Scene = null_mut();

#[no_mangle]
pub extern "C" fn create_scene() {
    unsafe {
        SCENE = Box::into_raw(Box::new(Scene::new()));
    }
}

#[no_mangle]
pub extern "C" fn update_and_render(width: i32, height: i32, user_input: UserInput, delta_time: f32) -> *mut Color {
    let mut scene = unsafe {
        Box::from_raw(SCENE)
    };
    update_scene(&mut scene, &user_input, delta_time);
    let screen_size = ScreenSize { width, height };
    let mut bitmap = render(screen_size, &scene);

    let bitmap_ptr = bitmap.as_mut_ptr();
    std::mem::forget(bitmap);
    std::mem::forget(scene);
    return bitmap_ptr;
}

#[no_mangle]
pub extern "C" fn free_bitmap(arr: *mut Color, length: usize) {
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
        let input = UserInput {
            w_pressed: false,
            a_pressed: true,
            s_pressed: false,
            d_pressed: false,
        };
        create_scene();
        update_and_render(1920, 1080, input, 2.5);
    }
}