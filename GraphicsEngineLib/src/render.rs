use std::cmp::{max, min};
use std::mem::swap;

use crate::game::Game;
use crate::math::{Triangle, Vec2};
use crate::Pixel;

trait PixelBuffer {
    fn set_pixel(&mut self, screen_width: usize, screen_height: usize, x: i32, y: i32, pixel: Pixel);
}

impl PixelBuffer for Vec<Pixel> {
    fn set_pixel(&mut self, screen_width: usize, screen_height: usize, x: i32, y: i32, pixel: Pixel) {
        let target_y = screen_height - y as usize;
        let target_x = x as usize;
        self[target_y * screen_width + target_x] = pixel;
    }
}

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
            draw_screen_triangle(screen_width, screen_height, &mut pixel_buffer, tr);
        }
    }
    return pixel_buffer;
}

fn draw_screen_triangle(screen_width: usize, screen_height: usize, pixel_buffer: &mut Vec<Pixel>, tr: &Triangle) {
    let screen_y1 = (tr.p1.y * screen_height as f32) as i32;
    let screen_x1 = (tr.p1.x * screen_width as f32) as i32;
    let p1 = Vec2::new(screen_x1, screen_y1);

    let screen_y2 = (tr.p2.y * screen_height as f32) as i32;
    let screen_x2 = (tr.p2.x * screen_width as f32) as i32;
    let p2 = Vec2::new(screen_x2, screen_y2);

    let screen_y3 = (tr.p3.y * screen_height as f32) as i32;
    let screen_x3 = (tr.p3.x * screen_width as f32) as i32;
    let p3 = Vec2::new(screen_x3, screen_y3);

    draw_screen_line(screen_width, screen_height, pixel_buffer, &p1, &p2);
    draw_screen_line(screen_width, screen_height, pixel_buffer, &p2, &p3);
    draw_screen_line(screen_width, screen_height, pixel_buffer, &p3, &p1);

    draw_screen_point(screen_width, screen_height, pixel_buffer, &p1);
    draw_screen_point(screen_width, screen_height, pixel_buffer, &p2);
    draw_screen_point(screen_width, screen_height, pixel_buffer, &p3);
}

fn draw_screen_line<'a>(screen_width: usize, screen_height: usize, pixel_buffer: &mut Vec<Pixel>,
                        mut p1: &'a Vec2<i32>,
                        mut p2: &'a Vec2<i32>) {
    if p1.x > p2.x {
        swap(&mut p1, &mut p2);
    }
    let dy = p2.y - p1.y;
    let dx = p2.x - p1.x;
    if dy >= 0 && dx >= dy {
        draw_screen_line_q1(screen_width, screen_height, pixel_buffer, p1, p2);
    } else if dy < 0 && dx >= -dy {
        draw_screen_line_q4(screen_width, screen_height, pixel_buffer, p1, p2);
    } else if dy >= 0 && dx < dy {
        draw_screen_line_q2(screen_width, screen_height, pixel_buffer, p1, p2);
    } else {
        draw_screen_line_q3(screen_width, screen_height, pixel_buffer, p1, p2);
    }
}

fn draw_screen_line_q1(screen_width: usize, screen_height: usize, pixel_buffer: &mut Vec<Pixel>,
                       p1: &Vec2<i32>,
                       p2: &Vec2<i32>) {
    let color = Pixel {
        red: 100,
        green: 0,
        blue: 0,
        alpha: 0,
    };

    let dy = p2.y - p1.y;
    let dx = p2.x - p1.x;
    let mut p = 2 * dy - dx;
    let mut x = p1.x;
    let mut y = p1.y;
    while x <= p2.x {
        pixel_buffer.set_pixel(screen_width, screen_height, x, y, color);
        x += 1;
        if p < 0 {
            p += 2 * dy;
        } else {
            p += 2 * (dy - dx);
            y += 1;
        }
    }
}

fn draw_screen_line_q2(screen_width: usize, screen_height: usize, pixel_buffer: &mut Vec<Pixel>,
                       p1: &Vec2<i32>,
                       p2: &Vec2<i32>) {
    let color = Pixel {
        red: 200,
        green: 0,
        blue: 200,
        alpha: 0,
    };

    let dy = p2.y - p1.y;
    let dx = p2.x - p1.x;
    let mut p = 2 * dx - dy;
    let mut x = p1.x;
    let mut y = p1.y;
    while y <= p2.y {
        pixel_buffer.set_pixel(screen_width, screen_height, x, y, color);
        y += 1;
        if p < 0 {
            p += 2 * dx;
        } else {
            p += 2 * (dx - dy);
            x += 1;
        }
    }
}

fn draw_screen_line_q3(screen_width: usize, screen_height: usize, pixel_buffer: &mut Vec<Pixel>,
                       p1: &Vec2<i32>,
                       p2: &Vec2<i32>) {
    let color = Pixel {
        red: 0,
        green: 191,
        blue: 250,
        alpha: 0,
    };

    let dy = p1.y - p2.y;
    let dx = p2.x - p1.x;
    let mut p = 2 * dx - dy;
    let mut x = p1.x;
    let mut y = p1.y;
    while y >= p2.y {
        pixel_buffer.set_pixel(screen_width, screen_height, x, y, color);
        y -= 1;
        if p < 0 {
            p += 2 * dx;
        } else {
            p += 2 * (dx - dy);
            x += 1;
        }
    }
}

fn draw_screen_line_q4(screen_width: usize, screen_height: usize, pixel_buffer: &mut Vec<Pixel>,
                       p1: &Vec2<i32>,
                       p2: &Vec2<i32>) {
    let color = Pixel {
        red: 0,
        green: 100,
        blue: 0,
        alpha: 0,
    };

    let dy = p1.y - p2.y;
    let dx = p2.x - p1.x;
    let mut p = 2 * dy - dx;
    let mut x = p1.x;
    let mut y = p1.y;
    while x <= p2.x {
        pixel_buffer.set_pixel(screen_width, screen_height, x, y, color);
        x += 1;
        if p < 0 {
            p += 2 * dy;
        } else {
            p += 2 * (dy - dx);
            y -= 1;
        }
    }
}

fn draw_screen_point(screen_width: usize, screen_height: usize, pixel_buffer: &mut Vec<Pixel>,
                     p: &Vec2<i32>) {
    let color = Pixel {
        red: 100,
        green: 100,
        blue: 100,
        alpha: 0,
    };

    let point_radius = 3;
    let left = max(0, p.x - point_radius);
    let right = min(screen_width as i32, p.x + point_radius);
    let top = max(0, p.y - point_radius);
    let bot = min(screen_height as i32, p.y + point_radius);

    for screen_x in left..=right {
        for screen_y in top..=bot {
            pixel_buffer.set_pixel(screen_width, screen_height, screen_x, screen_y, color);
        }
    }
}

