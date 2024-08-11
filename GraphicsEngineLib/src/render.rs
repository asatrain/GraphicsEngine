use std::cmp::{max, min};
use std::mem::swap;

use crate::game::Scene;
use crate::math::{Mat4x4, Triangle, Vec2};
use crate::Color;

pub struct ScreenSize {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy)]
struct BufferedPixel {
    color: Color,
    depth: f32,
}

struct DepthBuffer {
    buffer: Vec<BufferedPixel>,
    screen_size: ScreenSize,
}

struct Camera {
    fov: f32,
    z_near: f32,
    z_far: f32,
}

impl DepthBuffer {
    fn new(screen_size: ScreenSize, color: Color) -> Self {
        let buffered_pixel = BufferedPixel { color, depth: 1.0 };

        Self {
            buffer: vec![buffered_pixel; screen_size.width * screen_size.height],
            screen_size,
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, pixel: BufferedPixel) {
        let target_y = self.screen_size.height - y as usize;
        let target_x = x as usize;
        let ind = target_y * self.screen_size.width + target_x;
        if pixel.depth >= 0.0 && ind >= 0 && ind < self.buffer.len() {
            let old = self.buffer[ind];
            if pixel.depth <= old.depth {
                self.buffer[ind] = pixel;
            }
        }
    }

    fn to_bitmap(&self) -> Vec<Color> {
        self.buffer.iter().map(|bp| bp.color).collect()
    }
}

impl Camera {
    fn render(&self, buffer: &mut DepthBuffer, scene: &Scene) {
        let screen_size = &buffer.screen_size;
        let aspect_ratio = screen_size.width as f32 / screen_size.height as f32;
        let perspective_mat = self.perspective_mat(aspect_ratio);
        for mesh in &scene.meshes {
            for tr in &mesh.triangles {
                let mut projected_p1 = &perspective_mat * &tr.p1;
                projected_p1.perspective_div();
                let mut projected_p2 = &perspective_mat * &tr.p2;
                projected_p2.perspective_div();
                let mut projected_p3 = &perspective_mat * &tr.p3;
                projected_p3.perspective_div();
                let projected_tr = Triangle::new(projected_p1, projected_p2, projected_p3);
                draw_screen_triangle_with_lines(buffer, &projected_tr);
            }
        }
    }

    fn perspective_mat(&self, aspect_ratio: f32) -> Mat4x4 {
        let mut res = Mat4x4::default();
        let half_fov = self.fov.to_radians() / 2.0;
        let tan_half_fov = half_fov.tan();
        res.content[0][0] = 1.0 / (tan_half_fov * aspect_ratio);
        res.content[1][1] = 1.0 / (tan_half_fov);
        res.content[2][2] = self.z_far / (self.z_far - self.z_near);
        res.content[2][3] = -(self.z_far * self.z_near) / (self.z_far - self.z_near);
        res.content[3][2] = 1.0;
        return res;
    }
}

pub fn render(screen_size: ScreenSize, scene: &Scene) -> Vec<Color> {
    let orange = Color {
        red: 200,
        green: 100,
        blue: 0,
        alpha: 0,
    };
    let mut buffer = DepthBuffer::new(screen_size, orange);

    let camera: Camera = Camera { fov: 100.0, z_near: 1.0, z_far: 1000.0 };
    camera.render(&mut buffer, scene);

    return buffer.to_bitmap();
}

fn draw_screen_triangle_with_lines(buffer: &mut DepthBuffer, tr: &Triangle) {
    let width = buffer.screen_size.width;
    let height = buffer.screen_size.height;

    let screen_y1 = (((tr.p1.y + 1.0) / 2.0) * height as f32) as i32;
    let screen_x1 = (((tr.p1.x + 1.0) / 2.0) * width as f32) as i32;
    let p1 = Vec2::new(screen_x1, screen_y1);

    let screen_y2 = (((tr.p2.y + 1.0) / 2.0) * height as f32) as i32;
    let screen_x2 = (((tr.p2.x + 1.0) / 2.0) * width as f32) as i32;
    let p2 = Vec2::new(screen_x2, screen_y2);

    let screen_y3 = (((tr.p3.y + 1.0) / 2.0) * height as f32) as i32;
    let screen_x3 = (((tr.p3.x + 1.0) / 2.0) * width as f32) as i32;
    let p3 = Vec2::new(screen_x3, screen_y3);

    draw_screen_line(buffer, &p1, &p2);
    draw_screen_line(buffer, &p2, &p3);
    draw_screen_line(buffer, &p3, &p1);

    draw_screen_point(buffer, &p1);
    draw_screen_point(buffer, &p2);
    draw_screen_point(buffer, &p3);
}

fn draw_screen_line<'a>(buffer: &mut DepthBuffer,
                        mut p1: &'a Vec2<i32>,
                        mut p2: &'a Vec2<i32>) {
    if p1.x > p2.x {
        swap(&mut p1, &mut p2);
    }
    let dy = p2.y - p1.y;
    let dx = p2.x - p1.x;
    if dy >= 0 && dx >= dy {
        draw_screen_line_q1(buffer, p1, p2);
    } else if dy < 0 && dx >= -dy {
        draw_screen_line_q4(buffer, p1, p2);
    } else if dy >= 0 && dx < dy {
        draw_screen_line_q2(buffer, p1, p2);
    } else {
        draw_screen_line_q3(buffer, p1, p2);
    }
}

fn draw_screen_line_q1(buffer: &mut DepthBuffer,
                       p1: &Vec2<i32>,
                       p2: &Vec2<i32>) {
    let pixel = BufferedPixel {
        color: Color {
            red: 100,
            green: 0,
            blue: 0,
            alpha: 0,
        },
        depth: 0.0,
    };

    let dy = p2.y - p1.y;
    let dx = p2.x - p1.x;
    let mut p = 2 * dy - dx;
    let mut x = p1.x;
    let mut y = p1.y;
    while x <= p2.x {
        buffer.set_pixel(x, y, pixel);
        x += 1;
        if p < 0 {
            p += 2 * dy;
        } else {
            p += 2 * (dy - dx);
            y += 1;
        }
    }
}

fn draw_screen_line_q2(buffer: &mut DepthBuffer,
                       p1: &Vec2<i32>,
                       p2: &Vec2<i32>) {
    let pixel = BufferedPixel {
        color: Color {
            red: 200,
            green: 0,
            blue: 200,
            alpha: 0,
        },
        depth: 0.0,
    };

    let dy = p2.y - p1.y;
    let dx = p2.x - p1.x;
    let mut p = 2 * dx - dy;
    let mut x = p1.x;
    let mut y = p1.y;
    while y <= p2.y {
        buffer.set_pixel(x, y, pixel);
        y += 1;
        if p < 0 {
            p += 2 * dx;
        } else {
            p += 2 * (dx - dy);
            x += 1;
        }
    }
}

fn draw_screen_line_q3(buffer: &mut DepthBuffer,
                       p1: &Vec2<i32>,
                       p2: &Vec2<i32>) {
    let pixel = BufferedPixel {
        color: Color {
            red: 0,
            green: 190,
            blue: 255,
            alpha: 0,
        },
        depth: 0.0,
    };

    let dy = p1.y - p2.y;
    let dx = p2.x - p1.x;
    let mut p = 2 * dx - dy;
    let mut x = p1.x;
    let mut y = p1.y;
    while y >= p2.y {
        buffer.set_pixel(x, y, pixel);
        y -= 1;
        if p < 0 {
            p += 2 * dx;
        } else {
            p += 2 * (dx - dy);
            x += 1;
        }
    }
}

fn draw_screen_line_q4(buffer: &mut DepthBuffer,
                       p1: &Vec2<i32>,
                       p2: &Vec2<i32>) {
    let pixel = BufferedPixel {
        color: Color {
            red: 0,
            green: 100,
            blue: 0,
            alpha: 0,
        },
        depth: 0.0,
    };

    let dy = p1.y - p2.y;
    let dx = p2.x - p1.x;
    let mut p = 2 * dy - dx;
    let mut x = p1.x;
    let mut y = p1.y;
    while x <= p2.x {
        buffer.set_pixel(x, y, pixel);
        x += 1;
        if p < 0 {
            p += 2 * dy;
        } else {
            p += 2 * (dy - dx);
            y -= 1;
        }
    }
}

fn draw_screen_point(buffer: &mut DepthBuffer,
                     p: &Vec2<i32>) {
    let pixel = BufferedPixel {
        color: Color {
            red: 100,
            green: 100,
            blue: 100,
            alpha: 0,
        },
        depth: 0.0,
    };

    let point_radius = 3;
    let left = max(0, p.x - point_radius);
    let right = min(buffer.screen_size.width as i32, p.x + point_radius);
    let top = max(0, p.y - point_radius);
    let bot = min(buffer.screen_size.height as i32, p.y + point_radius);

    for screen_x in left..=right {
        for screen_y in top..=bot {
            buffer.set_pixel(screen_x, screen_y, pixel);
        }
    }
}

