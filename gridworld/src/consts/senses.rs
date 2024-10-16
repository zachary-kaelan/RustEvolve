use std::f32::consts::PI;

pub const EYE_CELLS_PER_OCTANT: u32 = 5;
pub const EYE_CELLS_TOTAL: u32 = EYE_CELLS_PER_OCTANT * 8;
pub const EYE_CELL_FOV: f32 = PI / EYE_CELLS_TOTAL as f32;
pub const EYE_CELL_FOV_HALF: f32 = EYE_CELL_FOV / 2.0;
pub const EYE_RANGE: u16 = super::world::GRID_WIDTH / 4;
