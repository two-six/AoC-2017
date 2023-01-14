use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    let instructions: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    println!("Silver: {}", silver(&instructions).unwrap());
    println!("Gold: {}", gold(&instructions).unwrap());
}

fn silver(instructions: &Vec<i32>) -> Option<i32> {
    let mut position: i32 = 0;
    let mut instructions = instructions.clone();
    for i in 1.. {
        instructions[position as usize] += 1;
        position += instructions[position as usize] - 1;
        if position < 0 || position >= instructions.len() as i32 {
            return Some(i);
        }
    }
    None
}

fn gold(instructions: &Vec<i32>) -> Option<i32> {
    let mut position: i32 = 0;
    let mut instructions = instructions.clone();
    for i in 1.. {
        let current_position: usize = position as usize;
        position += instructions[current_position];
        if position < 0 || position >= instructions.len() as i32 {
            return Some(i);
        }
        instructions[current_position] += if instructions[current_position] >= 3 {
            -1
        } else {
            1
        };
    }
    None
}
