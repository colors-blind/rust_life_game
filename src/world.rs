use std::fs::File;
use std::io::{BufRead, BufReader};

pub const SIZE: usize = 75;

pub type World = [[u8; SIZE]; SIZE];

pub fn new_world() -> World {
    [[0u8; SIZE]; SIZE]
}

pub fn populate_random() -> World {
    let mut world = new_world();
    for i in 0..SIZE {
        for j in 0..SIZE {
            world[i][j] = if macroquad::rand::gen_range(0u8, 2) == 1 { 1 } else { 0 };
        }
    }
    world
}

pub fn populate_from_file(filename: &str) -> World {
    let mut world = new_world();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap();
        let mut words = l.split_whitespace();
        let x: usize = words.next().unwrap().parse().unwrap();
        let y: usize = words.next().unwrap().parse().unwrap();
        if x < SIZE && y < SIZE {
            world[x][y] = 1;
        }
    }
    world
}

pub fn census(world: &World) -> u32 {
    let mut count = 0u32;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if world[i][j] == 1 {
                count += 1;
            }
        }
    }
    count
}

pub fn generation(world: &World) -> World {
    let mut newworld = new_world();

    for i in 0..SIZE {
        for j in 0..SIZE {
            let mut count: u8 = 0;
            for di in -1i32..=1 {
                for dj in -1i32..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < SIZE as i32 && nj >= 0 && nj < SIZE as i32 {
                        count += world[ni as usize][nj as usize];
                    }
                }
            }

            if world[i][j] == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            } else if world[i][j] == 0 && count == 3 {
                newworld[i][j] = 1;
            }
        }
    }
    newworld
}
