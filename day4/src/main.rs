use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    let passwords: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    println!("Silver: {}", silver(&passwords));
    println!("Gold: {}", gold(&passwords));
}

fn silver(passwords: &Vec<Vec<&str>>) -> i32 {
    let mut result = 0;
    passwords.iter().for_each(|password| {
        let password_set: HashSet<&str> = password.iter().map(|word| *word).collect();
        if password_set.len() == password.len() {
            result += 1;
        }
    });
    result
}

fn gold(passwords: &Vec<Vec<&str>>) -> i32 {
    let mut result = 0;
    passwords.iter().for_each(|password| {
        let password_sorted: Vec<String> = password
            .iter()
            .map(|word| {
                let mut chars: Vec<char> = word.chars().collect();
                chars.sort();
                chars.into_iter().collect()
            })
            .collect();
        let password_sorted_set: HashSet<&str> =
            password_sorted.iter().map(|word| word.as_str()).collect();
        if password_sorted.len() == password_sorted_set.len() {
            result += 1;
        }
    });
    result
}
