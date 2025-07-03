// hello, I am so cooked 😩 😩 😩

// a struct for the instruction pointer, program memory
#[derive(Debug)]
struct Yumyumcode {
    instruction_pointer: usize,
    program_memory: Vec<i32>, // the whole entire input, but will be modified
}

impl Yumyumcode {

    fn new(mut program_memory: Vec<i32>) -> Self {
        program_memory.extend(std::iter::repeat(0).take(30_000));
        Yumyumcode { instruction_pointer: 0, program_memory}
    }

    // method to access and write a value at a direct location
    fn write_yumyum(&mut self, location: usize, new_val: i32, parameter: i32) {
        if parameter == 1 {
            self.program_memory[location] = new_val;
        } else {
            let index = self.read_yumyum(location, parameter); 
            self.program_memory[index] = new_val;
        }
    }    

    // method to read value at given location
    fn read_yumyum(&mut self, location: usize, parameter: i32) -> usize {
        if parameter == 1 {
            self.program_memory[location] as usize
        } else {
            let index = self.program_memory[location] as usize;
            self.program_memory[index] as usize
        }
    }


    // parsing opcode
    fn given_opcode(&mut self, instruction: Instruction) {

        match instruction.opcode {
            1 => self.opcode_1(instruction),
            2 => self.opcode_2(instruction),
            3 => self.opcode_3(instruction),
            4 => self.opcode_4(instruction),
            99 => panic!("halp"),
            _ => { 
                dbg!(instruction);
                dbg!(self.instruction_pointer);
                unreachable!();
            },
        }
    }

    // addition
    fn opcode_1(&mut self, instruction: Instruction) {
        let mut a = 0;
        let mut b = 0;

        a = self.read_yumyum(self.instruction_pointer+1, instruction.a);
        b = self.read_yumyum(self.instruction_pointer+2, instruction.b);

        let new_val: i32 = a as i32 + b as i32;
        self.write_yumyum(self.instruction_pointer+3, new_val, instruction.c);

        self.instruction_pointer = self.instruction_pointer+4;
    }

    // multiply  
    fn opcode_2(&mut self, instruction: Instruction) {
        let mut a = 0;
        let mut b = 0;

        a = self.read_yumyum(self.instruction_pointer+1, instruction.a);
        b = self.read_yumyum(self.instruction_pointer+2, instruction.b);

        let new_val: i32 = a as i32 * b as i32;
        self.write_yumyum(self.instruction_pointer+3, new_val, instruction.c);

        self.instruction_pointer = self.instruction_pointer+4;
    }

    // input 
    fn opcode_3(&mut self, instruction: Instruction) {
        self.write_yumyum(self.instruction_pointer+1, 1, instruction.c);

        self.instruction_pointer = self.instruction_pointer+2;
    }

    // output
    fn opcode_4(&mut self, instruction: Instruction) {
        let the_answer = self.read_yumyum(self.instruction_pointer+1, instruction.a);
        println!("The opcode 4 for instruction pointer: {}: {the_answer}", self.instruction_pointer);

        self.instruction_pointer = self.instruction_pointer+2;
    }
    
    fn run(&mut self) {
        loop {
            self.given_opcode(Instruction::new(self.program_memory[self.instruction_pointer] as i32));
        }
    }


}

#[derive(Debug)]
struct Instruction {
    opcode: i32, // de
    a: i32, // mode of 3rd parameter
    b: i32, // mode of 2nd parameter
    c: i32, // mode of 1st parameter
}

impl Instruction {
    fn new(mut number: i32) -> Self {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        if number >= 10_000 {
            a = 1;
            number -= 10_000;
        } 
        if number >= 1_000 {
            b = 1;
            number -= 1_000;
        }
        if number >= 100 {
            c = 1;
            number -= 100;
        }
        let opcode = number;


        Instruction {
            opcode, a, b, c 
        }
    }
}


#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let numbers: Vec<i32> = input
    .split(',')
    .map(|s| s.trim().parse().expect("Failed to parse number"))
    .collect();

    let mut runner = Yumyumcode::new(numbers);
    runner.run();
    return 3

}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i32 {
    return 3
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

    #[test]
    fn sample1() {
        assert_eq!(part1("1002,4,3,4,33"), 2);
    }

}