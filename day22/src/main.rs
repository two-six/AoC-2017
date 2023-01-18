use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut board: HashSet<(i32, i32)> = HashSet::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                board.insert((j as i32, i as i32));
            }
        });
    });
    let n = input.lines().next().unwrap().len() as i32;
    println!("Silver: {}", silver(&board, n));
    println!("Gold: {}", gold(&board, n));
}

fn silver(board: &HashSet<(i32, i32)>, n: i32) -> usize {
    let mut result: usize = 0;
    let mut board = board.clone();
    let mut virus: (i32, i32) = (n / 2, n / 2);
    let mut direction: i8 = 0;
    for _ in 0..10_000 {
        if board.contains(&virus) {
            direction += 1;
            if direction == 4 {
                direction = 0;
            }
            board.remove(&virus);
        } else {
            direction -= 1;
            if direction == -1 {
                direction = 3;
            }
            board.insert(virus);
            result += 1;
        }
        match direction {
            0 => virus.1 -= 1,
            1 => virus.0 += 1,
            2 => virus.1 += 1,
            3 => virus.0 -= 1,
            _ => (),
        }
    }
    result
}

fn gold(board: &HashSet<(i32, i32)>, n: i32) -> usize {
    let mut result: usize = 0;
    let mut board_map: HashMap<(i32, i32), char> = board.iter().map(|node| (*node, 'I')).collect();
    let mut virus: (i32, i32) = (n / 2, n / 2);
    let mut direction: i8 = 0;
    for _ in 0..10_000_000 {
        if let Some(v) = board_map.get(&virus) {
            match v {
                'W' => {
                    board_map.insert(virus, 'I');
                    result += 1;
                }
                'I' => {
                    direction += 1;
                    if direction == 4 {
                        direction = 0;
                    }
                    board_map.insert(virus, 'F');
                }
                'F' => {
                    direction = match direction {
                        0 => 2,
                        1 => 3,
                        2 => 0,
                        3 => 1,
                        _ => 0,
                    };
                    board_map.remove(&virus);
                }
                _ => (),
            }
        } else {
            direction -= 1;
            if direction == -1 {
                direction = 3;
            }
            board_map.insert(virus, 'W');
        }
        match direction {
            0 => virus.1 -= 1,
            1 => virus.0 += 1,
            2 => virus.1 += 1,
            3 => virus.0 -= 1,
            _ => (),
        }
    }
    result
}
