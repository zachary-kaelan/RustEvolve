use crate::consts::senses::EYE_CELLS_TOTAL;

pub const NUM_TILES: usize = 4;
pub const SLOTS_PER_EYE_CELL: usize = (NUM_TILES + 1) * 2;
pub const INPUTS_SIZE: usize = (EYE_CELLS_TOTAL as usize) * (SLOTS_PER_EYE_CELL) + NUM_TILES;
pub const OUTPUTS_SIZE: usize = 8;
