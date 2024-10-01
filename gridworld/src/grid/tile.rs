use crate::grid_math::GridPoint;
use crate::*;

pub trait TileTrait {
    fn is_walkable(&self) -> bool;
    fn get_pos(&self) -> (u8, u8);
}

pub struct Tile {
    pub tile: Tiles,
    pub pos: GridPoint,
}

impl Tile {
    pub fn is_walkable(&self) -> bool {
        self.tile.is_walkable()
    }
}

pub enum Tiles {
    Reward(Reward),
    Creature(Creature),
    Hazard(Hazard),
    Wall,
    Floor,
}

impl Tiles {
    pub fn is_walkable(&self) -> bool {
        match self {
            Tiles::Reward(t) => t.walkable,
            Tiles::Creature(_) => false,
            Tiles::Hazard(t) => t.walkable,
            Tiles::Wall => false,
            Tiles::Floor => true,
        }
    }
}
