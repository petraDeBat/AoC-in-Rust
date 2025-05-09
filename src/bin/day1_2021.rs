// Day 1 2021
// in RUST
// que the dance and music

use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

use itertools::Itertools;

fn main() {
    // parsing data
    if let Ok(lines) = read_lines("./input.txt") {
        // store the values... in a map? THE VALUES ARE NOT STRINGS, I need to upwrap them??
        // like a little xmas present 🎁
        let lines: Vec<String> = lines.map_while(Result::ok).collect();
        // bruh
        let lines2 = lines.clone();
        part_1(lines);
        part_2(lines2);
    }
}

fn part_1(lines: Vec<String>) {
    let lines2 = lines.clone();
    println!(
        "The number of increases is {}",
        lines2
            .iter()
            .map(|number| number.parse::<i32>().unwrap())
            .tuple_windows()
            .map(|(a, b)| if a < b { 1 } else { 0 })
            .sum::<i32>(),
    );
}

fn part_2(lines: Vec<String>) {
    let lines2 = lines.clone();
    println!(
        "The number of increases for three-measurement sliding windows is {}",
        lines2
            .iter()
            .map(|number| number.parse::<i32>().unwrap())
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .map(|(a, b)| if a < b { 1 } else { 0 })
            .sum::<i32>(),
    );
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
