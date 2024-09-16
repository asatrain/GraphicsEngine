use std::mem::swap;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};

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

#[derive(Debug, Clone)]
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

#[derive(Clone)]
pub struct Triangle {
    pub p1: Vec4,
    pub p2: Vec4,
    pub p3: Vec4,
}

pub trait Plane {
    fn new_plane(point: Vec4, normal: Vec4) -> Self;
    fn is_point_inside(&self, point: &Vec4) -> bool;
    fn intersect_with_segment(&self, p1: &Vec4, p2: &Vec4) -> Vec4;
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

    pub fn dot(&self, rhs: &Vec4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    // pub fn len_2d(&self) -> f32 {
    //     (self.x.powi(2) + self.y.powi(2)).sqrt()
    // }
    //
    // pub fn cross_len_squared(&self, rhs: &Vec4) -> f32 {
    //     (self.y * rhs.z - self.z * rhs.y).powi(2) +
    //         (self.x * rhs.z - self.z * rhs.x).powi(2) +
    //         (self.x * rhs.y - self.y * rhs.x).powi(2)
    // }

    pub fn cross_len_2d(&self, rhs: &Vec4) -> f32 {
        self.x * rhs.y - rhs.x * self.y
    }
}

impl Mat3x3 {
    pub fn rotation(rotation: &Vec3) -> Mat3x3 {
        let (x, y, z) = (rotation.x.to_radians(), rotation.y.to_radians(), rotation.z.to_radians());
        let mut res = Mat3x3::default();

        res.content[0][0] = y.cos() * z.cos();
        res.content[0][1] = x.sin() * y.sin() * z.cos() - x.cos() * z.sin();
        res.content[0][2] = x.cos() * y.sin() * z.cos() + x.sin() * z.sin();
        res.content[1][0] = y.cos() * z.sin();
        res.content[1][1] = x.sin() * y.sin() * z.sin() + x.cos() * z.cos();
        res.content[1][2] = x.cos() * y.sin() * z.sin() - x.sin() * z.cos();
        res.content[2][0] = -y.sin();
        res.content[2][1] = x.sin() * y.cos();
        res.content[2][2] = x.cos() * y.cos();

        return res;
    }
}

impl Mat4x4 {
    pub fn translation(offset: &Vec3) -> Mat4x4 {
        let mut res = Mat4x4::default();
        res.content[0][0] = 1.0;
        res.content[1][1] = 1.0;
        res.content[2][2] = 1.0;
        res.content[3][3] = 1.0;

        res.content[0][3] = offset.x;
        res.content[1][3] = offset.y;
        res.content[2][3] = offset.z;

        return res;
    }

    // cosycosz sinxsinycosz−cosxsinz cosxsinycosz+sinxsinz
    // cosysinz sinxsinysinz+cosxcosz cosxsinysinz−sinxcosz
    // −siny sinxcosy cosxcosy
    pub fn rotation(rotation: &Vec3) -> Mat4x4 {
        let (x, y, z) = (rotation.x.to_radians(), rotation.y.to_radians(), rotation.z.to_radians());
        let mut res = Mat4x4::default();

        res.content[0][0] = y.cos() * z.cos();
        res.content[0][1] = x.sin() * y.sin() * z.cos() - x.cos() * z.sin();
        res.content[0][2] = x.cos() * y.sin() * z.cos() + x.sin() * z.sin();
        res.content[1][0] = y.cos() * z.sin();
        res.content[1][1] = x.sin() * y.sin() * z.sin() + x.cos() * z.cos();
        res.content[1][2] = x.cos() * y.sin() * z.sin() - x.sin() * z.cos();
        res.content[2][0] = -y.sin();
        res.content[2][1] = x.sin() * y.cos();
        res.content[2][2] = x.cos() * y.cos();

        res.content[3][3] = 1.0;

        return res;
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl Default for Mat3x3 {
    fn default() -> Self {
        Self {
            content:
            [
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ]
        }
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

impl Mul<&Mat4x4> for &Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: &Mat4x4) -> Self::Output {
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

impl Mul<&Vec3> for &Mat3x3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        let mut res = Vec3::default();
        let content = self.content;
        res.x = content[0][0] * rhs.x + content[0][1] * rhs.y + content[0][2] * rhs.z;
        res.y = content[1][0] * rhs.x + content[1][1] * rhs.y + content[1][2] * rhs.z;
        res.z = content[2][0] * rhs.x + content[2][1] * rhs.y + content[2][2] * rhs.z;
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

impl Mul<&Triangle> for &Mat4x4 {
    type Output = Triangle;

    fn mul(self, rhs: &Triangle) -> Self::Output {
        let mut res = rhs.clone();
        res *= self;
        return res;
    }
}

impl Mul<f32> for &Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut res = self.clone();
        res.x *= rhs;
        res.y *= rhs;
        res.z *= rhs;
        return res;
    }
}

impl MulAssign<f32> for &mut Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Add<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn add(self, rhs: &Vec4) -> Self::Output {
        Vec4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w)
    }
}

impl Sub<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: &Vec4) -> Self::Output {
        Vec4::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Triangle {
    pub const fn new(p1: Vec4, p2: Vec4, p3: Vec4) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn clockwise(&self) -> Self {
        let mut res = self.clone();
        let v12 = &self.p2 - &self.p1;
        let v13 = &self.p3 - &self.p1;
        if v12.cross_len_2d(&v13) > 0.0 {
            swap(&mut res.p1, &mut res.p2);
        }
        return res;
    }
}

impl MulAssign<&Mat4x4> for Triangle {
    fn mul_assign(&mut self, rhs: &Mat4x4) {
        self.p1 = rhs * &self.p1;
        self.p2 = rhs * &self.p2;
        self.p3 = rhs * &self.p3;
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new(Vec4::default(), Vec4::default(), Vec4::default())
    }
}

impl MulAssign<&Mat3x3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Mat3x3) {
        *self = rhs * self;
    }
}

impl AddAssign<&Vec3> for &mut Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Plane for Vec4 {
    fn new_plane(point: Vec4, mut normal: Vec4) -> Self {
        normal.w = -point.dot(&normal);
        return normal;
    }

    fn is_point_inside(&self, point: &Vec4) -> bool {
        self.dot(point) + self.w >= 0.0
    }

    fn intersect_with_segment(&self, p1: &Vec4, p2: &Vec4) -> Vec4 {
        let t = (-self.w - self.dot(p1)) / self.dot(&(p2 - p1));
        return p1 + &(&(p2 - p1) * t);
    }
}