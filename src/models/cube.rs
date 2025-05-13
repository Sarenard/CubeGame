#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32)
}

implement_vertex!(Vertex, position);

const SIZE: f32 = 30.0;

pub const VERTICES: [Vertex; 8] = [
    Vertex { position: ( SIZE,  SIZE,  SIZE) },
    Vertex { position: ( SIZE,  SIZE, -SIZE) },
    Vertex { position: ( SIZE, -SIZE,  SIZE) },
    Vertex { position: ( SIZE, -SIZE, -SIZE) },
    Vertex { position: (-SIZE,  SIZE,  SIZE) },
    Vertex { position: (-SIZE,  SIZE, -SIZE) },
    Vertex { position: (-SIZE, -SIZE,  SIZE) },
    Vertex { position: (-SIZE, -SIZE, -SIZE) },
];

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

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