use gridworld::{INPUTS_SIZE, OUTPUTS_SIZE};

#[derive(Clone, Debug)]
pub struct Config {
    pub brain_neurons: usize,

    pub ga_mut_chance: f32,
    pub ga_mut_coeff: f32,

    pub sim_generation_length: usize,

    pub world_creatures: usize,
    pub world_foods: usize,

    pub enable_memory: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            brain_neurons: (INPUTS_SIZE + OUTPUTS_SIZE) / 2 + 1,
            ga_mut_chance: 0.01,
            ga_mut_coeff: 0.3,
            sim_generation_length: 500,
            world_creatures: 128,
            world_foods: 128,
            enable_memory: false,
        }
    }
}
