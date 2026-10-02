use crate::block::block::Block;

pub fn get_block_damage_model(block: Block, block_damage: u8) -> String {
    match block {
        Block::OAK_LOG_NORTH => {
            match block_damage {
                1 => "break1north".to_string(),
                2 => "break2north".to_string(),
                3 => "break3north".to_string(),
                4 => "break4north".to_string(),
                5 => "break5north".to_string(),
                6 => "break6north".to_string(),
                7 => "break7north".to_string(),
                8 => "break8north".to_string(),
                _ => block.properties().get_model(),
            }
        }
        Block::OAK_LOG_EAST => {
            match block_damage {
                1 => "break1east".to_string(),
                2 => "break2east".to_string(),
                3 => "break3east".to_string(),
                4 => "break4east".to_string(),
                5 => "break5east".to_string(),
                6 => "break6east".to_string(),
                7 => "break7east".to_string(),
                8 => "break8east".to_string(),
                _ => block.properties().get_model(),
            }
        }
        Block::OAK_LOG => {
            match block_damage {
                1 => "break1up".to_string(),
                2 => "break2up".to_string(),
                3 => "break3up".to_string(),
                4 => "break4up".to_string(),
                5 => "break5up".to_string(),
                6 => "break6up".to_string(),
                7 => "break7up".to_string(),
                8 => "break8up".to_string(),
                _ => block.properties().get_model(),
            }
        }
        _ => {
            match block_damage {
                1 => "break1".to_string(),
                2 => "break2".to_string(),
                3 => "break3".to_string(),
                4 => "break4".to_string(),
                5 => "break5".to_string(),
                6 => "break6".to_string(),
                7 => "break7".to_string(),
                8 => "break8".to_string(),
                _ => block.properties().get_model(),
            }
        }
    }
}
