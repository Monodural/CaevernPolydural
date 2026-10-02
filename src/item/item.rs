use crate::block::block::Block;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Item {
    NONE, STONE, DIRT, GRASS_BLOCK, GRASS, FLOWER, OAK_LOG, OAK_LEAVES, LAMP, OAK_LEAVES_BLOCK, HOE, MAPLE_LEAVES_BLOCK,
    MAPLE_LEAVES, OAK_PLANKS, PEBBLE, CACTUS
}
impl Item {
    pub fn is_tool(self) -> bool {
        match self {
            Item::HOE => true,
            _ => false
        }
    }

    pub fn get_block(self) -> Block {
        match self {
            Item::STONE => Block::STONE,
            Item::DIRT => Block::DIRT,
            Item::GRASS_BLOCK => Block::GRASS_BLOCK_1,
            Item::GRASS => Block::GRASS_1,
            Item::FLOWER => Block::FLOWER_1,
            Item::OAK_LOG => Block::OAK_LOG,
            Item::OAK_LEAVES => Block::OAK_LEAVES,
            Item::LAMP => Block::LAMP,
            Item::OAK_LEAVES_BLOCK => Block::OAK_LEAVES_BLOCK,
            Item::HOE => Block::HOE,
            Item::MAPLE_LEAVES_BLOCK => Block::MAPLE_LEAVES_BLOCK,
            Item::MAPLE_LEAVES => Block::MAPLE_LEAVES,
            Item::OAK_PLANKS => Block::OAK_PLANKS,
            Item::PEBBLE => Block::PEBBLES,
            Item::CACTUS => Block::CACTUS,
            _ => Block::NONE,
        }
    }
}
