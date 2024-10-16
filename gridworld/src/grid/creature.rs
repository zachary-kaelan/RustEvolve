use crate::consts::brain_io::*;
use crate::consts::energy::CREATURE_START_ENERGY;
use crate::consts::senses::*;
use crate::consts::world::GRID_WIDTH;
use crate::world::World;
use crate::*;
use std::cell::Cell;
use std::f32::consts::PI;
use std::rc::{Rc, Weak};

pub struct Creature<T> {
    pub memory: Cell<[f32; CREATURE_MEMORY_SIZE]>,
    pub energy: f32,
    pub pos: Cell<GridPoint>,
    pub target_pos: Cell<Option<GridPoint>>,
    pub brain: T,
}

impl<T> Creature<T> {
    pub fn new(pos: GridPoint, brain: T) -> Self {
        Self {
            memory: Cell::new([0.0; CREATURE_MEMORY_SIZE]),
            energy: CREATURE_START_ENERGY,
            pos: Cell::new(pos),
            target_pos: Cell::new(None),
            brain,
        }
    }
}

impl<T> Creature<T> {
    pub fn form_brain_inputs(&self, world: &World<T>) -> [f32; INPUTS_SIZE] {
        let mut inputs = [0.0f32; INPUTS_SIZE];
        let mut ptr = 0usize;
        inputs[ptr + world[self.pos.get()] as usize] = 1.0f32;
        ptr += NUM_TILES;

        let pts_in_range = get_points_in_range(self.pos.get(), EYE_RANGE);
        let creatures_in_range = world.get_creatures_in_range(self.pos.get(), EYE_RANGE);

        for pt in pts_in_range {
            if self.pos.get().dist(pt) == 0 {
                continue;
            }

            let angle_diff = (get_angle(self.pos.get(), pt) + PI) % (PI * 2.0);
            let cell_index = ((angle_diff / (PI * 2.0)) * EYE_CELLS_TOTAL as f32).floor() as usize;
            let tile = world[pt];
            let slot_ptr = ptr + (cell_index * SLOTS_PER_EYE_CELL);
            let slot_1 = slot_ptr + (tile as u8) as usize;

            inputs[slot_1] = (1.0 / self.pos.get().dist(pt) as f32).max(inputs[slot_1]);
            inputs[slot_ptr + NUM_TILES + ((tile as u8) as usize)] += 1.0;
            let dist_metric = 1.0 / self.pos.get().dist(pt) as f32;
            inputs[slot_1] = dist_metric.max(inputs[slot_1]);
        }

        for creature_pos in creatures_in_range {
            let angle_diff = (get_angle(self.pos.get(), creature_pos) + PI) % (PI * 2.0);
            let cell_index = ((angle_diff / (PI * 2.0)) * EYE_CELLS_TOTAL as f32).floor() as usize;
            let slot_ptr = ptr + (cell_index * SLOTS_PER_EYE_CELL) + NUM_TILES; // * 2;

            inputs[slot_ptr] =
                (1.0 / self.pos.get().dist(creature_pos) as f32).max(inputs[slot_ptr]);
            inputs[slot_ptr + 1] += 1.0;
            let dist_metric = 1.0 / self.pos.get().dist(creature_pos) as f32;
            inputs[slot_ptr] = dist_metric.max(inputs[slot_ptr]);
        }

        if CREATURE_MEMORY_SIZE > 0 {
            ptr += SLOTS_PER_EYE_CELL * EYE_CELLS_TOTAL as usize;

            inputs[ptr..(CREATURE_MEMORY_SIZE + ptr)]
                .copy_from_slice(&self.memory.get()[..CREATURE_MEMORY_SIZE]);
        }

        inputs
    }
}
