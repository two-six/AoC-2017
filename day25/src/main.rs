use std::collections::HashMap;

#[derive(Debug)]
struct State {
    if_value: (u8, u8),
    if_index: (i8, i8),
    if_state: (char, char),
}

impl State {
    fn new(if_value: (u8, u8), if_index: (i8, i8), if_state: (char, char)) -> State {
        State {
            if_value,
            if_index,
            if_state,
        }
    }
}

fn main() {
    // let mut test_input: HashMap<char, State> = HashMap::new();
    // test_input.insert('A', State::new((1, 0), (1, -1), ('B', 'B')));
    // test_input.insert('B', State::new((1, 1), (-1, 1), ('A', 'A')));
    // println!("Silver: {}", run_steps(&test_input, 'A', 6).len());

    let mut input: HashMap<char, State> = HashMap::new();
    input.insert('A', State::new((1, 0), (1, -1), ('B', 'E')));
    input.insert('B', State::new((1, 0), (-1, 1), ('C', 'A')));
    input.insert('C', State::new((1, 0), (-1, 1), ('D', 'C')));
    input.insert('D', State::new((1, 0), (-1, -1), ('E', 'F')));
    input.insert('E', State::new((1, 1), (-1, -1), ('A', 'C')));
    input.insert('F', State::new((1, 1), (-1, 1), ('E', 'A')));
    println!("Silver: {}", run_steps(&input, 'A', 12_208_951).len());
    println!("Gold: {}", ";)")
}

fn run_steps(input: &HashMap<char, State>, begin: char, steps: usize) -> Vec<i64> {
    let mut index: i64 = 0;
    let mut current_state = begin;
    let mut tape: Vec<i64> = Vec::new();
    for _ in 0..steps {
        let tmp_state = input.get(&current_state).unwrap();
        if !tape.contains(&index) {
            if tmp_state.if_value.0 == 1 {
                tape.push(index);
            }
            index += tmp_state.if_index.0 as i64;
            current_state = tmp_state.if_state.0;
        } else {
            if tmp_state.if_value.1 == 0 {
                tape.remove(tape.iter().rposition(|ind| ind == &index).unwrap());
            }
            index += tmp_state.if_index.1 as i64;
            current_state = tmp_state.if_state.1;
        }
    }
    tape
}
