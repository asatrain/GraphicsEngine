#[derive(Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub struct Mat3x3 {
    pub content: [[f32; 3]; 3],
}

pub struct Mat4x4 {
    pub content: [[f32; 4]; 4],
}

pub struct Triangle {
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}

pub struct Mesh {
    pub triangles: &'static [Triangle],
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }
}

impl Triangle {
    pub const fn new(p1: Vec3, p2: Vec3, p3: Vec3) -> Triangle {
        Triangle { p1, p2, p3 }
    }
}
