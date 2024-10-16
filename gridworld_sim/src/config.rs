use gridworld::{CREATURE_MEMORY_SIZE, INPUTS_SIZE, OUTPUTS_SIZE};

#[derive(Clone, Debug)]
pub struct Config {
    pub start_fitness: f32,

    pub brain_neurons: usize,

    pub ga_mut_chance: f32,
    pub ga_mut_coeff: f32,

    pub sim_generation_length: usize,

    pub world_creatures: usize,
    pub world_foods: usize,
    pub world_lava: usize,

    pub enable_memory: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_fitness: 0.0,
            brain_neurons: 128,
            ga_mut_chance: 0.025,
            ga_mut_coeff: 0.5,
            sim_generation_length: 500,
            world_creatures: 96,
            world_foods: 128,
            world_lava: 512,
            enable_memory: false,
        }
    }
}
