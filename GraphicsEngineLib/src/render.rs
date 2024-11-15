use crate::game::Scene;
use crate::math::{Lerp, Mat4x4, Plane, Triangle, Vec2, Vec3, Vec4};
use crate::Color;
use std::cmp::{max, min};
use std::mem::swap;

static BACKGROUND_COLOR: Color = Color {
    red: 200,
    green: 50,
    blue: 0,
    alpha: 0,
};

static BLACK_COLOR: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 0,
};

static MODEL_COLOR: Color = Color {
    red: 200,
    green: 200,
    blue: 0,
    alpha: 0,
};

static WIREFRAME_LINE_COLOR: Color = Color {
    red: 50,
    green: 50,
    blue: 50,
    alpha: 0,
};

static WIREFRAME_POINT_COLOR: Color = Color {
    red: 50,
    green: 50,
    blue: 50,
    alpha: 0,
};

pub struct ScreenSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy, Debug)]
struct DeepPixel {
    color: Color,
    depth: f32,
}

struct DepthBuffer {
    buffer: Vec<DeepPixel>,
    screen_size: ScreenSize,
}

pub struct Camera {
    pub vertical_fov: f32,
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
        let pixel_pos = self.screen_space_to_pixel_pos(screen_space_x, screen_space_y);
        self.set_pixel(pixel_pos.x, pixel_pos.y, pixel);
    }

    fn set_pixel(&mut self, x: i32, y: i32, pixel: DeepPixel) {
        if x < 0 || x >= self.screen_size.width ||
            y < 0 || y >= self.screen_size.height ||
            pixel.depth < 0.0
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
        self.buffer.iter().map(|dp| dp.color).collect()
    }
}

impl Camera {
    fn render(&self, buffer: &mut DepthBuffer, scene: &Scene) {
        let screen_size = &buffer.screen_size;
        let aspect_ratio = screen_size.width as f32 / screen_size.height as f32;
        let perspective_mat = self.perspective_mat(aspect_ratio);
        let mut triangles = vec![];
        for object in &scene.objects {
            for tr in &object.mesh.triangles {
                let mut tr = &Mat4x4::rotation(&object.rotation) * tr;
                tr *= &Mat4x4::translation(&object.position);

                let triangle_normal = (&tr.p2 - &tr.p1).cross(&(&tr.p3 - &tr.p1)).normalized();
                tr.world_normal = Some(triangle_normal);

                let camera_negative_pos = -&scene.camera.position;
                tr *= &Mat4x4::translation(&camera_negative_pos);
                let camera_negative_rotation = -&scene.camera.rotation;
                tr *= &Mat4x4::rotation(&camera_negative_rotation);

                triangles.push(tr);
            }
        }

        let z_near_plane = Vec4::new_plane(Vec4::new3d(0.0, 0.0, self.z_near), Vec4::new3d(0.0, 0.0, 1.0));
        triangles = clip_triangles(triangles, &z_near_plane);
        let z_far_plane = Vec4::new_plane(Vec4::new3d(0.0, 0.0, self.z_far), Vec4::new3d(0.0, 0.0, -1.0));
        triangles = clip_triangles(triangles, &z_far_plane);

        let half_horizontal_fov = ((self.vertical_fov / 2.0).to_radians().tan() * aspect_ratio).atan().to_degrees();
        let hor_normal_rotation_angle = 90.0 - half_horizontal_fov;
        let ver_normal_rotation_angle = 90.0 - self.vertical_fov / 2.0;

        let normal = &Mat4x4::rotation(&Vec3::new(0.0, hor_normal_rotation_angle, 0.0)) * &Vec4::new3d(0.0, 0.0, 1.0);
        let left_plane = Vec4::new_plane(Vec4::new3d(0.0, 0.0, 0.0), normal);
        triangles = clip_triangles(triangles, &left_plane);
        let normal = &Mat4x4::rotation(&Vec3::new(0.0, -hor_normal_rotation_angle, 0.0)) * &Vec4::new3d(0.0, 0.0, 1.0);
        let right_plane = Vec4::new_plane(Vec4::new3d(0.0, 0.0, 0.0), normal);
        triangles = clip_triangles(triangles, &right_plane);
        let normal = &Mat4x4::rotation(&Vec3::new(-ver_normal_rotation_angle, 0.0, 0.0)) * &Vec4::new3d(0.0, 0.0, 1.0);
        let bot_plane = Vec4::new_plane(Vec4::new3d(0.0, 0.0, 0.0), normal);
        triangles = clip_triangles(triangles, &bot_plane);
        let normal = &Mat4x4::rotation(&Vec3::new(ver_normal_rotation_angle, 0.0, 0.0)) * &Vec4::new3d(0.0, 0.0, 1.0);
        let top_plane = Vec4::new_plane(Vec4::new3d(0.0, 0.0, 0.0), normal);
        triangles = clip_triangles(triangles, &top_plane);

        let light_direction = &(&Mat4x4::rotation(&scene.directional_light_rotation) * &Vec4::new3d(0.0, 0.0, 1.0));

        for tr in triangles {
            let alpha = light_direction.dot(&tr.world_normal.unwrap());
            let color = BLACK_COLOR.lerp(&MODEL_COLOR, alpha);

            let mut projected_p1 = &perspective_mat * &tr.p1;
            projected_p1.perspective_div();
            let mut projected_p2 = &perspective_mat * &tr.p2;
            projected_p2.perspective_div();
            let mut projected_p3 = &perspective_mat * &tr.p3;
            projected_p3.perspective_div();
            let projected_tr = Triangle::new(projected_p1, projected_p2, projected_p3);

            draw_triangle(buffer, &projected_tr, color);
            draw_wireframe_triangle(buffer, &projected_tr);
        }
    }

    fn perspective_mat(&self, aspect_ratio: f32) -> Mat4x4 {
        let mut res = Mat4x4::default();
        let half_vertical_fov = self.vertical_fov.to_radians() / 2.0;
        res.content[0][0] = 1.0 / (half_vertical_fov.tan() * aspect_ratio);
        res.content[1][1] = 1.0 / (half_vertical_fov.tan());
        res.content[2][2] = self.z_far / (self.z_far - self.z_near);
        res.content[2][3] = -self.z_far * self.z_near / (self.z_far - self.z_near);
        res.content[3][2] = 1.0;
        return res;
    }
}

pub fn render(screen_size: ScreenSize, scene: &Scene) -> Vec<Color> {
    let mut buffer = DepthBuffer::new(screen_size, BACKGROUND_COLOR);
    scene.camera.render(&mut buffer, scene);

    return buffer.to_bitmap();
}

fn clip_triangles(triangles: Vec<Triangle>, plane: &impl Plane) -> Vec<Triangle> {
    let mut clipped = vec![];
    for tr in triangles {
        let mut res1 = None;
        let mut res2 = None;
        clip_triangle(tr, plane, &mut res1, &mut res2);
        if let Some(tr1) = res1 {
            clipped.push(tr1);
            if let Some(tr2) = res2 {
                clipped.push(tr2);
            }
        }
    }
    return clipped;
}

fn clip_triangle(triangle: Triangle, plane: &impl Plane,
                 res1: &mut Option<Triangle>, res2: &mut Option<Triangle>) {
    let mut inside_count = 0;
    let mut outside_count = 0;
    let mut inside_points = [&triangle.p1; 3];
    let mut outside_points = [&triangle.p1; 3];
    for point in [&triangle.p1, &triangle.p2, &triangle.p3] {
        if plane.is_point_inside(&point) {
            inside_points[inside_count] = point;
            inside_count += 1;
        } else {
            outside_points[outside_count] = point;
            outside_count += 1;
        }
    }

    if inside_count == 0 {
        return;
    } else if inside_count == 3 {
        *res1 = Some(triangle);
    } else if inside_count == 1 {
        let intersection1 = plane.intersect_with_segment(&inside_points[0], &outside_points[0]);
        let intersection2 = plane.intersect_with_segment(&inside_points[0], &outside_points[1]);
        *res1 = Some(Triangle::new_with_normal(inside_points[0].clone(), intersection1, intersection2, triangle.world_normal));
    } else { // if inside_points == 2
        let intersection1 = plane.intersect_with_segment(&inside_points[0], &outside_points[0]);
        let intersection2 = plane.intersect_with_segment(&inside_points[1], &outside_points[0]);
        *res1 = Some(Triangle::new_with_normal(inside_points[0].clone(), inside_points[1].clone(), intersection1.clone(), triangle.world_normal.clone()));
        *res2 = Some(Triangle::new_with_normal(inside_points[1].clone(), intersection2, intersection1, triangle.world_normal.clone()));
    }
}

fn draw_triangle(buffer: &mut DepthBuffer, tr: &Triangle, color: Color) {
    let tr = tr.clockwise();
    let p1 = tr.p1;
    let p2 = tr.p2;
    let p3 = tr.p3;

    let x_min = p1.x.min(p2.x).min(p3.x);
    let x_max = p1.x.max(p2.x).max(p3.x);
    let y_min = p1.y.min(p2.y).min(p3.y);
    let y_max = p1.y.max(p2.y).max(p3.y);

    let pixel_bot_left = buffer.screen_space_to_pixel_pos(x_min, y_min);
    let pixel_top_right = buffer.screen_space_to_pixel_pos(x_max, y_max);

    let tr_area = (&p2 - &p1).cross_len_2d(&(&p3 - &p2));

    let x_delta = (x_max - x_min) / (pixel_top_right.x - pixel_bot_left.x) as f32;
    let y_delta = (y_max - y_min) / (pixel_top_right.y - pixel_bot_left.y) as f32;
    let mut x = x_min;
    let mut y = y_min;
    for _y in pixel_bot_left.y..=pixel_top_right.y {
        for _x in pixel_bot_left.x..=pixel_top_right.x {
            let point = Vec4::new3d(x, y, 0.0);

            let t3 = (&p2 - &p1).cross_len_2d(&(&point - &p1)) / tr_area;
            let t1 = (&p3 - &p2).cross_len_2d(&(&point - &p2)) / tr_area;
            let t2 = (&p1 - &p3).cross_len_2d(&(&point - &p3)) / tr_area;

            if t1 >= 0.0 && t2 >= 0.0 && t3 >= 0.0 {
                let mut z = (t1 * p1.z) + (t2 * p2.z) + (t3 * p3.z);
                z += 0.01 * (1.0 - z) + 0.000001;

                let pixel = DeepPixel { color, depth: z };
                buffer.set_screen_space_pixel(x, y, pixel);
            }

            x += x_delta;
        }
        y += y_delta;
        x = x_min;
    }
}

fn draw_wireframe_triangle(buffer: &mut DepthBuffer, tr: &Triangle) {
    draw_wireframe_line(buffer, &tr.p1, &tr.p2);
    draw_wireframe_line(buffer, &tr.p2, &tr.p3);
    draw_wireframe_line(buffer, &tr.p3, &tr.p1);

    draw_wireframe_point(buffer, &tr.p1);
    draw_wireframe_point(buffer, &tr.p2);
    draw_wireframe_point(buffer, &tr.p3);
}

fn draw_wireframe_line<'a>(buffer: &mut DepthBuffer,
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
        draw_wireframe_line_q1(buffer, p1, p2);
    } else if dy < 0.0 && dx >= -dy {
        draw_wireframe_line_q4(buffer, p1, p2);
    } else if dy >= 0.0 && dx < dy {
        draw_wireframe_line_q2(buffer, p1, p2);
    } else {
        draw_wireframe_line_q3(buffer, p1, p2);
    }
}

fn draw_wireframe_line_q1(buffer: &mut DepthBuffer,
                          p1: &Vec4,
                          p2: &Vec4) {
    let mut pixel = DeepPixel {
        color: WIREFRAME_LINE_COLOR,
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

fn draw_wireframe_line_q2(buffer: &mut DepthBuffer,
                          p1: &Vec4,
                          p2: &Vec4) {
    let mut pixel = DeepPixel {
        color: WIREFRAME_LINE_COLOR,
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

fn draw_wireframe_line_q3(buffer: &mut DepthBuffer,
                          p1: &Vec4,
                          p2: &Vec4) {
    let mut pixel = DeepPixel {
        color: WIREFRAME_LINE_COLOR,
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

fn draw_wireframe_line_q4(buffer: &mut DepthBuffer,
                          p1: &Vec4,
                          p2: &Vec4) {
    let mut pixel = DeepPixel {
        color: WIREFRAME_LINE_COLOR,
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

fn draw_wireframe_point(buffer: &mut DepthBuffer,
                        p: &Vec4) {
    let pixel = DeepPixel {
        color: WIREFRAME_POINT_COLOR,
        depth: p.z,
    };

    let pixel_pos = buffer.screen_space_to_pixel_pos(p.x, p.y);
    let point_radius = 3;
    let left = max(0, pixel_pos.x - point_radius);
    let right = min(buffer.screen_size.width, pixel_pos.x + point_radius);
    let bot = max(0, pixel_pos.y - point_radius);
    let top = min(buffer.screen_size.height, pixel_pos.y + point_radius);

    for screen_x in left..=right {
        for screen_y in bot..=top {
            buffer.set_pixel(screen_x, screen_y, pixel);
        }
    }
}

