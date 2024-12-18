use crate::game::{update_scene, Scene};
use crate::render::{render, ScreenSize};
use std::ptr::null_mut;

mod render;
mod math;
mod game;
mod assets;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UserInput {
    w_pressed: bool,
    a_pressed: bool,
    s_pressed: bool,
    d_pressed: bool,
    q_pressed: bool,
    e_pressed: bool,
    shift_pressed: bool,
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
            a_pressed: false,
            s_pressed: false,
            d_pressed: false,
            q_pressed: false,
            e_pressed: false,
            shift_pressed: false,
        };
        create_scene();
        for _ in 0..1 {
            update_and_render(200, 100, input, 0.5);
        }
    }
}