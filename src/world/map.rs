use crate::models::object::Object;
use crate::world::block::Block;
use crate::world::block::BlockType;

pub struct Chunk {
    coordonates: [i64; 3],
    blocks: [[[Block; 16]; 16]; 16],
}

impl Chunk {
    fn new(coords: [i64; 3]) -> Chunk {
        let blocks = core::array::from_fn(|x| {
            core::array::from_fn(|y| {
                core::array::from_fn(|z| {
                    let world_x = coords[0] * 16 + x as i64;
                    let world_y = coords[1] * 16 + y as i64;
                    let world_z = coords[2] * 16 + z as i64;
                    Block::new(BlockType::Void, [world_x, world_y, world_z])
                })
            })
        });

        Chunk {
            coordonates: coords,
            blocks,
        }
    }

    fn floor(coords: [i64; 3]) -> Chunk {
        let blocks = core::array::from_fn(|x| {
            core::array::from_fn(|y| {
                core::array::from_fn(|z| {
                    let world_x = coords[0] * 16 + x as i64;
                    let world_y = coords[1] * 16 + y as i64;
                    let world_z = coords[2] * 16 + z as i64;
                    let block_type = if y == 0 {
                        BlockType::Bedrock
                    } else {
                        BlockType::Void
                    };
                    Block::new(block_type, [world_x, world_y, world_z])
                })
            })
        });

        Chunk {
            coordonates: coords,
            blocks,
        }
    }

    pub fn get_show(&self) -> Vec<Object> {

        let mut to_render: Vec<Object> = vec![];

        for (_x, plane) in self.blocks.iter().enumerate() {
            for (_y, row) in plane.iter().enumerate() {
                for (_z, block) in row.iter().enumerate() {
                    // `block` est &Block
                    if block.block_type == BlockType::Void {
                        continue;
                    }

                    to_render.push(block.render.clone());
                }
            }
        }

        to_render
    }
}

pub struct Map {
    pub chunks: Vec<Chunk>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            chunks: vec![Chunk::floor([0, 0, 0]), Chunk::floor([1, 0, 1])],
        }
    }
}