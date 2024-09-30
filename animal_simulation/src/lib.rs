mod config;
mod parts;
mod simulation;
mod stats;
mod world;

pub use config::*;
pub use genetic_algorithm::*;
pub use neural_network::*;
pub use parts::*;
pub use simulation::*;
pub use world::*;

use nalgebra as na;
use neural_network as nn;
use rand::rngs::OsRng;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use std::f32::consts::*;
