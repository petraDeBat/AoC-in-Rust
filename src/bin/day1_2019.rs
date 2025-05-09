// Day 1 2019
// in RUST
// yippeeee

use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

fn main() {
    // Part 1:
    // find the fuel required for a module:
    // take its mass, divide by three, round down, and subtract 2
    // sum all the fuel values together
    if let Ok(lines) = read_lines("./input.txt") {
        // storing the values in a vector of strings (unsure if this is really needed or redundant)
        // then mapping them into f32 (in order to use .floor() later)
        // then mapping into their fuel value
        // then summing fuel values together
        let fuel: f32 = lines
            .map_while(Result::ok)
            .collect::<Vec<String>>()
            .iter()
            .map(|number| number.parse::<f32>().unwrap())
            .map(|mass| (mass / 3.0).floor() - 2.0)
            .sum();

        println!("Part 1: Total fuel needed is {}", fuel);
    }
    // Part 2:
    // same as part1 but continue mass calulation till you reach zero
    // sum ALLLLL the fuel values together

    // lol I got cooked with cloning and decided to move on with my life
    if let Ok(lines) = read_lines("./input.txt") {
        // storing the values in a vector of strings (unsure if this is really needed or redundant)
        // then mapping them into f32 (in order to use .floor() later)
        // then mapping into their fuel value
        // then summing fuel values together
        let fuel: f32 = lines
            .map(|mass| Fuel::new(mass.unwrap().parse().unwrap()).sum::<f32>())
            .sum();

        println!("Part 2: Total fuel needed is {}", fuel);
    
    }

    // testing my fuel function
    // assert_eq!(calculate_fuel(14.0), 2.0);
    // assert_eq!(calculate_fuel(100756.0), 50346.0);

    // testing Fuel iter
    let test_mass: f32 = 100756.0;
    println!(
        "result of Fuel iter sum??:  {}",
        Fuel::new(test_mass).sum::<f32>()
    );
}

// this does not work, idk whats going on the fuel sum returning
// fn calculate_fuel(mut mass: f32) -> f32 {
//     let mut fuel_sum = 0.0;
//     while mass >= 0.0 {
//         mass = (mass / 3.0).floor() - 2.0;
//         fuel_sum += mass;
//         if mass <= 0.0 {
//             return fuel_sum;
//         }
//     }
//     return fuel_sum;
// }

struct Fuel {
    curr: f32,
}
impl Fuel {
    fn new(curr: f32) -> Fuel {
        Fuel { curr }
    }
}

impl Iterator for Fuel {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = (current / 3.0).floor() - 2.0;

        // Endpoint to the mass fuel sequence
        if self.curr <= 0.0 {
            None
        } else {
            Some(self.curr)
        }
    }
}

// fn fuel(mass: f32) -> Fuel {
// Fuel { curr: mass }
// }

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
