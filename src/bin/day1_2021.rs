// Day 1
// in RUST
// que the dance and music

use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

use itertools::Itertools;

fn main() {
    // parsing data
    if let Ok(lines) = read_lines("./day1_2021_input.txt") {
        // store the values... in a map? THE VALUES ARE NOT STRINGS, I need to upwrap them??
        // like a little xmas present 🎁
        let lines: Vec<String> = lines.map_while(Result::ok).collect();
        // bruh
        let  lines2 = lines.clone();
        part_1(lines);
        part_2(lines2);
    }
}

fn part_1(lines: Vec<String>) {
    // setting varibles
    // let mut num_increase = 0;
    // let mut curr_largest: i32;
    // let mut curr_num: i32;

    let lines2 = lines.clone();
    println!("The number of increases is {}", 
        lines2
            .iter()
            .map(|number| number.parse::<i32>().unwrap())
            .tuple_windows()
            .map(|(a, b)| if a < b { 1 } else { 0 }).sum::<i32>(),
    );

    // lines = lines.into_iter();

    // // get first number, make first line into number, please I beg of you
    // let num: String = lines.next().unwrap();
    // curr_largest = num.trim().parse().expect("Input not an integer");

    // // Consumes the iterator, returns an (Optional) String
    // for line in lines {
    //     curr_num = line.trim().parse().expect("Input not an integer");

    //     if curr_largest < curr_num {
    //         num_increase += 1;
    //     }
    //     curr_largest = curr_num; // only comparing with the previous number
    // }

    // // print result
    // println!("The number of increases was: {}", num_increase);
}

fn part_2(lines: Vec<String>) {
    let lines2 = lines.clone();
    println!("The number of increases for three-measurement sliding windows is {}", 
        lines2
            .iter()
            .map(|number| number.parse::<i32>().unwrap())
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .map(|(a, b)| if a < b { 1 } else { 0 }).sum::<i32>(),
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
