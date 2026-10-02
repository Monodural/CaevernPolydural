use caevern_sdk::game;
use rustc_hash::FxHashMap;

use crate::{block::block_damage::get_block_damage_model, chunks::chunk::Chunk, render::create_face::create_face, world::{get_block::get_block_global_or_direct, get_chunk::get_chunk_key, world_data::WorldData}};

pub fn render_chunk(chunk: &Chunk, chunks: &FxHashMap<i64, Chunk>, world_data: &WorldData, chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64, camera_position: (f32, f32, f32)) -> (Vec<[f64; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f64; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f64; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<(usize, f64)>, Vec<(usize, f64)>) {
    game::log("Starting time");

    let current_time = game::get_time();

    game::log("Rendering chunk");

    let mut vertices: Vec<[f64; 3]> = Vec::new();
    let mut normals: Vec<[i8; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let mut vertices_transparent: Vec<[f64; 3]> = Vec::new();
    let mut normals_transparent: Vec<[i8; 3]> = Vec::new();
    let mut colors_transparent: Vec<[f32; 3]> = Vec::new();
    let mut uvs_transparent: Vec<[f32; 2]> = Vec::new();

    let mut vertices_windy: Vec<[f64; 3]> = Vec::new();
    let mut normals_windy: Vec<[i8; 3]> = Vec::new();
    let mut colors_windy: Vec<[f32; 3]> = Vec::new();
    let mut uvs_windy: Vec<[f32; 2]> = Vec::new();

    let mut vertex_index_transparent = Vec::new();
    let mut vertex_index_windy = Vec::new();

    let chunk_coordinates = chunk.get_chunk_coordinates();

    let empty_chunk = Chunk::empty();
    let neighboring_chunks: [&Chunk; 6] = [
        chunks.get(&get_chunk_key((chunk_coordinates.0 - 1, chunk_coordinates.1, chunk_coordinates.2))).unwrap_or(&empty_chunk),
        chunks.get(&get_chunk_key((chunk_coordinates.0 + 1, chunk_coordinates.1, chunk_coordinates.2))).unwrap_or(&empty_chunk),
        chunks.get(&get_chunk_key((chunk_coordinates.0, chunk_coordinates.1 - 1, chunk_coordinates.2))).unwrap_or(&empty_chunk),
        chunks.get(&get_chunk_key((chunk_coordinates.0, chunk_coordinates.1 + 1, chunk_coordinates.2))).unwrap_or(&empty_chunk),
        chunks.get(&get_chunk_key((chunk_coordinates.0, chunk_coordinates.1, chunk_coordinates.2 - 1))).unwrap_or(&empty_chunk),
        chunks.get(&get_chunk_key((chunk_coordinates.0, chunk_coordinates.1, chunk_coordinates.2 + 1))).unwrap_or(&empty_chunk),
    ];

    let atlas_size = world_data.texture_atlas_size as f32;

    for x in 0..16 {
        game::log(&format!("Rendering chunk x={}", x));
        for z in 0..16 {
            game::log(&format!("Rendering chunk z={}", z));
            for y in 0..16 {
                game::log(&format!("Rendering chunk y={}", y));
                let block = chunk.get_block(x, y, z);
                if block.is_air() { continue; }

                let light_level = 1.0;
                let light_level_full = [1.0 * light_level, 1.0 * light_level, 1.0 * light_level];
                let light_level_half = [0.5 * light_level, 0.5 * light_level, 0.5 * light_level];

                let mut render_all_block_faces = false;

                let mut block_model = block.properties().get_model();
                if block_model == "default".to_string() {
                    let block_damage = chunk.get_block_damage(x, y, z);
                    if block_damage > 0 {
                        render_all_block_faces = true;
                    }
                    block_model = get_block_damage_model(block, block_damage);
                }

                let mut directions = vec![
                    false, false, false, false, false, false,
                    false, false, false, false, false, false,
                    false, false, false, false, false, false,
                    false, false, false, false
                ];

                if !block.properties().is_transparent() && !render_all_block_faces {
                    let block_right = get_block_global_or_direct(x as i8 + 1, y as i8, z as i8, &neighboring_chunks, chunk);
                    let block_left = get_block_global_or_direct(x as i8 - 1, y as i8, z as i8, &neighboring_chunks, chunk);
                    let block_up = get_block_global_or_direct(x as i8, y as i8 + 1, z as i8, &neighboring_chunks, chunk);
                    let block_down = get_block_global_or_direct(x as i8, y as i8 - 1, z as i8, &neighboring_chunks, chunk);
                    let block_front = get_block_global_or_direct(x as i8, y as i8, z as i8 + 1, &neighboring_chunks, chunk);
                    let block_back = get_block_global_or_direct(x as i8, y as i8, z as i8 - 1, &neighboring_chunks, chunk);

                    if !block_right.0.properties().is_transparent() && !block_right.0.properties().render_neighbors() && block_right.2 == 0 {
                        directions[0] = true;
                    }
                    if !block_left.0.properties().is_transparent() && !block_left.0.properties().render_neighbors() && block_left.2 == 0 {
                        directions[1] = true;
                    }
                    if !block_up.0.properties().is_transparent() && !block_up.0.properties().render_neighbors() && block_up.2 == 0 {
                        directions[2] = true;
                    }
                    if !block_down.0.properties().is_transparent() && !block_down.0.properties().render_neighbors() && block_down.2 == 0 {
                        directions[3] = true;
                    }
                    if !block_front.0.properties().is_transparent() && !block_front.0.properties().render_neighbors() && block_front.2 == 0 {
                        directions[4] = true;
                    }
                    if !block_back.0.properties().is_transparent() && !block_back.0.properties().render_neighbors() && block_back.2 == 0 {
                        directions[5] = true;
                    }
                }

                if !directions[0] || !directions[3] {
                    let block_7 = get_block_global_or_direct(x as i8 + 1, y as i8 - 1, z as i8, &neighboring_chunks, chunk);
                    if block_7.0.properties().has_ambient_occlusion() && block_7.2 == 0 { // right bottom (7)
                        directions[7] = true;
                    }
                }
                if !directions[2] {
                    let block_6 = get_block_global_or_direct(x as i8 + 1, y as i8 + 1, z as i8, &neighboring_chunks, chunk);
                    if block_6.0.properties().has_ambient_occlusion() && block_6.2 == 0 { // right top (6)
                        directions[6] = true;
                    }
                    let block_8 = get_block_global_or_direct(x as i8 - 1, y as i8 + 1, z as i8, &neighboring_chunks, chunk);
                    if block_8.0.properties().has_ambient_occlusion() && block_8.2 == 0 { // left top (8)
                        directions[8] = true;
                    }
                    let block_10 = get_block_global_or_direct(x as i8, y as i8 + 1, z as i8 + 1, &neighboring_chunks, chunk);
                    if block_10.0.properties().has_ambient_occlusion() && block_10.2 == 0 { // front top (10)
                        directions[10] = true;
                    }
                    let block_12 = get_block_global_or_direct(x as i8, y as i8 + 1, z as i8 - 1, &neighboring_chunks, chunk);
                    if block_12.0.properties().has_ambient_occlusion() && block_12.2 == 0 { // back top (12)
                        directions[12] = true;
                    }
                    let block_18 = get_block_global_or_direct(x as i8 + 1, y as i8 + 1, z as i8 + 1, &neighboring_chunks, chunk);
                    if block_18.0.properties().has_ambient_occlusion() && block_18.2 == 0 { // right front top (18)
                        directions[18] = true;
                    }
                    let block_19 = get_block_global_or_direct(x as i8 + 1, y as i8 + 1, z as i8 - 1, &neighboring_chunks, chunk);
                    if block_19.0.properties().has_ambient_occlusion() && block_19.2 == 0 { // right back top (19)
                        directions[19] = true;
                    }
                    let block_20 = get_block_global_or_direct(x as i8 - 1, y as i8 + 1, z as i8 + 1, &neighboring_chunks, chunk);
                    if block_20.0.properties().has_ambient_occlusion() && block_20.2 == 0 { // left front top (20)
                        directions[20] = true;
                    }
                    let block_21 = get_block_global_or_direct(x as i8 - 1, y as i8 + 1, z as i8 - 1, &neighboring_chunks, chunk);
                    if block_21.0.properties().has_ambient_occlusion() && block_21.2 == 0 { // left back top (21)
                        directions[21] = true;
                    }
                    /*if get_block_global_or_direct(x as i8 + 1, y as i8, z as i8 + 1, &neighboring_chunks, chunk).properties().has_ambient_occlusion() { // right front (14)
                        directions[14] = true;
                    }
                    if get_block_global_or_direct(x as i8 + 1, y as i8, z as i8 - 1, &neighboring_chunks, chunk).properties().has_ambient_occlusion() { // right back (15)
                        directions[15] = true;
                    }
                    if get_block_global_or_direct(x as i8 - 1, y as i8, z as i8 + 1, &neighboring_chunks, chunk).properties().has_ambient_occlusion() { // left front (16)
                        directions[16] = true;
                    }
                    if get_block_global_or_direct(x as i8 - 1, y as i8, z as i8 - 1, &neighboring_chunks, chunk).properties().has_ambient_occlusion() { // left back (17)
                        directions[17] = true;
                    }*/
                }
                if !directions[1] || !directions[3] {
                    let block_9 = get_block_global_or_direct(x as i8 - 1, y as i8 - 1, z as i8, &neighboring_chunks, chunk);
                    if block_9.0.properties().has_ambient_occlusion() && block_9.2 == 0 { // left bottom (9)
                        directions[9] = true;
                    }
                }
                if !directions[3] || !directions[4] {
                    let block_11 = get_block_global_or_direct(x as i8, y as i8 - 1, z as i8 + 1, &neighboring_chunks, chunk);
                    if block_11.0.properties().has_ambient_occlusion() && block_11.2 == 0 { // front bottom (11)
                        directions[11] = true;
                    }
                }
                if !directions[3] || !directions[5] {
                    let block_13 = get_block_global_or_direct(x as i8, y as i8 - 1, z as i8 - 1, &neighboring_chunks, chunk);
                    if block_13.0.properties().has_ambient_occlusion() && block_13.2 == 0 { // back bottom (13)
                        directions[13] = true;
                    }
                }

                let mut block_position_x = x as f64;
                let block_position_y = y as f64;
                let mut block_position_z = z as f64;

                let block_offset = block.properties().get_offset() as f64;

                if block_offset > 0.0 {
                    block_position_x += (x as f64 * 10.0 + z as f64 + y as f64 * 10.0).cos() * block_offset;
                    block_position_z += (x as f64 + z as f64 * 10.0 + y as f64).sin() * block_offset;
                }

                let block_position_globalized_x = block_position_x * 2.0 + chunk_position_x as f64 * 32.0;
                let block_position_globalized_y = block_position_y * 2.0 + chunk_position_y as f64 * 32.0;
                let block_position_globalized_z = block_position_z * 2.0 + chunk_position_z as f64 * 32.0;

                let elements = world_data.shapes.get(&block_model).unwrap();
                for element in elements {
                    let vertices_from = element.from;
                    let vertices_to = element.to;

                    let mut cube_vertices = Vec::new();

                    if !directions[0] || !directions[1] || !directions[2] ||
                        !directions[3] || !directions[4] || !directions[5] {

                        let vertices_to_devided_x = 2.0 - vertices_to[0] / 8.0;
                        let vertices_to_devided_y = 2.0 - vertices_to[1] / 8.0;
                        let vertices_to_devided_z = 2.0 - vertices_to[2] / 8.0;

                        let vertices_from_devided_x = vertices_from[0] / 8.0;
                        let vertices_from_devided_y = vertices_from[1] / 8.0;
                        let vertices_from_devided_z = vertices_from[2] / 8.0;

                        cube_vertices.push([ (1.0 - vertices_to_devided_x) + block_position_globalized_x,   (-1.0 + vertices_from_devided_y) + block_position_globalized_y,  (1.0 - vertices_to_devided_z) + block_position_globalized_z]);
                        cube_vertices.push([ (1.0 - vertices_to_devided_x) + block_position_globalized_x,   (-1.0 + vertices_from_devided_y) + block_position_globalized_y, (-1.0 + vertices_from_devided_z) + block_position_globalized_z]);
                        cube_vertices.push([ (1.0 - vertices_to_devided_x) + block_position_globalized_x,    (1.0 - vertices_to_devided_y) + block_position_globalized_y,    (1.0 - vertices_to_devided_z) + block_position_globalized_z]);
                        cube_vertices.push([ (1.0 - vertices_to_devided_x) + block_position_globalized_x,    (1.0 - vertices_to_devided_y) + block_position_globalized_y,   (-1.0 + vertices_from_devided_z) + block_position_globalized_z]);
                        cube_vertices.push([(-1.0 + vertices_from_devided_x) + block_position_globalized_x, (-1.0 + vertices_from_devided_y) + block_position_globalized_y, (-1.0 + vertices_from_devided_z) + block_position_globalized_z]);
                        cube_vertices.push([(-1.0 + vertices_from_devided_x) + block_position_globalized_x, (-1.0 + vertices_from_devided_y) + block_position_globalized_y,  (1.0 - vertices_to_devided_z) + block_position_globalized_z]);
                        cube_vertices.push([(-1.0 + vertices_from_devided_x) + block_position_globalized_x,  (1.0 - vertices_to_devided_y) + block_position_globalized_y,   (-1.0 + vertices_from_devided_z) + block_position_globalized_z]);
                        cube_vertices.push([(-1.0 + vertices_from_devided_x) + block_position_globalized_x,  (1.0 - vertices_to_devided_y) + block_position_globalized_y,    (1.0 - vertices_to_devided_z) + block_position_globalized_z]);
                    }

                    /*if !block.properties().is_transparent() {
                        if !directions[0] {
                            let uv_x = (block.properties().get_atlas_position(0) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(0) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices, &mut uvs, &mut normals, &cube_vertices,
                                [0, 1, 2, 3], [1, 0, 0],
                                uv_x, uv_y, atlas_size, element.faces.north.uv
                            );

                            if !directions[7] {
                                colors.push(light_level_full);
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                                colors.push(light_level_half);
                            }
                            colors.push(light_level_full);
                            colors.push(light_level_full);
                            if !directions[7] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            colors.push(light_level_full);
                        }
                        if !directions[1] {
                            let uv_x = (block.properties().get_atlas_position(1) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(1) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices, &mut uvs, &mut normals, &cube_vertices,
                                [4, 5, 6, 7], [-1, 0, 0],
                                uv_x, uv_y, atlas_size, element.faces.south.uv
                            );

                            if !directions[9] {
                                colors.push(light_level_full);
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                                colors.push(light_level_half);
                            }
                            colors.push(light_level_full);
                            colors.push(light_level_full);
                            if !directions[9] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            colors.push(light_level_full);
                        }
                        if !directions[2] {
                            let uv_x = (block.properties().get_atlas_position(2) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(2) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices, &mut uvs, &mut normals, &cube_vertices,
                                [7, 2, 6, 3], [0, 1, 0],
                                uv_x, uv_y, atlas_size, element.faces.up.uv
                            );

                            if !directions[8] && ! directions[10] && !directions[20] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            if !directions[6] && ! directions[10] && !directions[18] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            if !directions[8] && ! directions[12] && !directions[21] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            if !directions[8] && ! directions[12] && !directions[21] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            if !directions[6] && ! directions[10] && !directions[18] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            if !directions[6] && ! directions[12] && !directions[19] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                        }
                        if !directions[3] {
                            let uv_x = (block.properties().get_atlas_position(3) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(3) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices, &mut uvs, &mut normals, &cube_vertices,
                                [4, 1, 5, 0], [0, -1, 0],
                                uv_x, uv_y, atlas_size, element.faces.down.uv
                            );

                            if !directions[9] && !directions[13] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            if !directions[7] && !directions[13] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            if !directions[9] && !directions[11] {
                                colors.push(light_level_full);
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                                colors.push(light_level_half);
                            }
                            if !directions[7] && !directions[13] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            if !directions[7] && !directions[11] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                        }
                        if !directions[4] {
                            let uv_x = (block.properties().get_atlas_position(4) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(4) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices, &mut uvs, &mut normals, &cube_vertices,
                                [5, 0, 7, 2], [0, 0, 1],
                                uv_x, uv_y, atlas_size, element.faces.east.uv
                            );

                            if !directions[11] {
                                colors.push(light_level_full);
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                                colors.push(light_level_half);
                            }
                            colors.push(light_level_full);
                            colors.push(light_level_full);
                            if !directions[11] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            colors.push(light_level_full);
                        }
                        if !directions[5] {
                            let uv_x = (block.properties().get_atlas_position(5) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(5) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices, &mut uvs, &mut normals, &cube_vertices,
                                [1, 4, 3, 6], [0, 0, -1],
                                uv_x, uv_y, atlas_size, element.faces.west.uv
                            );

                            if !directions[13] {
                                colors.push(light_level_full);
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                                colors.push(light_level_half);
                            }
                            colors.push(light_level_full);
                            colors.push(light_level_full);
                            if !directions[13] {
                                colors.push(light_level_full);
                            } else  {
                                colors.push(light_level_half);
                            }
                            colors.push(light_level_full);
                        }
                    } else if block.properties().is_wind_affected() {
                        if !directions[0] && vertices_from[1] != vertices_to[1] && vertices_from[2] != vertices_to[2] {
                            vertex_index_windy.push((vertices_windy.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 + 1.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(0) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(0) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_windy, &mut uvs_windy, &mut normals_windy, &cube_vertices,
                                [0, 1, 2, 3], [1, 0, 0],
                                uv_x, uv_y, atlas_size, element.faces.north.uv
                            );

                            if !directions[7] {
                                colors_windy.push(light_level_full);
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                                colors_windy.push(light_level_half);
                            }
                            colors_windy.push(light_level_full);
                            colors_windy.push(light_level_full);
                            if !directions[7] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            colors_windy.push(light_level_full);
                        }
                        if !directions[1] && vertices_from[1] != vertices_to[1] && vertices_from[2] != vertices_to[2] {
                            vertex_index_windy.push((vertices_windy.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - 1.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(1) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(1) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_windy, &mut uvs_windy, &mut normals_windy, &cube_vertices,
                                [4, 5, 6, 7], [-1, 0, 0],
                                uv_x, uv_y, atlas_size, element.faces.south.uv
                            );

                            if !directions[9] {
                                colors_windy.push(light_level_full);
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                                colors_windy.push(light_level_half);
                            }
                            colors_windy.push(light_level_full);
                            colors_windy.push(light_level_full);
                            if !directions[9] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            colors_windy.push(light_level_full);
                        }
                        if !directions[2] && vertices_from[0] != vertices_to[0] && vertices_from[2] != vertices_to[2] {
                            vertex_index_windy.push((vertices_windy.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 + 1.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(2) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(2) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_windy, &mut uvs_windy, &mut normals_windy, &cube_vertices,
                                [7, 2, 6, 3], [0, 1, 0],
                                uv_x, uv_y, atlas_size, element.faces.up.uv
                            );

                            if !directions[8] && ! directions[10] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            if !directions[6] && ! directions[10] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            if !directions[8] && ! directions[12] {
                                colors_windy.push(light_level_full);
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                                colors_windy.push(light_level_half);
                            }
                            if !directions[6] && ! directions[10] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            if !directions[6] && ! directions[12] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                        }
                        if !directions[3] && vertices_from[0] != vertices_to[0] && vertices_from[2] != vertices_to[2] {
                            vertex_index_windy.push((vertices_windy.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 - 1.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(3) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(3) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_windy, &mut uvs_windy, &mut normals_windy, &cube_vertices,
                                [4, 1, 5, 0], [0, -1, 0],
                                uv_x, uv_y, atlas_size, element.faces.down.uv
                            );

                            if !directions[9] && !directions[13] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            if !directions[7] && !directions[13] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            if !directions[9] && !directions[11] {
                                colors_windy.push(light_level_full);
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                                colors_windy.push(light_level_half);
                            }
                            if !directions[7] && !directions[13] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            if !directions[7] && !directions[11] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                        }
                        if !directions[4] && vertices_from[0] != vertices_to[0] && vertices_from[1] != vertices_to[1] {
                            vertex_index_windy.push((vertices_windy.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 + 1.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(4) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(4) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_windy, &mut uvs_windy, &mut normals_windy, &cube_vertices,
                                [5, 0, 7, 2], [0, 0, 1],
                                uv_x, uv_y, atlas_size, element.faces.east.uv
                            );

                            if !directions[11] {
                                colors_windy.push(light_level_full);
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                                colors_windy.push(light_level_half);
                            }
                            colors_windy.push(light_level_full);
                            colors_windy.push(light_level_full);
                            if !directions[11] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            colors_windy.push(light_level_full);
                        }
                        if !directions[5] && vertices_from[0] != vertices_to[0] && vertices_from[1] != vertices_to[1] {
                            vertex_index_windy.push((vertices_windy.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - 1.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(5) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(5) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_windy, &mut uvs_windy, &mut normals_windy, &cube_vertices,
                                [1, 4, 3, 6], [0, 0, -1],
                                uv_x, uv_y, atlas_size, element.faces.west.uv
                            );

                            if !directions[13] {
                                colors_windy.push(light_level_full);
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                                colors_windy.push(light_level_half);
                            }
                            colors_windy.push(light_level_full);
                            colors_windy.push(light_level_full);
                            if !directions[13] {
                                colors_windy.push(light_level_full);
                            } else  {
                                colors_windy.push(light_level_half);
                            }
                            colors_windy.push(light_level_full);
                        }
                    } else {
                        if !directions[0] && vertices_from[1] != vertices_to[1] && vertices_from[2] != vertices_to[2] {
                            vertex_index_transparent.push((vertex_index_transparent.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 + 1.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(0) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(0) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_transparent, &mut uvs_transparent, &mut normals_transparent, &cube_vertices,
                                [0, 1, 2, 3], [1, 0, 0],
                                uv_x, uv_y, atlas_size, element.faces.north.uv
                            );

                            if !directions[7] {
                                colors_transparent.push(light_level_full);
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                                colors_transparent.push(light_level_half);
                            }
                            colors_transparent.push(light_level_full);
                            colors_transparent.push(light_level_full);
                            if !directions[7] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            colors_transparent.push(light_level_full);
                        }
                        if !directions[1] && vertices_from[1] != vertices_to[1] && vertices_from[2] != vertices_to[2] {
                            vertex_index_transparent.push((vertex_index_transparent.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - 1.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(1) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(1) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_transparent, &mut uvs_transparent, &mut normals_transparent, &cube_vertices,
                                [4, 5, 6, 7], [-1, 0, 0],
                                uv_x, uv_y, atlas_size, element.faces.south.uv
                            );

                            if !directions[9] {
                                colors_transparent.push(light_level_full);
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                                colors_transparent.push(light_level_half);
                            }
                            colors_transparent.push(light_level_full);
                            colors_transparent.push(light_level_full);
                            if !directions[9] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            colors_transparent.push(light_level_full);
                        }
                        if !directions[2] && vertices_from[0] != vertices_to[0] && vertices_from[2] != vertices_to[2] {
                            vertex_index_transparent.push((vertex_index_transparent.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 + 1.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(2) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(2) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_transparent, &mut uvs_transparent, &mut normals_transparent, &cube_vertices,
                                [7, 2, 6, 3], [0, 1, 0],
                                uv_x, uv_y, atlas_size, element.faces.up.uv
                            );

                            if !directions[8] && ! directions[10] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            if !directions[6] && ! directions[10] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            if !directions[8] && ! directions[12] {
                                colors_transparent.push(light_level_full);
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                                colors_transparent.push(light_level_half);
                            }
                            if !directions[6] && ! directions[10] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            if !directions[6] && ! directions[12] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                        }
                        if !directions[3] && vertices_from[0] != vertices_to[0] && vertices_from[2] != vertices_to[2] {
                            vertex_index_transparent.push((vertex_index_transparent.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 + 1.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(3) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(3) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_transparent, &mut uvs_transparent, &mut normals_transparent, &cube_vertices,
                                [4, 1, 5, 0], [0, -1, 0],
                                uv_x, uv_y, atlas_size, element.faces.down.uv
                            );

                            if !directions[9] && !directions[13] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            if !directions[7] && !directions[13] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            if !directions[9] && !directions[11] {
                                colors_transparent.push(light_level_full);
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                                colors_transparent.push(light_level_half);
                            }
                            if !directions[7] && !directions[13] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            if !directions[7] && !directions[11] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                        }
                        if !directions[4] && vertices_from[0] != vertices_to[0] && vertices_from[1] != vertices_to[1] {
                            vertex_index_transparent.push((vertex_index_transparent.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 + 1.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(4) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(4) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_transparent, &mut uvs_transparent, &mut normals_transparent, &cube_vertices,
                                [5, 0, 7, 2], [0, 0, 1],
                                uv_x, uv_y, atlas_size, element.faces.east.uv
                            );

                            if !directions[11] {
                                colors_transparent.push(light_level_full);
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                                colors_transparent.push(light_level_half);
                            }
                            colors_transparent.push(light_level_full);
                            colors_transparent.push(light_level_full);
                            if !directions[11] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            colors_transparent.push(light_level_full);
                        }
                        if !directions[5] && vertices_from[0] != vertices_to[0] && vertices_from[1] != vertices_to[1] {
                            vertex_index_transparent.push((vertex_index_transparent.len(), (
                                (block_position_x * 2.0 + chunk_position_x as f64 * 32.0 - camera_position.0 as f64).powi(2) +
                                (block_position_y * 2.0 + chunk_position_y as f64 * 32.0 - camera_position.1 as f64).powi(2) +
                                (block_position_z * 2.0 + chunk_position_z as f64 * 32.0 - 1.0 - camera_position.2 as f64).powi(2)
                            )));

                            let uv_x = (block.properties().get_atlas_position(5) as f32 % atlas_size).floor();
                            let uv_y = (block.properties().get_atlas_position(5) as f32 / atlas_size).floor();
                            create_face(
                                &mut vertices_transparent, &mut uvs_transparent, &mut normals_transparent, &cube_vertices,
                                [1, 4, 3, 6], [0, 0, -1],
                                uv_x, uv_y, atlas_size, element.faces.west.uv
                            );

                            if !directions[13] {
                                colors_transparent.push(light_level_full);
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                                colors_transparent.push(light_level_half);
                            }
                            colors_transparent.push(light_level_full);
                            colors_transparent.push(light_level_full);
                            if !directions[13] {
                                colors_transparent.push(light_level_full);
                            } else  {
                                colors_transparent.push(light_level_half);
                            }
                            colors_transparent.push(light_level_full);
                        }
                    }*/
                }
            }
        }
    }

    let time_passed = game::get_time() - current_time;
    game::log(&format!("rendered chunk in {}ms", time_passed));

    //vertex_index_transparent.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    //vertex_index_windy.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    return (vertices, normals, colors, uvs, vertices_transparent, normals_transparent, colors_transparent, uvs_transparent, vertices_windy, normals_windy, colors_windy, uvs_windy, vertex_index_transparent, vertex_index_windy);
}
