use crate::models::{cube::{self, SIZE}, object::{self, Object}};

#[derive(PartialEq)]
pub enum BlockType {
    Void,
    Bedrock,
}

pub struct Block {
    pub render: Object, // the 3d representation of the block
    pub coordinates: [i64; 3],
    pub block_type: BlockType
}

impl Block {
    pub fn new(blocktype: BlockType, coordinates: [i64; 3]) -> Block {
        Block {
            render: cube::new([
                (coordinates[0]) as f32 * SIZE * 2., 
                (coordinates[1]) as f32 * SIZE * 2., 
                (coordinates[2]) as f32 * SIZE * 2.]
            ),
            coordinates: coordinates,
            block_type: blocktype,
        }
    }
}