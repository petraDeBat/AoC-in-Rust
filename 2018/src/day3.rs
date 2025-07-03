use std::collections::HashMap;

use regex::Regex;

pub fn modify_claim_counter(claim_counter: &mut i32) {
    *claim_counter += 1;
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .fold(HashMap::new(), |mut hashmap, line| {
            let re = Regex::new(r"#\d+ @ (\d{1,}).(\d+). (\d+).(\d+)").unwrap();
            let captures = re.captures(line).unwrap();

            let x_pos: i32 = captures[1].parse().unwrap();
            let y_pos: i32 = captures[2].parse().unwrap();

            let x_wid: i32 = captures[3].parse().unwrap();
            let y_wid: i32 = captures[4].parse().unwrap();

            for y in y_pos..y_pos + y_wid {
                for x in x_pos..x_pos + x_wid {
                    hashmap
                        .entry((x, y))
                        .and_modify(modify_claim_counter)
                        .or_insert(1);

                    // if hashmap.get(&(x,y)).is_some() {
                    //     hashmap.insert((x,y), hashmap.get(&(x,y)).unwrap() + 1);
                    // } else {
                    //     hashmap.insert((x,y), 1);
                    // }

                    // If (x,y) in hashmap then hashmap[(x,y)] += 1 else hashmap[(x,y)] = 1
                }
            }

            hashmap
        })
        .values()
        .filter(|x| **x > 1)
        .count() as i32
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i32 {
    // let mut claim_id_clear: HashMap<i32, bool> = HashMap::new();

    *input
        .lines()
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut hashmap, mut claim_id_clear), line| {
                let re = Regex::new(r"#(\d+) @ (\d{1,}).(\d+). (\d+).(\d+)").unwrap();
                let captures = re.captures(line).unwrap();

                let claim_id: i32 = captures[1].parse().unwrap();

                let x_pos: i32 = captures[2].parse().unwrap();
                let y_pos: i32 = captures[3].parse().unwrap();

                let x_wid: i32 = captures[4].parse().unwrap();
                let y_wid: i32 = captures[5].parse().unwrap();

                for y in y_pos..y_pos + y_wid {
                    for x in x_pos..x_pos + x_wid {
                        // hashmap
                        //     .entry((x, y))
                        //     .and_modify(|claim_id_list: &mut Vec<i32>| claim_id_list.push(claim_id))
                        //     .or_insert(vec![claim_id]);

                        // If it's already in the hashmap, that means it's overlapping
                        // with at least something else, and we can set everything in
                        // the vec to false

                        // If this isn't the hashmap, and we haven't set the claim id to
                        // false already, then we can set it to true

                        if hashmap.contains_key(&(x, y)) {
                            let value: &i32 = hashmap.get(&(x, y)).unwrap();

                            claim_id_clear.insert(*value, false);
                            claim_id_clear.insert(claim_id, false);
                        } else {
                            // We need to add it to the hashmap
                            hashmap.insert((x, y), claim_id);

                            // If it's in the other one, then we shouldn't change it
                            // If it doesn't exist, we should set it to true
                            if !claim_id_clear.contains_key(&claim_id) {
                                claim_id_clear.insert(claim_id, true);
                            }
                        }
                    }
                }

                (hashmap, claim_id_clear)
            },
        )
        .1
        .iter()
        .find(|(id, overlapping)| **overlapping)
        .unwrap()
        .0
}
