mod config;
mod parts;
mod simulation;
mod stats;

pub use genetic_algorithm::*;
pub use gridworld::*;
use macroquad::miniquad::window::{screen_size, set_window_size};
pub use neural_network::*;
use rand::rngs::OsRng;

pub use config::*;
pub use parts::*;

use crate::simulation::Simulation;

#[macroquad::main("Gridworld")]
async fn main() {
    use macroquad::prelude::*;

    let config = Config::default();
    println!(
        "{} -> {} -> {}",
        INPUTS_SIZE, config.brain_neurons, OUTPUTS_SIZE
    );
    let mut simulation = Simulation::random(&config);

    let grid_color = Color::from_rgba(255, 255, 255, 255);
    let floor_color = Color::from_rgba(128, 128, 128, 255);
    let wall_color = Color::from_rgba(64, 64, 64, 255);
    let food_color = Color::from_rgba(0, 128, 0, 255);
    let hazard_color = Color::from_rgba(255, 0, 0, 255);
    let creature_color = Color::from_rgba(0, 0, 255, 255);

    let mut frame_index = 0usize;

    let mut active = true;
    let mut display = true;
    let rng = &mut OsRng;

    let screen_width = 3440;
    let screen_height = 1440;

    let window_size = {
        let size = screen_width.min(screen_height) as u16;
        size - (size % GRID_WIDTH)
    };
    set_window_size(window_size as u32, window_size as u32);
    let square_size = (window_size / GRID_WIDTH) as f32;

    let mut display_index = 0;
    loop {
        clear_background(GRAY);
        if is_key_pressed(KeyCode::T) || (!display && display_index % 2 == 0) {
            for i in 0..25 {
                if i > 0 {
                    println!();
                }

                let stats = simulation.train(rng);
                println!("{}", stats);
            }

            display_index += 1;
        }

        if is_key_pressed(KeyCode::P) {
            active = !active;
        }

        if is_key_pressed(KeyCode::D) {
            display = !display;
        }

        if active {
            let stats = simulation.step(rng);

            if let Some(stats) = stats {
                println!("{}", stats);
                display_index += 1;
            }

            if display {
                for x in 0..GRID_WIDTH {
                    for y in 0..GRID_WIDTH {
                        let pt = pt!(x, y);
                        let tile = simulation.world()[pt];
                        let color = match tile {
                            Tiles::EnergyGain => food_color,
                            Tiles::EnergyLoss => hazard_color,
                            Tiles::Wall => wall_color,
                            Tiles::Floor => floor_color,
                        };

                        draw_rectangle(
                            x as f32 * square_size,
                            y as f32 * square_size,
                            square_size,
                            square_size,
                            color,
                        );
                    }
                }

                for creature in &simulation.world().creatures {
                    let pt = creature.pos.get();
                    draw_rectangle(
                        pt.x as f32 * square_size,
                        pt.y as f32 * square_size,
                        square_size,
                        square_size,
                        creature_color,
                    );
                }

                for x in 0..GRID_WIDTH {
                    draw_line(
                        x as f32 * square_size,
                        0.0,
                        x as f32 * square_size,
                        GRID_WIDTH as f32 * square_size,
                        1.0,
                        grid_color,
                    );
                }

                for y in 0..GRID_WIDTH {
                    draw_line(
                        0.0,
                        y as f32 * square_size,
                        GRID_WIDTH as f32 * square_size,
                        y as f32 * square_size,
                        1.0,
                        grid_color,
                    );
                }
            }
        }

        //if simulation.generation >= 5 {
        //    break;
        //}

        frame_index += 1;
        next_frame().await
    }
}
