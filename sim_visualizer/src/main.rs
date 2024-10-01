use animal_simulation::*;
use macroquad::prelude::*;
use std::f32::consts::PI;

use ::rand::rngs::OsRng;

fn draw_triangle_rotated(x: f32, y: f32, size: f32, angle: f32, color: Color) {
    let v1 = vec2(x - angle.sin() * size * 1.5, y + angle.cos() * size * 1.5);
    let angle2 = angle + 2.0 / 3.0 * PI;
    let v2 = vec2(x - angle2.sin() * size, y + angle2.cos() * size);
    let angle3 = angle - 2.0 / 3.0 * PI;
    let v3 = vec2(x - angle3.sin() * size, y + angle3.cos() * size);

    draw_triangle_lines(v1, v2, v3, 1.0, color);
}

#[macroquad::main("Simulation")]
async fn main() {
    let config = Config::default();
    let mut simulation = Simulation::random(&config);
    let food_color = Color::from_rgba(0, 255, 128, 255);
    let animal_color = Color::from_rgba(255, 255, 255, 255);
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

        for food in simulation.world().foods() {
            let x = food.position().x * screen_width();
            let y = food.position().y * screen_width();
            //println!("{}, {}, {}", food.position(), x, y);
            draw_circle(x, y, config.food_size / 2.0 * screen_width(), food_color);
        }

        for animal in simulation.world().animals() {
            let mut r = 64u8;
            let mut g = 0u8;
            let mut b = 64u8;

            if animal.stunned > 0 {
                r = 0;
                g = 0;
                b = 0;
            } else {
                if animal.boosting {
                    r = 128;
                }
                g = (animal.satiation * 8).min(255usize) as u8;
            }

            draw_triangle_rotated(
                animal.position().x * screen_width(),
                animal.position().y * screen_width(),
                config.food_size * screen_width(),
                animal.rotation().angle(),
                Color::from_rgba(r, g, b, 255),
            );

            let angle_per_cell = config.eye_fov_angle / (config.eye_cells as f32);

            for cell_id in (0..config.eye_cells) {
                let angle_from = animal.rotation().angle() - config.eye_fov_angle / 2.0
                    + (cell_id as f32) * angle_per_cell
                    + PI / 2.0;
                let angle_to = angle_from + angle_per_cell;

                let energy = animal.vision()[cell_id].round();

                draw_arc(
                    animal.position().x * screen_width(),
                    animal.position().y * screen_width(),
                    5,
                    config.food_size * 2.5 * screen_width(),
                    angle_from,
                    1.0,
                    angle_per_cell,
                    Color::from_rgba(0, 255, 128, energy as u8),
                );
            }
        }

        //if simulation.generation >= 5 {
        //    break;
        //}

        next_frame().await
    }
}
