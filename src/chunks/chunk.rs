use crate::block::block::Block;

#[derive(Clone, Copy)]
pub struct Chunk {
    blocks: [(Block, u8); 4096 ],

    position: (i64, i64, i64),

    has_vertex_buffer: bool,
    vertex_buffer_id: usize
}
impl Chunk {
    pub fn init(x: i64, y: i64, z: i64) -> Self {
        Self {
            blocks: [(Block::AIR, 15); 4096 ],

            position: (x, y, z),

            has_vertex_buffer: false,
            vertex_buffer_id: 0
        }
    }
    pub fn empty() -> Self {
        Self {
            blocks: [(Block::NONE, 15); 4096 ],

            position: (0, 0, 0),

            has_vertex_buffer: false,
            vertex_buffer_id: 0
        }
    }
    pub fn get_vertex_buffer_index(&self) -> usize {
        self.vertex_buffer_id
    }

    pub fn get_light_data(&self, byte: u8) -> u8 {
        byte & 0b0000_1111
    }
    pub fn get_block_data(&self, byte: u8) -> u8 {
        (byte >> 4) & 0b0000_1111
    }

    pub fn set_block_data(&self, byte: u8, block: u8) -> u8 {
        let block = (block & 0b0000_1111) << 4;
        (byte & 0b0000_1111) | block
    }

    /*pub fn create_vertex_buffer(&mut self, init: &InitWgpu, vertex_data_default: Vec<Vertex>, vertex_data_transparent: Vec<Vertex>, vertex_data_windy: Vec<Vertex>, vertex_buffer_id: usize, vertex_index_transparent: Vec<(usize, f64)>, vertex_index_windy: Vec<(usize, f64)>) -> (Buffer, Buffer, Buffer) {
        self.has_vertex_buffer = true;
        self.vertex_buffer_id = vertex_buffer_id;

        let mut vertex_data_transparent_sorted: Vec<Vertex> = Vec::new();
        for face in vertex_index_transparent {
            vertex_data_transparent_sorted.extend(
                [vertex_data_transparent[face.0],
                vertex_data_transparent[face.0 + 1],
                vertex_data_transparent[face.0 + 2],
                vertex_data_transparent[face.0 + 3],
                vertex_data_transparent[face.0 + 4],
                vertex_data_transparent[face.0 + 5]].iter()
            );
        }

        let mut vertex_data_windy_sorted: Vec<Vertex> = Vec::new();
        for face in vertex_index_windy {
            vertex_data_windy_sorted.extend(
                [vertex_data_windy[face.0],
                vertex_data_windy[face.0 + 1],
                vertex_data_windy[face.0 + 2],
                vertex_data_windy[face.0 + 3],
                vertex_data_windy[face.0 + 4],
                vertex_data_windy[face.0 + 5]].iter()
            );
        }

        let vertex_buffer_default = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (vertex_data_default.len() * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let vertex_buffer_transparent = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (vertex_data_transparent_sorted.len() * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let vertex_buffer_windy = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (vertex_data_windy_sorted.len() * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        init.queue.write_buffer(&vertex_buffer_default, 0, bytemuck::cast_slice(&vertex_data_default));
        init.queue.write_buffer(&vertex_buffer_transparent, 0, bytemuck::cast_slice(&vertex_data_transparent_sorted));
        init.queue.write_buffer(&vertex_buffer_windy, 0, bytemuck::cast_slice(&vertex_data_windy_sorted));
        (vertex_buffer_default, vertex_buffer_transparent, vertex_buffer_windy)
    }*/

    fn xyz_to_index(&self, x: u8, y: u8, z: u8) -> usize {
        x as usize * 16 * 16 + y as usize * 16 + z as usize
    }

    pub fn get_chunk_coordinates(&self) -> (i64, i64, i64) {
        self.position
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, block_type: Block) {
        let block_index = self.xyz_to_index(x, y, z);
        self.blocks[block_index].0 = block_type;
        self.set_block_damage(x, y, z, 0);
    }
    pub fn get_block(&self, x: u8, y: u8, z: u8) -> Block {
        self.blocks[self.xyz_to_index(x, y, z)].0
    }

    pub fn damage_block(&mut self, x: u8, y: u8, z: u8) {
        let block_index = self.xyz_to_index(x, y, z);
        self.blocks[block_index].1 = self.set_block_data(self.blocks[block_index].1,
            self.get_block_damage(x, y, z) + 1
        );
    }
    pub fn set_block_damage(&mut self, x: u8, y: u8, z: u8, value: u8) {
        let block_index = self.xyz_to_index(x, y, z);
        self.blocks[block_index].1 = self.set_block_data(self.blocks[block_index].1, value);
    }
    pub fn get_block_damage(&self, x: u8, y: u8, z: u8) -> u8 {
        self.get_block_data(self.blocks[self.xyz_to_index(x, y, z)].1)
    }
}
