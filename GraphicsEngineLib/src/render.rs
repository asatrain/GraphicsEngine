use std::cmp::{max, min};
use std::mem::swap;

use crate::game_logic::Game;
use crate::math::Triangle;
use crate::Pixel;

pub fn render(screen_width: usize, screen_height: usize, game: &Game) -> Vec<Pixel> {
    let orange = Pixel {
        red: 200,
        green: 100,
        blue: 0,
        alpha: 0,
    };
    let mut pixel_buffer: Vec<Pixel> = vec![orange; screen_width * screen_height];

    for mesh in game.meshes {
        for tr in mesh.triangles {
            draw_triangle(screen_width, screen_height, &mut pixel_buffer, tr);
        }
    }
    return pixel_buffer;
}

fn draw_triangle(screen_width: usize, screen_height: usize, pixel_buffer: &mut Vec<Pixel>, tr: &Triangle) {
    let screen_y1 = (tr.p1.y * screen_height as f32) as i32;
    let screen_x1 = (tr.p1.x * screen_width as f32) as i32;

    let screen_y2 = (tr.p2.y * screen_height as f32) as i32;
    let screen_x2 = (tr.p2.x * screen_width as f32) as i32;

    let screen_y3 = (tr.p3.y * screen_height as f32) as i32;
    let screen_x3 = (tr.p3.x * screen_width as f32) as i32;

    draw_screen_line(screen_width, screen_height, pixel_buffer, screen_x1, screen_y1, screen_x2, screen_y2);
    draw_screen_line(screen_width, screen_height, pixel_buffer, screen_x1, screen_y1, screen_x3, screen_y3);
    draw_screen_line(screen_width, screen_height, pixel_buffer, screen_x2, screen_y2, screen_x3, screen_y3);

    draw_screen_point(screen_width, screen_height, pixel_buffer, screen_x1, screen_y1);
    draw_screen_point(screen_width, screen_height, pixel_buffer, screen_x2, screen_y2);
    draw_screen_point(screen_width, screen_height, pixel_buffer, screen_x3, screen_y3);
}

fn draw_screen_line(screen_width: usize, screen_height: usize, pixel_buffer: &mut Vec<Pixel>,
                    mut x1: i32, mut y1: i32,
                    mut x2: i32, mut y2: i32) {
    if x1 > x2 {
        swap(&mut x1, &mut x2);
        swap(&mut y1, &mut y2);
    }

    let black = Pixel {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    let width = x2 - x1;
    let height = y2 - y1;
    if width > height
    {
        let tg = height as f32 / width as f32;
        for x in x1..=x2 {
            let y = y1 + ((x - x1) as f32 * tg) as i32;
            pixel_buffer[y as usize * screen_width + x as usize] = black;
        }
    } else {
        let tg = width as f32 / height as f32;
        for y in y1..=y2 {
            let x = x1 + ((y - y1) as f32 * tg) as i32;
            pixel_buffer[y as usize * screen_width + x as usize] = black;
        }
    }
}

fn draw_screen_point(width: usize, height: usize, pixel_buffer: &mut Vec<Pixel>,
                     x: i32, y: i32) {
    let green = Pixel {
        red: 0,
        green: 200,
        blue: 0,
        alpha: 0,
    };

    let point_radius = 5;
    let left = max(0, x - point_radius) as usize;
    let right = min(width as i32, x + point_radius) as usize;
    let top = max(0, y - point_radius) as usize;
    let bot = min(height as i32, y + point_radius) as usize;

    for screen_x in left..=right {
        for screen_y in top..=bot {
            pixel_buffer[screen_y * width + screen_x] = green;
        }
    }
}

