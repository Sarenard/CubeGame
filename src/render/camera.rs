use crate::render::quaternion::Quaternion;

use super::vec3d::Vec3d;

pub struct Camera {
    pub position: [f32; 3],
    pub direction: Quaternion,
    pub up: [f32; 3],
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: [2.0, -1.0, 1.0],
            direction: Vec3d::new(-2.0, 1.0, 1.0).to_quaternion().normalize(), 
            up: [0.0, 1.0, 0.0]
        }
    }

    pub fn render(self: &Camera) -> [[f32; 4]; 4] {
        return view_matrix(
            &self.position, 
            &[self.direction.x, self.direction.y, self.direction.z], 
            &self.up
        );
    }

    pub fn deplace(self: &mut Camera, vect: Vec3d) {
        let normalised_vect = vect.normalize();
        let speed = 0.1;
        self.position[0] += normalised_vect.x * speed;
        self.position[1] += normalised_vect.y * speed;
        self.position[2] += normalised_vect.z * speed;
    }

}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
