use macroquad::prelude::*;

mod world;
mod gui;

use world::{World, SIZE, populate_random, populate_from_file, census, generation, new_world};
use gui::{draw_grid, draw_hud};

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_string(),
        window_width: 830,
        window_height: 860,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut world: World = if args.len() >= 2 {
        populate_from_file(&args[1])
    } else {
        populate_random()
    };

    let mut generations: u64 = 0;
    let mut paused = false;
    let mut speed: f64 = 1.0;
    let mut timer: f64 = 0.0;
    let base_interval: f64 = 0.15;

    loop {
        clear_background(BLACK);

        let cell_size = {
            let w = screen_width() / SIZE as f32;
            let h = (screen_height() - 30.0) / SIZE as f32;
            w.min(h)
        };
        let grid_w = cell_size * SIZE as f32;
        let offset_x = (screen_width() - grid_w) / 2.0;
        let offset_y = 30.0;

        // input handling
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            world = populate_random();
            generations = 0;
        }
        if is_key_pressed(KeyCode::C) {
            world = new_world();
            generations = 0;
        }
        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            speed = (speed + 0.5).min(10.0);
        }
        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            speed = (speed - 0.5).max(0.5);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = ((mx - offset_x) / cell_size) as i32;
            let gy = ((my - offset_y) / cell_size) as i32;
            if gx >= 0 && gx < SIZE as i32 && gy >= 0 && gy < SIZE as i32 {
                let row = gy as usize;
                let col = gx as usize;
                world[row][col] ^= 1;
            }
        }

        // evolution
        if !paused {
            timer += get_frame_time() as f64;
            let interval = base_interval / speed;
            if timer >= interval {
                world = generation(&world);
                generations += 1;
                timer = 0.0;
            }
        }

        draw_grid(&world, cell_size, offset_x, offset_y);
        draw_hud(generations, census(&world), paused, speed);

        next_frame().await;
    }
}
