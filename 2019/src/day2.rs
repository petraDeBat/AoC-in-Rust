#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    // replace position 1 with the value 12
    // replace position 2 with the value 2.
    // What value is left at position 0 after the program halts?

    // go by 4's
    // position 0
    // 1 - add
    // 2 - multiply
    // 99 - ends program
    // position 1 and 2 -> represent the indices in which you either add/multiply
    // position 3 represents the index where you will overwrite with the sum/product
    let mut numbers: Vec<usize> = input
        .split(',')
        .map(|s| s.trim().parse().expect("Failed to parse number"))
        .collect();

    // do the replaces first
    numbers[1] = 12;
    numbers[2] = 2;

    println!("{:?}", numbers);

    // let mut counter = 0;
    let mut pos_0 = 0;
    let mut pos_1 = 1;
    let mut pos_2 = 2;
    let mut pos_3 = 3;

    loop {
        let output_index = numbers[pos_3];
        if numbers[pos_0] == 99 {
            break;
        } else if numbers[pos_0] == 1 {
            numbers[output_index] = numbers[numbers[pos_1]] + numbers[numbers[pos_2]];
        } else if numbers[pos_0] == 2 {
            numbers[output_index] = numbers[numbers[pos_1]] * numbers[numbers[pos_2]];
        } else {
            println!("Unexpected value for numbers[pos_0]: {}", numbers[pos_0])
        }

        // end of loop, update all counter values
        // counter += 1;
        pos_0 += 4;
        pos_1 += 4;
        pos_2 += 4;
        pos_3 += 4;
    }

    println!("{}", numbers[0]);
    numbers[0]
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
    // determine what pair of inputs produces the output 19690720
    // the value placed in address 1 is called the noun
    // the value placed in address 2 is called the verb
    // noun and verb should be between 0 and 99
    let target_output = 19690720;
    let mut numbers: Vec<i32> = input
        .split(',')
        .map(|s| s.trim().parse().expect("Failed to parse number"))
        .collect();
    
    // is there any way to work backwards from the target output?
    // LMAOOOO no i don't think so, let's brute force it
    
    let mut noun = 0;
    let mut verb = 0;

    for n in 0..100 {
        for v in 0..100 {
            // reset the numbers vector
            let mut temp_numbers = numbers.clone();
            temp_numbers[1] = n;
            temp_numbers[2] = v;

            let mut intruction_pointer = 0;

            loop {
                let parameter_a = temp_numbers[intruction_pointer + 1] as usize;
                let parameter_b = temp_numbers[intruction_pointer + 2] as usize;
                let output_index = temp_numbers[intruction_pointer + 3] as usize;
                if temp_numbers[intruction_pointer] == 99 {
                    break;
                } else if temp_numbers[intruction_pointer] == 1 {
                    temp_numbers[output_index] = temp_numbers[parameter_a] + temp_numbers[parameter_b];
                } else if temp_numbers[intruction_pointer] == 2 {
                    temp_numbers[output_index] = temp_numbers[parameter_a] * temp_numbers[parameter_b];
                } else {
                    println!("Unexpected value for numbers[intruction_pointer]: {}", temp_numbers[intruction_pointer]);
                }

                // end of loop, update instruction pointer by 4
                intruction_pointer += 4;
            }

            if temp_numbers[0] == target_output {
                noun = n;
                verb = v;
                break;
            }
        }
    }

    (100 * noun + verb) as i32
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

    #[test]
    fn sample1() {
        assert_eq!(part1("1,0,0,0,99"), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part1("1,1,1,4,99,5,6,0,99"), 30);
    }

    #[test]
    fn sample3() {
        assert_eq!(part1("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2("1, 0, 0, 0, 99"), 1202);
    }
}
