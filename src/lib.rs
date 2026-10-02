use caevern_sdk::game;
use rustc_hash::FxHashMap;

use crate::{block::block::Block, chunks::chunk::Chunk, render::render_chunk::render_chunk, world::{get_chunk::get_chunk_key, world_data::WorldData}};

pub mod render;
pub mod chunks;
pub mod block;
pub mod item;
pub mod world;

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    game::log("Initializing Caevern Polydural");

    let world_data = WorldData::new();

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

    render_chunk(&test_chunk, &chunks, &world_data, 0, 0, 0, (0.0, 0.0, 0.0));

    /*let object = game::create_mesh_object(
        &[
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,

            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
        ],
        &[
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,

            1.0, 0.0,
            0.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
        ],
        &[
            0, 1, 2,
            2, 3, 0,

            5, 4, 7,
            7, 6, 5,

            4, 0, 3,
            3, 7, 4,

            1, 5, 6,
            6, 2, 1,

            3, 2, 6,
            6, 7, 3,

            4, 5, 1,
            1, 0, 4,
        ],
    );
    game::log(&format!("object: {}", object.get_id()));*/
}

#[unsafe(no_mangle)]
pub extern "C" fn update(dt: f32) {
    game::log(&format!("I'm a module... {}", dt));
}
