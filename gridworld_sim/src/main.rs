mod config;
mod parts;
mod simulation;
mod stats;

pub use genetic_algorithm::*;
pub use gridworld::*;
pub use neural_network::*;
use rand::rngs::OsRng;

pub use config::*;
pub use parts::*;

use crate::simulation::Simulation;

#[macroquad::main("Griworld")]
async fn main() {
    use macroquad::prelude::*;

    let config = Config::default();
    let mut simulation = Simulation::random(&config);

    let floor_color = Color::from_rgba(255, 255, 255, 255);
    let wall_color = Color::from_rgba(64, 64, 64, 255);
    let food_color = Color::from_rgba(0, 128, 0, 255);
    let creature_color = Color::from_rgba(0, 0, 255, 255);

    let mut active = true;
    let rng = &mut OsRng;

    loop {
        clear_background(GRAY);
        if is_key_pressed(KeyCode::T) {
            for i in 0..25 {
                if i > 0 {
                    println!();
                }

                let stats = simulation.train(rng);
                println!("{}", stats);
            }
        }

        if is_key_pressed(KeyCode::P) {
            active = !active;
        }

        if active {
            let stats = simulation.step(rng);

            if let Some(stats) = stats {
                println!("{}", stats);
            }
        }

        //if simulation.generation >= 5 {
        //    break;
        //}

        next_frame().await
    }
}
