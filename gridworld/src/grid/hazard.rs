use crate::GridPoint;

#[derive(Copy, Clone, Debug)]
pub struct Hazard {
    pub walkable: bool,
    pub damage: u8,
    pub pos: GridPoint,
}
