use crate::*;
use consts::world::*;
use rand::{Rng, RngCore};
use std::cmp::PartialEq;
use std::ops::Index;

pub struct World<T> {
    map: [Tiles; GRID_AREA],
    pub creatures: Vec<Creature<T>>,
}

impl<T> Index<GridPoint> for World<T> {
    type Output = Tiles;

    fn index(&self, index: GridPoint) -> &Self::Output {
        &self.map[(index.x * GRID_WIDTH) as usize + index.y as usize]
    }
}

// initialization functions
impl<T> World<T> {
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

    pub fn get_random_free_pt(&self, rng: &mut dyn RngCore) -> GridPoint {
        loop {
            let pt = GridPoint::rand(rng);
            if self[pt] == Tiles::Floor && !self.creatures.iter().any(|x| x.pos == pt) {
                break pt;
            }
        }
    }

    pub fn place_random_food(&mut self, amount: usize, rng: &mut dyn RngCore) {
        for _ in 0..amount {
            let pt = self.get_random_free_pt(rng);
            self.set(pt, Tiles::EnergyGain);
        }
    }
}

impl<T> World<T> {
    fn set(&mut self, pt: GridPoint, tile: Tiles) {
        self.map[(pt.x * GRID_WIDTH) as usize + pt.y as usize] = tile;
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
