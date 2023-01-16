const STEPS: usize = 301;
// const STEPS: usize = 3;
const REPEATS: usize = 2017;

fn main() {
    let result = solve();
    println!("Silver: {}", result.0);
    println!("Gold: {}", result.1);
}

fn solve() -> (usize, usize) {
    let mut buffer: Vec<usize> = vec![0];
    let mut current_position = 0;
    for _ in 0..REPEATS {
        current_position = next_position(&current_position, &STEPS, &buffer.len());
        buffer.insert(current_position + 1, buffer.len());
    }
    let silver = buffer.get(current_position + 2).unwrap().clone();
    let mut gold = *buffer.get(1).unwrap();
    for i in REPEATS..50_000_000 {
        current_position = next_position(&current_position, &STEPS, &i);
        if current_position == 0 {
            gold = i;
        }
    }
    (silver, gold)
}

fn next_position(current_position: &usize, steps: &usize, buffer_size: &usize) -> usize {
    (steps + current_position + 1) % buffer_size
}
