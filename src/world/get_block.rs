use rustc_hash::FxHashMap;

use crate::{block::block::Block, chunks::chunk::Chunk, world::get_chunk::get_chunk_key};

pub fn get_block_global_or_direct(
    x: i8, y: i8, z: i8,
    chunks: &[&Chunk; 6],
    chunk: &Chunk
) -> (Block, u8, u8) {
    let dx = if x < 0 { -1 } else if x > 15 { 1 } else { 0 };
    let dy = if y < 0 { -1 } else if y > 15 { 1 } else { 0 };
    let dz = if z < 0 { -1 } else if z > 15 { 1 } else { 0 };

    let lx = (x & 15) as u8;
    let ly = (y & 15) as u8;
    let lz = (z & 15) as u8;

    let target_chunk = match (dx, dy, dz) {
        (0, 0, 0) => chunk,
        (-1, 0, 0) => chunks[0],
        (1, 0, 0) => chunks[1],
        (0, -1, 0) => chunks[2],
        (0, 1, 0) => chunks[3],
        (0, 0, -1) => chunks[4],
        (0, 0, 1) => chunks[5],
        _ => return (Block::NONE, 15, 0),
    };

    (
        target_chunk.get_block(lx, ly, lz),
        15,
        target_chunk.get_block_damage(lx, ly, lz),
    )
}

pub fn get_block_center_global(x: f64, y: f64, z: f64, chunks: &FxHashMap<i64, Chunk>) -> Block {
    let chunk_x_coordinate = ((x + 0.5) / 16.0).floor() as i64;
    let chunk_y_coordinate = ((y + 0.5) / 16.0).floor() as i64;
    let chunk_z_coordinate = ((z + 0.5) / 16.0).floor() as i64;

    if let Some(chunk) = chunks.get(&get_chunk_key((chunk_x_coordinate, chunk_y_coordinate, chunk_z_coordinate))) {
        let mut chunk_x_local_coordinate = ((x + 0.5) % 16.0).floor() as i64;
        let mut chunk_y_local_coordinate = ((y + 0.5) % 16.0).floor() as i64;
        let mut chunk_z_local_coordinate = ((z + 0.5) % 16.0).floor() as i64;

        if chunk_x_local_coordinate < 0 { chunk_x_local_coordinate += 16; }
        if chunk_y_local_coordinate < 0 { chunk_y_local_coordinate += 16; }
        if chunk_z_local_coordinate < 0 { chunk_z_local_coordinate += 16; }

        return chunk.get_block(chunk_x_local_coordinate as u8, chunk_y_local_coordinate as u8, chunk_z_local_coordinate as u8);
    } else {
        return Block::NONE;
    }
}
