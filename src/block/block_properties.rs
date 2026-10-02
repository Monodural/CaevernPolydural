use crate::item::item::Item;

pub struct BlockProperties {
    pub is_plant_growable: bool,
    pub hardness: u8,
    pub is_transparent: bool,
    pub render_neighbors: bool,
    pub is_folliage: bool,
    pub can_collide: bool,
    pub breakable: bool,
    pub placable: bool,
    pub wind_affected: bool,
    pub ambient_occlusion: bool,
    pub textures: [u16; 6],
    pub block_model: &'static str,
    pub item: Item,
    pub offset: f32,
}
impl BlockProperties {
    pub fn is_plant_growable(&self) -> bool {
        self.is_plant_growable
    }
    pub fn is_transparent(&self) -> bool {
        self.is_transparent
    }
    pub fn render_neighbors(&self) -> bool {
        self.render_neighbors
    }
    pub fn is_wind_affected(&self) -> bool {
        self.wind_affected
    }
    pub fn is_folliage(&self) -> bool {
        self.is_folliage
    }
    pub fn is_breakable(&self) -> bool {
        self.breakable
    }

    pub fn get_item(&self) -> Item {
        self.item
    }
    pub fn get_offset(&self) -> f32 {
        self.offset
    }

    pub fn can_collide(&self) -> bool {
        self.can_collide
    }
    pub fn can_place(&self) -> bool {
        self.placable
    }

    pub fn get_model(&self) -> String {
        self.block_model.to_string()
    }

    pub fn has_ambient_occlusion(&self) -> bool {
        self.ambient_occlusion
    }

    pub fn get_atlas_position(&self, face: usize) -> u16 {
        self.textures[face]
    }
}
