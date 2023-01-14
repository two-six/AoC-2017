use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    let programs: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split(" <-> ")
                .last()
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    println!("Silver: {}", get_group(&programs, 0).len());
    println!("Gold: {}", gold(&programs));
}

fn gold(programs: &Vec<Vec<usize>>) -> usize {
    let mut result: Vec<Vec<usize>> = Vec::new();
    let mut excluded: Vec<usize> = Vec::new();
    for i in 0..programs.len() {
        if excluded.contains(&i) {
            continue;
        }
        let new_group = get_group(programs, i);
        excluded.extend(&new_group);
        result.push(new_group);
    }
    result.len()
}

fn get_group(programs: &Vec<Vec<usize>>, id: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    expand_communication(id, &mut result, programs);
    result
}

fn expand_communication(id: usize, result: &mut Vec<usize>, programs: &Vec<Vec<usize>>) {
    if !result.contains(&id) {
        result.push(id);
    }
    programs[id].iter().for_each(|program| {
        if !result.contains(program) {
            expand_communication(*program, result, programs);
        }
    });
}
