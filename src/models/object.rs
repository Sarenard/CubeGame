#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32)
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

#[derive(Debug, Clone)]
pub struct Object {
    pub vertices: Vec<Vertex>,
    #[allow(unused)]
    pub normals: Vec<Normal>,
    pub indices: Vec<u16>,
}

impl Object {
    pub fn new(vertices: Vec<Vertex>, normals: Vec<Normal>, indices: Vec<u16>) -> Object {
        Object {
            vertices: vertices,
            normals: normals,
            indices: indices
        }
    }
}
