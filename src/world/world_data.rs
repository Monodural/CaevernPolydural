use std::collections::HashMap;

use crate::block::{block::Block, shapes::Element};

#[derive(Clone)]
pub struct WorldData {
    pub shapes: HashMap<String, Vec<Element>>,

    //pub textures: Vec<(image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, wgpu::Extent3d, u32, u32)>,
    pub texture_atlas_size: u16,

    pub biomes: HashMap<String, (i8, i8, i8, Vec<(Vec<Block>, i64)>, i64, Vec<(String, f32)>, Vec<(String, f32)>, Vec<(String, f32)>)>,

    pub structures: HashMap<String, Vec<((i32, i32, i32), Block)>>,
    pub block_queue: Vec<((i64, i64, i64), (u8, u8, u8), Block)>,

    pub audio_files: Vec<Vec<Vec<i16>>>,
    pub music_files: Vec<Vec<Vec<i16>>>,
}
impl WorldData {
    pub fn new() -> Self {
        WorldData {
            shapes: HashMap::new(),
            //textures: Vec::new(),
            texture_atlas_size: 8,
            biomes: HashMap::new(),
            structures: HashMap::new(),
            block_queue: Vec::new(),
            audio_files: Vec::new(),
            music_files: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape_name: String, elements: Vec<Element>) {
        self.shapes.insert(shape_name, elements);
    }
    /*pub fn add_structure(&mut self, structure_name: String, blocks: Vec<StructureBlock>) {
        let mut blocks_converted = Vec::new();
        for block in blocks {
            if let Ok(block_type) = block.block.to_uppercase().parse::<Block>() {
                blocks_converted.push(((block.position[0], block.position[1], block.position[2]), block_type))
            } else {
                blocks_converted.push(((block.position[0], block.position[1], block.position[2]), Block::ERROR));
            }
        }

        self.structures.insert(structure_name, blocks_converted);
    }
    pub fn add_biome(&mut self, biome_name: String, temperature: i8, moisture: i8, height: i8,
                    block_levels: Vec<(Vec<String>, i64)>, sea_level: i64, trees: Vec<(String, f32)>,
                    folliage: Vec<(String, f32)>, buildings: Vec<(String, f32)>) {

        let mut block_layers_converted = Vec::new();
        for block_layer in block_levels {
            let block_layer_height = block_layer.1;
            let mut block_layer_converted = Vec::new();

            for block in block_layer.0 {
                if let Ok(block_type) = block.to_uppercase().parse::<Block>() {
                    block_layer_converted.push(block_type);
                } else {
                    block_layer_converted.push(Block::ERROR);
                }
            }

            block_layers_converted.push((block_layer_converted, block_layer_height));
        }

        if !self.biomes.contains_key(&biome_name) {
            self.biomes.insert(biome_name, (temperature, moisture, height, block_layers_converted, sea_level, trees, folliage, buildings));
        }
    }*/
}
