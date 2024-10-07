use crate::*;
use consts::world::*;
use std::cmp::PartialEq;
use std::ops::Index;

pub struct World {
    map: [Tiles; GRID_AREA],
    creatures: Vec<Creature>,
}

impl Index<GridPoint> for World {
    type Output = Tiles;

    fn index(&self, index: GridPoint) -> &Self::Output {
        &self.map[(index.x * 256) as usize + index.y as usize]
    }
}

// initialization functions
impl World {
    pub fn new(map: [Tiles; GRID_AREA]) -> Self {
        Self {
            map,
            creatures: vec![],
        }
    }

    pub fn default() -> Self {
        Self::filled_with(Tiles::Floor)
    }

    pub fn filled_with(tile: Tiles) -> Self {
        Self {
            map: [tile; GRID_AREA],
            creatures: vec![],
        }
    }

    pub fn set_border(&mut self, tile: Tiles) {
        for x in 0..GRID_WIDTH {
            self.set(pt!(x, 0), tile);
            self.set(pt!(x, GRID_WIDTH - 1), tile);
        }

        for y in 1..GRID_WIDTH - 1 {
            self.set(pt!(0, y), tile);
            self.set(pt!(GRID_WIDTH - 1, y), tile);
        }
    }
}

impl World {
    fn set(&mut self, pt: GridPoint, tile: Tiles) {
        self.map[(pt.x * 256) as usize + pt.y as usize] = tile;
    }

    pub fn get_creatures_in_range(&self, pos: GridPoint, max_range: u16) -> Vec<GridPoint> {
        let mut points = vec![];
        for creature in &self.creatures {
            let dist = pos.dist(creature.pos);
            if 0 < dist && dist <= max_range {
                points.push(creature.pos);
            }
        }
        points
    }
}
