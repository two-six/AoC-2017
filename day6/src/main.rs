use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    let banks: Vec<u8> = input
        .split_whitespace()
        .map(|bank| bank.parse::<u8>().unwrap())
        .collect();
    let result = solve(&banks).unwrap();
    println!("Silver: {}", result.0);
    println!("Gold: {}", result.1);
}

fn solve(banks: &Vec<u8>) -> Option<(i32, i32)> {
    let mut banks = banks.clone();
    let mut configurations: Vec<Vec<u8>> = vec![banks.clone()];
    for i in 1.. {
        redistribute(get_max_index(&banks).unwrap(), &mut banks);
        if configurations.contains(&banks) {
            let prev_index = configurations
                .iter()
                .enumerate()
                .find(|(_, value)| **value == banks)
                .map(|(index, _)| index)
                .unwrap() as i32;
            return Some((i, i - prev_index));
        }
        configurations.push(banks.clone());
    }
    None
}

fn get_max_index(list: &Vec<u8>) -> Option<usize> {
    if list.is_empty() {
        return None;
    }
    let mut m = list[0];
    let mut result = 0;
    for i in 1..list.len() {
        if list[i] > m {
            result = i;
            m = list[i];
        }
    }
    Some(result)
}

fn redistribute(from: usize, banks: &mut Vec<u8>) {
    let blocks = banks[from];
    banks[from] = 0;
    let mut position = from;
    for _ in 0..blocks {
        position += 1;
        if position >= banks.len() {
            position = 0;
        }
        banks[position] += 1;
    }
}
