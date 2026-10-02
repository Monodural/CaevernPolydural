use caevern_sdk::game;
use rustc_hash::FxHashMap;

use crate::{block::{block::Block, shapes::{Element, ElementFaceTexture, ElementFaces}}, chunks::chunk::Chunk, render::render_chunk::render_chunk, world::{get_chunk::get_chunk_key, world_data::WorldData}};

pub mod render;
pub mod chunks;
pub mod block;
pub mod item;
pub mod world;

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    game::log("Initializing Caevern Polydural");

    let mut world_data = WorldData::new();

    world_data.add_shape("default".to_string(), vec![Element {
        from: [0.0, 0.0, 0.0],
        to: [16.0, 16.0, 16.0],
        faces: ElementFaces {
            north: ElementFaceTexture { uv: [0.0, 0.0, 16.0, 16.0] },
            east: ElementFaceTexture { uv: [0.0, 0.0, 16.0, 16.0] },
            south: ElementFaceTexture { uv: [0.0, 0.0, 16.0, 16.0] },
            west: ElementFaceTexture { uv: [0.0, 0.0, 16.0, 16.0] },
            up: ElementFaceTexture { uv: [0.0, 0.0, 16.0, 16.0] },
            down: ElementFaceTexture { uv: [0.0, 0.0, 16.0, 16.0] },
        }
    }]);

    game::log("Created world data");

    let mut test_chunk = Chunk::init(0, 0, 0);
    test_chunk.set_block(0, 0, 0, Block::DIRT);
    test_chunk.set_block(1, 1, 1, Block::DIRT);
    test_chunk.set_block(2, 2, 2, Block::DIRT);
    test_chunk.set_block(3, 3, 3, Block::DIRT);

    game::log("Created chunk");

    let mut chunks = FxHashMap::default();
    chunks.insert(get_chunk_key((0, 0, 0)), test_chunk);

    game::log("Created chunk map");

    let (
        vertices, _normals, _colors, uvs,
        _vertices_transparent, _normals_transparent, _colors_transparent, _uvs_transparent,
        _vertices_windy, _normals_windy, _colors_windy, _uvs_windy, _vertex_index_transparent, _vertex_index_windy
    ) = render_chunk(&test_chunk, &chunks, &world_data, 0, 0, 0, (0.0, 0.0, 0.0));

    let object = game::create_mesh_object(
        &vertices,
        &uvs
    );
    game::log(&format!("object: {}", object.get_id()));
}

#[unsafe(no_mangle)]
pub extern "C" fn update(dt: f32) {
    game::log(&format!("I'm a module... {}", dt));
}
