use std::ops::Neg;

use crate::render::quaternion::Quaternion;

#[derive(Debug, Clone, Copy)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3d { x, y, z }
    }

    pub fn normalize(self) -> Vec3d {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3d::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn to_quaternion(self) -> Quaternion {
        Quaternion::new(0.0, self.x, self.y, self.z)
    }

    pub fn cross(self, other: Vec3d) -> Vec3d {
        Vec3d::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

}

impl Neg for Vec3d {
    type Output = Vec3d;

    fn neg(self) -> Vec3d {
        Vec3d {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// scalar multiplication
impl std::ops::Mul<f32> for Vec3d {
    type Output = Vec3d;

    fn mul(self, scalar: f32) -> Vec3d {
        Vec3d {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

