use std::collections::{hash_set, HashSet};

use regex::Regex;

#[derive(Debug)]
struct Light {
    x_pos: i32,
    y_pos: i32,
    x_velo: i32,
    y_velo: i32,
}

impl Light {
    fn new(x_pos: i32, y_pos: i32, x_velo: i32, y_velo: i32) -> Self {
        Light {
            x_pos: x_pos,
            y_pos: y_pos,
            x_velo: x_velo,
            y_velo: y_velo,
        }
    }

    fn take_step(&mut self) {
        self.x_pos += self.x_velo;
        self.y_pos += self.y_velo;
    }
}

pub fn visualize_grid(lights: &mut Vec<Light>) {
    let light_positions: HashSet<(i32, i32)> = lights.iter().map(|light| (light.x_pos, light.y_pos)).collect();
    for y in -300..300 {
        for x in -300..300 {
            match light_positions.contains(&(x,y)) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut lights = input
        .lines()
        .fold(Vec::new(), |mut lights: Vec<Light>, line| {
            let re =
                Regex::new(r"position=<\s*(-?\d+),\s+(-?\d+)>\s+velocity=<\s*(-?\d+),\s+(-?\d+)>")
                    .unwrap();
            let captures = re.captures(line).unwrap();

            let x_pos: i32 = captures[1].parse().unwrap();
            let y_pos: i32 = captures[2].parse().unwrap();

            let x_velo: i32 = captures[3].parse().unwrap();
            let y_velo: i32 = captures[4].parse().unwrap();

            // add to lights list: light struct
            lights.push(Light::new(x_pos, y_pos, x_velo, y_velo));

            lights
        });

    for i in 0..11_000 {
        for light in lights.iter_mut() {
            light.take_step();
        } // ur cute <3 <3
        println!("{}", i);
        if i > 10_900 {
            visualize_grid(&mut lights);
        }
    }

    dbg!(lights);

    3
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> i32 {
    3
}
