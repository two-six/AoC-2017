use std::collections::HashMap;

const NEIGHBOURS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn main() {
    let input = 368078;
    println!("Silver: {}", silver(input));
    println!("Gold: {}", gold(input));
}

fn gold(n: i32) -> i32 {
    let mut board: HashMap<(i32, i32), i32> = HashMap::new();
    board.insert((0, 0), 1);
    board.insert((1, 0), 1);
    let mut direction: u8 = 0;
    let mut current = (1, 0);
    loop {
        let next = (
            current.0
                + if direction == 1 || direction == 3 {
                    if direction == 1 {
                        1
                    } else {
                        -1
                    }
                } else {
                    0
                },
            current.1
                + if direction == 0 || direction == 2 {
                    if direction == 0 {
                        -1
                    } else {
                        1
                    }
                } else {
                    0
                },
        );
        let next_value = add_neighbours(&board, &next);
        if next_value > n {
            return next_value;
        }
        current = next;
        board.insert(current, next_value);
        if current.0.abs() == current.1.abs() {
            if current.0.is_positive() && current.1.is_negative() {
                direction = 3;
            } else if current.0.is_negative() && current.1.is_positive() {
                direction = 1;
            } else if current.0.is_negative() && current.1.is_negative() {
                direction = 2;
            } else {
                current = (current.0 + 1, current.1);
                let next_value = add_neighbours(&board, &current);
                if next_value > n {
                    return next_value;
                }
                board.insert(current, next_value);
                direction = 0;
            }
        }
    }
}

fn silver(n: i32) -> i32 {
    let ring = get_ring(n).unwrap();
    let mut max_in_ring = 4 * (ring * ring) + 4 * ring + 1;
    let mut i = ring;
    let mut up = false;
    let additional_distance: i32;
    loop {
        if max_in_ring == n {
            additional_distance = i;
            break;
        }
        max_in_ring -= 1;
        if up {
            i += 1;
        } else {
            i -= 1;
        }
        if i == 0 {
            up = true;
        }
        if i == ring {
            up = false;
        }
    }
    ring + additional_distance
}

fn add_neighbours(board: &HashMap<(i32, i32), i32>, current: &(i32, i32)) -> i32 {
    let mut result = 0;
    NEIGHBOURS.iter().for_each(|neighbour| {
        if board.contains_key(&(current.0 + neighbour.0, current.1 + neighbour.1)) {
            result += board
                .get(&(neighbour.0 + current.0, neighbour.1 + current.1))
                .unwrap();
        }
    });
    result
}

fn get_ring(n: i32) -> Option<i32> {
    if n == 1 {
        return Some(0);
    }
    if n <= 0 {
        return None;
    }
    for i in 1.. {
        if 4 * (i * i) + 4 * i + 1 >= n {
            return Some(i);
        }
    }
    None
}
