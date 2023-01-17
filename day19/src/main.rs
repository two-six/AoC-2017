use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let board = parse(&input);
    let result = solve(&board);
    println!("Silver: {}", result.0);
    println!("Gold: {}", result.1);
}

fn solve(board: &HashMap<(usize, usize), char>) -> (String, usize) {
    let mut position = board.keys().find(|&&x| x.1 == 0).unwrap().clone();
    let mut direction: u8 = 2;
    let mut silver: Vec<char> = Vec::new();
    let mut gold: usize = 1;
    loop {
        match direction {
            0 => {
                if position.1 == 0 {
                    break;
                }
                position.1 -= 1;
            }
            1 => {
                position.0 += 1;
            }
            2 => {
                position.1 += 1;
            }
            3 => {
                if position.0 == 0 {
                    break;
                }
                position.0 -= 1;
            }
            _ => (),
        }
        if !board.contains_key(&position) {
            break;
        }
        match board.get(&position).unwrap() {
            '|' | '-' => (),
            '+' => {
                if board.contains_key(&(position.0, position.1 - 1)) && direction != 2 {
                    direction = 0;
                } else if board.contains_key(&(position.0 + 1, position.1)) && direction != 3 {
                    direction = 1;
                } else if board.contains_key(&(position.0, position.1 + 1)) && direction != 0 {
                    direction = 2;
                } else if board.contains_key(&(position.0 - 1, position.1)) && direction != 1 {
                    direction = 3;
                }
            }
            c => {
                silver.push(*c);
            }
        }
        gold += 1;
    }
    (silver.iter().collect(), gold)
}

fn parse(input: &str) -> HashMap<(usize, usize), char> {
    let mut result: HashMap<(usize, usize), char> = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| match c {
            ' ' => (),
            _ => {
                result.insert((j, i), c);
            }
        });
    });
    result
}
