use macroquad::prelude::*;
use crate::world::{World, SIZE};

pub fn draw_grid(world: &World, cell_size: f32, offset_x: f32, offset_y: f32) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            let x = offset_x + j as f32 * cell_size;
            let y = offset_y + i as f32 * cell_size;

            if world[i][j] == 1 {
                draw_rectangle(x, y, cell_size - 1.0, cell_size - 1.0, GREEN);
            } else {
                draw_rectangle(x, y, cell_size - 1.0, cell_size - 1.0, Color::new(0.1, 0.1, 0.1, 1.0));
            }
        }
    }
}

pub fn draw_hud(gen: u64, population: u32, paused: bool, speed: f64) {
    let status = if paused { "PAUSED" } else { "RUNNING" };
    let text = format!(
        "Gen: {}  Pop: {}  Speed: {:.1}x  [{}]  |  Esc/Q:quit  Space:pause  Click:toggle  R:random  C:clear  +/-:speed",
        gen, population, speed, status
    );
    draw_text(&text, 10.0, 20.0, 18.0, WHITE);
}
