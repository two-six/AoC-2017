use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
enum InstructionType {
    Snd,
    Set,
    Add,
    Mul,
    Mod,
    Rcv,
    Jgz,
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
            "snd" => Some(InstructionType::Snd),
            "set" => Some(InstructionType::Set),
            "add" => Some(InstructionType::Add),
            "mul" => Some(InstructionType::Mul),
            "mod" => Some(InstructionType::Mod),
            "rcv" => Some(InstructionType::Rcv),
            "jgz" => Some(InstructionType::Jgz),
            _ => None,
        };
        match instruction_type {
            Some(t) => {
                let tmp_value = splits.next().unwrap();
                let value_a = match tmp_value.parse::<i32>() {
                    Ok(x) => (None, x),
                    Err(_) => (Some(tmp_value.chars().next().unwrap()), 0),
                };
                let mut value_b: (Option<char>, i32) = (None, 0);
                if t != InstructionType::Snd && t != InstructionType::Rcv {
                    let tmp_value = splits.next().unwrap();
                    value_b = match tmp_value.parse::<i32>() {
                        Ok(x) => (None, x),
                        Err(_) => (Some(tmp_value.chars().next().unwrap()), 0),
                    };
                }
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
    let input = fs::read_to_string("input.txt").unwrap();
    let instructions = parse(&input);
    println!("Silver: {}", silver(&instructions));
    println!("Gold: {}", gold(&instructions));
}

fn silver(instructions: &Vec<Instruction>) -> i128 {
    let mut index: i32 = 0;
    let mut last_sound_frequency = 0;
    let mut registers: HashMap<char, i128> = HashMap::new();
    instructions
        .iter()
        .for_each(|instruction| match instruction.value_a {
            (Some(x), _) => {
                registers.insert(x, 0);
            }
            _ => (),
        });
    loop {
        match instructions[index as usize].instruction_type {
            InstructionType::Snd => {
                last_sound_frequency = get_value(&instructions[index as usize].value_a, &registers);
            }
            InstructionType::Set => {
                registers.insert(
                    instructions[index as usize].value_a.0.unwrap(),
                    get_value(&instructions[index as usize].value_b, &registers),
                );
            }
            InstructionType::Add => {
                let tmp_value = get_value(&instructions[index as usize].value_b, &registers);
                *registers
                    .get_mut(&instructions[index as usize].value_a.0.unwrap())
                    .unwrap() += tmp_value;
            }
            InstructionType::Mul => {
                let tmp_value = get_value(&instructions[index as usize].value_b, &registers);
                *registers
                    .get_mut(&instructions[index as usize].value_a.0.unwrap())
                    .unwrap() *= tmp_value;
            }
            InstructionType::Mod => {
                let tmp_value = get_value(&instructions[index as usize].value_b, &registers);
                *registers
                    .get_mut(&instructions[index as usize].value_a.0.unwrap())
                    .unwrap() %= tmp_value;
            }
            InstructionType::Rcv => {
                if registers
                    .get(&instructions[index as usize].value_a.0.unwrap())
                    .unwrap()
                    != &0
                {
                    break;
                }
            }
            InstructionType::Jgz => {
                if get_value(&instructions[index as usize].value_a, &registers) > 0 {
                    index +=
                        get_value(&instructions[index as usize].value_b, &registers) as i32 - 1;
                }
            }
        }
        index += 1;
    }
    last_sound_frequency
}

fn gold(instructions: &Vec<Instruction>) -> usize {
    let mut index: (i32, i32) = (0, 0);
    let mut registers: (HashMap<char, i128>, HashMap<char, i128>) =
        (HashMap::new(), HashMap::new());
    instructions
        .iter()
        .for_each(|instruction| match instruction.value_a {
            (Some(x), _) => {
                registers.0.insert(x, 0);
                registers.1.insert(x, 0);
            }
            _ => (),
        });
    registers.1.insert('p', 1);
    let mut sent_values: (Vec<i128>, Vec<i128>) = (Vec::new(), Vec::new());
    let mut result = 0;
    loop {
        let program0 = run_gold_instructions(
            &mut registers.0,
            &mut index.0,
            instructions,
            &mut sent_values.0,
            &mut sent_values.1,
        );
        let program1 = run_gold_instructions(
            &mut registers.1,
            &mut index.1,
            instructions,
            &mut sent_values.1,
            &mut sent_values.0,
        );
        result += program1.0;
        if (sent_values.0.len() == 0 && sent_values.1.len() == 0) || (program0.1 && program1.1) {
            break;
        }
    }
    result
}

fn run_gold_instructions(
    registers: &mut HashMap<char, i128>,
    index: &mut i32,
    instructions: &Vec<Instruction>,
    rcv_values: &mut Vec<i128>,
    sent_values: &mut Vec<i128>,
) -> (usize, bool) {
    let mut result = 0;
    loop {
        if *index >= instructions.len() as i32 {
            return (result, true);
        }
        match instructions[*index as usize].instruction_type {
            InstructionType::Snd => {
                sent_values.push(get_value(
                    &instructions[*index as usize].value_a,
                    &registers,
                ));
                result += 1;
            }
            InstructionType::Set => {
                registers.insert(
                    instructions[*index as usize].value_a.0.unwrap(),
                    get_value(&instructions[*index as usize].value_b, &registers),
                );
            }
            InstructionType::Add => {
                let tmp_value = get_value(&instructions[*index as usize].value_b, &registers);
                *registers
                    .get_mut(&instructions[*index as usize].value_a.0.unwrap())
                    .unwrap() += tmp_value;
            }
            InstructionType::Mul => {
                let tmp_value = get_value(&instructions[*index as usize].value_b, &registers);
                *registers
                    .get_mut(&instructions[*index as usize].value_a.0.unwrap())
                    .unwrap() *= tmp_value;
            }
            InstructionType::Mod => {
                let tmp_value = get_value(&instructions[*index as usize].value_b, &registers);
                *registers
                    .get_mut(&instructions[*index as usize].value_a.0.unwrap())
                    .unwrap() %= tmp_value;
            }
            InstructionType::Rcv => match rcv_values.first() {
                Some(v) => {
                    registers.insert(instructions[*index as usize].value_a.0.unwrap(), *v);
                    rcv_values.remove(0);
                }
                None => {
                    break;
                }
            },
            InstructionType::Jgz => {
                if get_value(&instructions[*index as usize].value_a, &registers) > 0 {
                    *index +=
                        get_value(&instructions[*index as usize].value_b, &registers) as i32 - 1;
                }
            }
        }
        *index += 1;
    }
    (result, false)
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::new(&line).unwrap())
        .collect()
}

fn get_value(value: &(Option<char>, i32), registers: &HashMap<char, i128>) -> i128 {
    match value {
        (Some(x), _) => *registers.get(&x).unwrap(),
        (None, x) => *x as i128,
    }
}
