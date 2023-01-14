use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    let moves: Vec<&str> = input.split(",").collect();
    let result = solve(&moves);
    println!("Silver: {}", result.0);
    println!("Gold: {}", result.1);
}

fn solve(moves: &Vec<&str>) -> (usize, usize) {
    let mut position = (0, 0);
    let mut gold: usize = 0;
    moves.iter().for_each(|m| {
        next_move(m, &mut position);
        let required = moves_required(&position);
        if required > gold {
            gold = required;
        }
    });
    (moves_required(&position), gold)
}

fn moves_required(position: &(i32, i32)) -> usize {
    (position.0.abs() / 2 + position.1.abs() / 2 + position.0.abs() % 2) as usize
}

fn next_move(m: &str, position: &mut (i32, i32)) {
    match m {
        "n" => position.1 -= 2,
        "s" => position.1 += 2,
        "ne" => {
            position.1 -= 1;
            position.0 += 1;
        }
        "nw" => {
            position.1 -= 1;
            position.0 -= 1;
        }
        "sw" => {
            position.1 += 1;
            position.0 -= 1;
        }
        "se" => {
            position.1 += 1;
            position.0 += 1;
        }
        _ => (),
    };
}
