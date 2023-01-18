use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum InstructionType {
    Set,
    Sub,
    Mul,
    Jnz,
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    value_a: (Option<char>, i32),
    value_b: (Option<char>, i32),
}

impl Instruction {
    fn new(input: &str) -> Option<Instruction> {
        let mut splits = input.split_whitespace();
        let instruction_type = match splits.next().unwrap() {
            "set" => Some(InstructionType::Set),
            "sub" => Some(InstructionType::Sub),
            "mul" => Some(InstructionType::Mul),
            "jnz" => Some(InstructionType::Jnz),
            _ => None,
        };
        match instruction_type {
            Some(t) => {
                let tmp_value = splits.next().unwrap();
                let value_a = match tmp_value.parse::<i32>() {
                    Ok(x) => (None, x),
                    Err(_) => (Some(tmp_value.chars().next().unwrap()), 0),
                };
                let tmp_value = splits.next().unwrap();
                let value_b = match tmp_value.parse::<i32>() {
                    Ok(x) => (None, x),
                    Err(_) => (Some(tmp_value.chars().next().unwrap()), 0),
                };
                Some(Instruction {
                    instruction_type: t,
                    value_a,
                    value_b,
                })
            }
            None => None,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::new(line).unwrap())
        .collect();
    println!("Silver: {}", silver(&instructions));
    println!("Gold: {}", gold());
}

fn silver(instructions: &Vec<Instruction>) -> usize {
    let mut registers: HashMap<char, i128> = HashMap::new();
    instructions.iter().for_each(|instruction| {
        if let Some(c) = instruction.value_a.0 {
            registers.insert(c, 0);
        }
    });
    let mut index: i32 = 0;
    let mut result = 0;
    loop {
        if index >= instructions.len() as i32 {
            break;
        }
        match instructions[index as usize].instruction_type {
            InstructionType::Set => {
                registers.insert(
                    instructions[index as usize].value_a.0.unwrap(),
                    get_value(&instructions[index as usize].value_b, &registers),
                );
            }
            InstructionType::Sub => {
                *registers
                    .get_mut(&instructions[index as usize].value_a.0.unwrap())
                    .unwrap() -= get_value(&instructions[index as usize].value_b, &registers);
            }
            InstructionType::Mul => {
                *registers
                    .get_mut(&instructions[index as usize].value_a.0.unwrap())
                    .unwrap() *= get_value(&instructions[index as usize].value_b, &registers);
                result += 1;
            }
            InstructionType::Jnz => {
                if get_value(&instructions[index as usize].value_a, &registers) != 0 {
                    index +=
                        (get_value(&instructions[index as usize].value_b, &registers) - 1) as i32;
                }
            }
        }
        index += 1;
    }
    result
}

fn gold() -> i128 {
    let mut h = 0;
    let mut b = 79;
    b *= 100;
    b += 100000;
    for _ in 0..1001 {
        'inner: for d in 2..b {
            if (b / d) * d == b {
                h += 1;
                break 'inner;
            }
        }
        b += 17;
    }
    h
}

fn get_value(value: &(Option<char>, i32), registers: &HashMap<char, i128>) -> i128 {
    match value {
        (None, x) => *x as i128,
        (Some(x), _) => *registers.get(&x).unwrap(),
    }
}
