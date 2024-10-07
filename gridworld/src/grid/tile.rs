use crate::grid_math::GridPoint;
use crate::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Tiles {
    EnergyGain,
    EnergyLoss,
    Wall,
    Floor,
}
