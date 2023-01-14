use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Instruction {
    register: String,
    inc: bool,
    value: i32,
    register_if: String,
    is_if: String,
    value_if: i32,
}

impl Instruction {
    fn new(
        register: String,
        inc: bool,
        value: i32,
        register_if: String,
        is_if: String,
        value_if: i32,
    ) -> Instruction {
        Instruction {
            register,
            inc,
            value,
            register_if,
            is_if,
            value_if,
        }
    }

    fn check(&self, registers: &HashMap<&str, i32>) -> bool {
        let register_if = if registers.contains_key(&self.register_if as &str) {
            *registers.get(&self.register_if as &str).unwrap()
        } else {
            0
        };
        match self.is_if.as_str() {
            "==" => register_if == self.value_if,
            ">" => register_if > self.value_if,
            "<" => register_if < self.value_if,
            ">=" => register_if >= self.value_if,
            "<=" => register_if <= self.value_if,
            "!=" => register_if != self.value_if,
            _ => false,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    let instructions = parse(&input);
    let result = solve(&instructions);
    println!("Silver: {}", result.0);
    println!("Gold: {}", result.1);
}

fn solve(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut m = 0;
    for instruction in instructions {
        if !registers.contains_key(&instruction.register as &str) {
            registers.insert(&instruction.register, 0);
        }
        if instruction.check(&registers) {
            registers.insert(
                &instruction.register,
                registers.get(&instruction.register as &str).unwrap()
                    + if instruction.inc {
                        instruction.value
                    } else {
                        -instruction.value
                    },
            );
            if m < *registers.get(&instruction.register as &str).unwrap() {
                m = *registers.get(&instruction.register as &str).unwrap();
            }
        }
    }
    (*registers.values().max().unwrap(), m)
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let regex = Regex::new(r"(\w+) (inc|dec) (.*+) if (\w+) (.*) (.*)").unwrap();
    for cap in regex.captures_iter(&input) {
        instructions.push(Instruction::new(
            String::from(&cap[1]),
            if &cap[2] == "inc" { true } else { false },
            cap[3].parse::<i32>().unwrap(),
            String::from(&cap[4]),
            String::from(&cap[5]),
            cap[6].parse::<i32>().unwrap(),
        ));
    }
    instructions
}
