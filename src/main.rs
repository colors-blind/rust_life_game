extern crate rand;
extern crate termion;
use std::{env, thread, time};
use termion::clear;
use termion::color;

mod world;
mod display;

use world::{populate_from_file, census, generation};
use display::displayworld;

fn main() {
    let mut world = [[0u8; 75]; 75];
    let mut generations = 0;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        for i in 0..74 {
            for j in 0..74 {
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename);
    }

    println!("Population at generation {} is {}", generations, census(world));
    for _gens in 0..100 {
        let temp = generation(world);
        world = temp;
        generations += 1;
        println!("{}", clear::All);
        displayworld(world);
        println!("{blue}Population at generation {g} is {c}", blue = color::Fg(color::Blue), g = generations, c = census(world));
        thread::sleep(time::Duration::from_secs(2));
    }

}
