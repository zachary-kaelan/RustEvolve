use crate::consts::brain_io::*;
use crate::consts::senses::*;
use crate::consts::world::GRID_WIDTH;
use crate::world::World;
use crate::*;
use std::f32::consts::PI;

pub struct Creature {
    pub memory: Box<[f32; CREATURE_MEMORY_SIZE]>,
    pub energy: u32,
    pub pos: GridPoint,
}

impl Default for Creature {
    fn default() -> Self {
        Self {
            memory: Box::new([0.0; CREATURE_MEMORY_SIZE]),
            energy: consts::energy::CREATURE_START_ENERGY,
            pos: GridPoint::new(GRID_WIDTH / 2, GRID_WIDTH / 2),
        }
    }
}

impl Creature {
    pub fn form_brain_inputs(&self, world: &World) -> [f32; INPUTS_SIZE] {
        let mut inputs = [0.0f32; INPUTS_SIZE];
        let mut ptr = 0usize;
        inputs[ptr + world[self.pos] as usize] = 1.0f32;
        ptr += NUM_TILES;

        let pts_in_range = get_points_in_range(self.pos, EYE_RANGE);
        let creatures_in_range = world.get_creatures_in_range(self.pos, EYE_RANGE);

        for pt in pts_in_range {
            let angle_diff = get_angle(self.pos, pt);
            let cell_index = ((angle_diff / (2.0 * PI)) * EYE_CELLS_TOTAL as f32).floor() as usize;
            let tile = world[pt];
            let slot_ptr = ptr + (cell_index * SLOTS_PER_EYE_CELL);
            let slot_1 = slot_ptr + (tile as u8) as usize;

            inputs[slot_1] = (1.0 / self.pos.dist(pt) as f32).max(inputs[slot_1]);
            inputs[slot_ptr + NUM_TILES + ((tile as u8) as usize)] += 1.0;
        }

        for creature_pos in creatures_in_range {
            let angle_diff = get_angle(self.pos, creature_pos);
            let cell_index = ((angle_diff / (2.0 * PI)) * EYE_CELLS_TOTAL as f32).floor() as usize;
            let slot_ptr = ptr + (cell_index * SLOTS_PER_EYE_CELL) + NUM_TILES * 2;

            inputs[slot_ptr] = (1.0 / self.pos.dist(creature_pos) as f32).max(inputs[slot_ptr]);
            inputs[slot_ptr + 1] += 1.0;
        }

        ptr += SLOTS_PER_EYE_CELL * EYE_CELLS_TOTAL as usize;

        for i in 0..CREATURE_MEMORY_SIZE {
            inputs[ptr + i] = self.memory[i];
        }

        inputs
    }
}
