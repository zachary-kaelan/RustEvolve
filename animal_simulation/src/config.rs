use crate::*;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::cell::Cell;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub brain_neurons: usize,

    pub eye_fov_range: f32,
    pub eye_fov_angle: f32,
    pub eye_cells: usize,

    pub food_size: f32,
    pub animal_size: f32,
    pub arc_size: f32,

    pub ga_reverse: usize,
    pub ga_mut_chance: f32,
    pub ga_mut_coeff: f32,

    pub sim_speed_min: f32,
    pub sim_speed_max: f32,
    pub sim_speed_accel: f32,
    pub sim_rotation_accel: f32,
    pub sim_generation_length: usize,

    pub stun_duration: u8,
    pub stun_cooldown: u8,
    pub boost_cost: f32,

    pub world_animals: usize,
    pub world_foods: usize,

    pub window_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            brain_neurons: 27,
            //
            eye_fov_range: 0.25,
            eye_fov_angle: PI + FRAC_PI_4,
            eye_cells: 9,
            //
            food_size: 0.01,
            animal_size: 0.02,
            arc_size: 0.05,
            //
            ga_reverse: 0,
            ga_mut_chance: 0.015,
            ga_mut_coeff: 0.3,
            //
            sim_speed_min: 0.001,
            sim_speed_max: 0.004,
            sim_speed_accel: 0.2,
            sim_rotation_accel: FRAC_PI_2,
            sim_generation_length: 2500,
            //
            stun_duration: 15,
            stun_cooldown: 15,
            boost_cost: 0.015,
            //
            world_animals: 20,
            world_foods: 20,
            //
            window_size: 640,
        }
    }
}
