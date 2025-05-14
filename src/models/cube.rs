use crate::models::object::Vertex as Vertex;
use crate::models::object::Normal as Normal;

use super::object::Object;

const SIZE: f32 = 10.0;

fn vertices(pos: [f32; 3]) -> [Vertex; 8] {
    [
        Vertex { position: ( SIZE+pos[0],  SIZE+pos[1],  SIZE+pos[2]) },
        Vertex { position: ( SIZE+pos[0],  SIZE+pos[1], -SIZE+pos[2]) },
        Vertex { position: ( SIZE+pos[0], -SIZE+pos[1],  SIZE+pos[2]) },
        Vertex { position: ( SIZE+pos[0], -SIZE+pos[1], -SIZE+pos[2]) },
        Vertex { position: (-SIZE+pos[0],  SIZE+pos[1],  SIZE+pos[2]) },
        Vertex { position: (-SIZE+pos[0],  SIZE+pos[1], -SIZE+pos[2]) },
        Vertex { position: (-SIZE+pos[0], -SIZE+pos[1],  SIZE+pos[2]) },
        Vertex { position: (-SIZE+pos[0], -SIZE+pos[1], -SIZE+pos[2]) },
    ]
}

pub const NORMALS: [Normal; 12] = [
    Normal { normal: ( 0.0,  0.0,  1.0) },
    Normal { normal: ( 0.0,  0.0, -1.0) },
    Normal { normal: ( 1.0,  0.0,  0.0) },
    Normal { normal: (-1.0,  0.0,  0.0) },
    Normal { normal: ( 0.0,  1.0,  0.0) },
    Normal { normal: ( 0.0, -1.0,  0.0) },
    Normal { normal: ( 1.0,  1.0,  1.0) },
    Normal { normal: ( 1.0,  1.0, -1.0) },
    Normal { normal: ( 1.0, -1.0,  1.0) },
    Normal { normal: ( 1.0, -1.0, -1.0) },
    Normal { normal: (-1.0,  1.0,  1.0) },
    Normal { normal: (-1.0,  1.0, -1.0) },
];

pub const INDICES: [u16; 3*12] = [
    0, 1, 2,
    1, 2, 3,
    0, 1, 4,
    1, 4, 5,
    4, 5, 6,
    5, 6, 7,
    2, 3, 6,
    3, 6, 7,
    0, 2, 4,
    2, 4, 6,
    1, 3, 5,
    3, 5, 7,
];

pub fn new(pos: [f32; 3]) -> Object {
    Object::new(
        vertices(pos).to_vec(),
        NORMALS.to_vec(),
        INDICES.to_vec()
    )
}