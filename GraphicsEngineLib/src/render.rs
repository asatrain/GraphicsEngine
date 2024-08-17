use std::cmp::{max, min};
use std::mem::swap;

use crate::game::Scene;
use crate::math::{Mat4x4, Triangle, Vec2, Vec3, Vec4};
use crate::Color;

pub struct ScreenSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy)]
struct DeepPixel {
    color: Color,
    depth: f32,
}

struct DepthBuffer {
    buffer: Vec<DeepPixel>,
    screen_size: ScreenSize,
}

pub struct Camera {
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub position: Vec3,
    pub rotation: Vec3,
}

impl DepthBuffer {
    fn new(screen_size: ScreenSize, color: Color) -> Self {
        let background_pixel = DeepPixel { color, depth: 1.0 };
        Self {
            buffer: vec![background_pixel; (screen_size.width * screen_size.height) as usize],
            screen_size,
        }
    }

    fn screen_space_to_pixel_pos(&self, mut screen_space_x: f32, mut screen_space_y: f32) -> Vec2<i32> {
        screen_space_x = (screen_space_x + 1.0) / 2.0;
        screen_space_y = (screen_space_y + 1.0) / 2.0;
        let pixel_x = (screen_space_x * self.screen_size.width as f32).round() as i32;
        let pixel_y = (screen_space_y * self.screen_size.height as f32).round() as i32;
        return Vec2::new(pixel_x, pixel_y);
    }

    fn set_screen_space_pixel(&mut self, screen_space_x: f32, screen_space_y: f32, pixel: DeepPixel) {
        if pixel.depth < 0.0 {
            return;
        }

        let pixel_pos = self.screen_space_to_pixel_pos(screen_space_x, screen_space_y);
        self.set_pixel(pixel_pos.x, pixel_pos.y, pixel);
    }

    fn set_pixel(&mut self, x: i32, y: i32, pixel: DeepPixel) {
        if x < 0 || x >= self.screen_size.width ||
            y < 0 || y >= self.screen_size.height
        {
            return;
        }

        let y = self.screen_size.height - 1 - y;
        let ind = (y * self.screen_size.width + x) as usize;
        let old = self.buffer[ind];
        if pixel.depth <= old.depth {
            self.buffer[ind] = pixel;
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
        for object in &scene.objects {
            for tr in &object.mesh.triangles {
                let mut tr = &Mat4x4::rotation(&object.rotation) * tr;
                tr *= &Mat4x4::translation(&object.position);

                let camera_negative_pos = -&scene.camera.position;
                tr *= &Mat4x4::translation(&camera_negative_pos);
                let camera_negative_rotation = -&scene.camera.rotation;
                tr *= &Mat4x4::rotation(&camera_negative_rotation);

                let mut projected_p1 = &perspective_mat * &tr.p1;
                projected_p1.perspective_div();
                let mut projected_p2 = &perspective_mat * &tr.p2;
                projected_p2.perspective_div();
                let mut projected_p3 = &perspective_mat * &tr.p3;
                projected_p3.perspective_div();
                let projected_tr = Triangle::new(projected_p1, projected_p2, projected_p3);
                draw_screen_triangle(buffer, &projected_tr);
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
        res.content[2][3] = -self.z_far * self.z_near / (self.z_far - self.z_near);
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
    scene.camera.render(&mut buffer, scene);

    return buffer.to_bitmap();
}

fn draw_screen_triangle(buffer: &mut DepthBuffer, tr: &Triangle) {
    draw_screen_line(buffer, &tr.p1, &tr.p2);
    draw_screen_line(buffer, &tr.p2, &tr.p3);
    draw_screen_line(buffer, &tr.p3, &tr.p1);

    draw_screen_point(buffer, &tr.p1);
    draw_screen_point(buffer, &tr.p2);
    draw_screen_point(buffer, &tr.p3);
}

fn draw_screen_line<'a>(buffer: &mut DepthBuffer,
                        mut p1: &'a Vec4,
                        mut p2: &'a Vec4) {
    if p1.x > p2.x {
        swap(&mut p1, &mut p2);
    }
    let (p1_pos, p2_pos) =
        (buffer.screen_space_to_pixel_pos(p1.x, p1.y), buffer.screen_space_to_pixel_pos(p2.x, p2.y));
    let dy = (p2_pos.y - p1_pos.y) as f32;
    let dx = (p2_pos.x - p1_pos.x) as f32;
    if dy >= 0.0 && dx >= dy {
        draw_screen_line_q1(buffer, p1, p2);
    } else if dy < 0.0 && dx >= -dy {
        draw_screen_line_q4(buffer, p1, p2);
    } else if dy >= 0.0 && dx < dy {
        draw_screen_line_q2(buffer, p1, p2);
    } else {
        draw_screen_line_q3(buffer, p1, p2);
    }
}

fn draw_screen_line_q1(buffer: &mut DepthBuffer,
                       p1: &Vec4,
                       p2: &Vec4) {
    let mut pixel = DeepPixel {
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
    let dz = p2.z - p1.z;
    let (p1_pos, p2_pos) =
        (buffer.screen_space_to_pixel_pos(p1.x, p1.y), buffer.screen_space_to_pixel_pos(p2.x, p2.y));
    let pixel_dx = (p2_pos.x - p1_pos.x) as f32;
    let mut screen_x = p1.x;
    let mut screen_y = p1.y;
    let mut screen_z = p1.z;
    for _x in p1_pos.x..=p2_pos.x {
        pixel.depth = screen_z;
        buffer.set_screen_space_pixel(screen_x, screen_y, pixel);
        screen_x += dx / pixel_dx;
        screen_y += dy / pixel_dx;
        screen_z += dz / pixel_dx;
    }
}

fn draw_screen_line_q2(buffer: &mut DepthBuffer,
                       p1: &Vec4,
                       p2: &Vec4) {
    let mut pixel = DeepPixel {
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
    let dz = p2.z - p1.z;
    let (p1_pos, p2_pos) =
        (buffer.screen_space_to_pixel_pos(p1.x, p1.y), buffer.screen_space_to_pixel_pos(p2.x, p2.y));
    let pixel_dy = (p2_pos.y - p1_pos.y) as f32;
    let mut screen_x = p1.x;
    let mut screen_y = p1.y;
    let mut screen_z = p1.z;
    for _y in p1_pos.y..=p2_pos.y {
        pixel.depth = screen_z;
        buffer.set_screen_space_pixel(screen_x, screen_y, pixel);
        screen_x += dx / pixel_dy;
        screen_y += dy / pixel_dy;
        screen_z += dz / pixel_dy;
    }
}

fn draw_screen_line_q3(buffer: &mut DepthBuffer,
                       p1: &Vec4,
                       p2: &Vec4) {
    let mut pixel = DeepPixel {
        color: Color {
            red: 0,
            green: 190,
            blue: 255,
            alpha: 0,
        },
        depth: 0.0,
    };

    let dy = p2.y - p1.y;
    let dx = p2.x - p1.x;
    let dz = p2.z - p1.z;
    let (p1_pos, p2_pos) =
        (buffer.screen_space_to_pixel_pos(p1.x, p1.y), buffer.screen_space_to_pixel_pos(p2.x, p2.y));
    let pixel_dy = (p1_pos.y - p2_pos.y) as f32;
    let mut screen_x = p1.x;
    let mut screen_y = p1.y;
    let mut screen_z = p1.z;
    for _y in p2_pos.y..=p1_pos.y {
        pixel.depth = screen_z;
        buffer.set_screen_space_pixel(screen_x, screen_y, pixel);
        screen_x += dx / pixel_dy;
        screen_y += dy / pixel_dy;
        screen_z += dz / pixel_dy;
    }
}

fn draw_screen_line_q4(buffer: &mut DepthBuffer,
                       p1: &Vec4,
                       p2: &Vec4) {
    let mut pixel = DeepPixel {
        color: Color {
            red: 0,
            green: 100,
            blue: 0,
            alpha: 0,
        },
        depth: 0.0,
    };

    let dy = p2.y - p1.y;
    let dx = p2.x - p1.x;
    let dz = p2.z - p1.z;
    let (p1_pos, p2_pos) =
        (buffer.screen_space_to_pixel_pos(p1.x, p1.y), buffer.screen_space_to_pixel_pos(p2.x, p2.y));
    let pixel_dx = (p2_pos.x - p1_pos.x) as f32;
    let mut screen_x = p1.x;
    let mut screen_y = p1.y;
    let mut screen_z = p1.z;
    for _x in p1_pos.x..=p2_pos.x {
        pixel.depth = screen_z;
        buffer.set_screen_space_pixel(screen_x, screen_y, pixel);
        screen_x += dx / pixel_dx;
        screen_y += dy / pixel_dx;
        screen_z += dz / pixel_dx;
    }
}

fn draw_screen_point(buffer: &mut DepthBuffer,
                     p: &Vec4) {
    let pixel = DeepPixel {
        color: Color {
            red: 100,
            green: 100,
            blue: 100,
            alpha: 0,
        },
        depth: p.z,
    };

    let pixel_pos = buffer.screen_space_to_pixel_pos(p.x, p.y);
    let point_radius = 3;
    let left = max(0, pixel_pos.x as i32 - point_radius);
    let right = min(buffer.screen_size.width, pixel_pos.x + point_radius);
    let bot = max(0, pixel_pos.y as i32 - point_radius);
    let top = min(buffer.screen_size.height, pixel_pos.y + point_radius);

    for screen_x in left..=right {
        for screen_y in bot..=top {
            buffer.set_pixel(screen_x, screen_y, pixel);
        }
    }
}

