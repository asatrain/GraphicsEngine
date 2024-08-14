use std::ops::Mul;

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
    pub p1: Vec4,
    pub p2: Vec4,
    pub p3: Vec4,
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
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

    pub const fn new3d(x: f32, y: f32, z: f32) -> Vec4 {
        Vec4 { x, y, z, w: 1.0 }
    }

    pub fn perspective_div(&mut self) {
        if self.w != 0.0 {
            self.x /= self.w;
            self.y /= self.w;
            self.z /= self.w;
        }
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Vec4::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Default for Mat4x4 {
    fn default() -> Self {
        Self {
            content:
            [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
        }
    }
}

impl Mul for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Mat4x4::default();
        for target_i in 0..4 {
            for target_j in 0..4 {
                for shift in 0..4 {
                    res.content[target_i][target_j] += self.content[target_i][shift] * rhs.content[shift][target_j];
                }
            }
        }
        return res;
    }
}

impl Mul<&Vec4> for &Mat4x4 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        let mut res = Vec4::default();
        let content = self.content;
        res.x = content[0][0] * rhs.x + content[0][1] * rhs.y + content[0][2] * rhs.z + content[0][3] * rhs.w;
        res.y = content[1][0] * rhs.x + content[1][1] * rhs.y + content[1][2] * rhs.z + content[1][3] * rhs.w;
        res.z = content[2][0] * rhs.x + content[2][1] * rhs.y + content[2][2] * rhs.z + content[2][3] * rhs.w;
        res.w = content[3][0] * rhs.x + content[3][1] * rhs.y + content[3][2] * rhs.z + content[3][3] * rhs.w;
        return res;
    }
}

impl Triangle {
    pub const fn new(p1: Vec4, p2: Vec4, p3: Vec4) -> Triangle {
        Triangle { p1, p2, p3 }
    }
}