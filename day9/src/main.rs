use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    println!("Silver: {}", silver(&input));
    println!("Gold: {}", gold(&input));
}

fn silver(input: &str) -> i32 {
    let exclamation_regex = Regex::new(r"!.").unwrap();
    let garbage_regex = Regex::new(r"<.*?>").unwrap();
    let parsed_input = exclamation_regex.replace_all(&input, "");
    let parsed_input = garbage_regex.replace_all(&parsed_input, "");
    let mut result = 0;
    let mut depth = 0;
    parsed_input.chars().for_each(|char| {
        match char {
            '{' => {
                depth += 1;
                result += depth;
            }
            '}' => depth -= 1,
            _ => (),
        };
    });
    result
}

fn gold(input: &str) -> usize {
    let exclamation_regex = Regex::new(r"!.").unwrap();
    let garbage_regex = Regex::new(r"<(.*?)>").unwrap();
    let parsed_input = exclamation_regex.replace_all(&input, "");
    let mut result = 0;
    for cap in garbage_regex.captures_iter(&parsed_input) {
        result += &cap[1].len();
    }
    result
}
