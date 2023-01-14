use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    let spreadsheet: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    println!("Silver: {}", silver(&spreadsheet));
    println!("Gold: {}", gold(&spreadsheet));
}

fn silver(spreadsheet: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    spreadsheet.iter().for_each(|line| {
        result += max_minus_min(line);
    });
    result
}

fn gold(spreadsheet: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    spreadsheet.iter().for_each(|line| {
        result += divisible_divided(line).unwrap();
    });
    result
}

fn max_minus_min(row: &Vec<i32>) -> i32 {
    row.iter().max().unwrap() - row.iter().min().unwrap()
}

fn divisible_divided(row: &Vec<i32>) -> Option<i32> {
    let mut row = row.clone();
    row.sort_by(|a, b| b.cmp(a));
    for i in 0..row.len() {
        for j in i + 1..row.len() {
            if row[i] % row[j] == 0 {
                return Some(row[i] / row[j]);
            }
        }
    }
    None
}
