use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Program {
    weight: usize,
    disc: Option<Vec<String>>,
}

impl Program {
    fn new(weight: usize, disc: Option<Vec<String>>) -> Program {
        Program { weight, disc }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    println!("Silver: {}", root(&input).unwrap());
    println!("Gold: {}", gold(&input).unwrap());
}

fn root(input: &str) -> Option<String> {
    let regex_right = Regex::new(r".*? -> (.*)").unwrap();
    let regex_left = Regex::new(r"(\w+) \(\d+\)").unwrap();
    let mut elements: HashSet<String> = HashSet::new();
    let mut children: HashSet<String> = HashSet::new();
    for cap in regex_right.captures_iter(input) {
        cap[1].replace(" ", "").split(",").for_each(|el| {
            children.insert(String::from(el));
        });
    }
    for cap in regex_left.captures_iter(input) {
        elements.insert(String::from(&cap[1]));
    }
    elements.difference(&children).last().cloned()
}

fn gold(input: &str) -> Option<usize> {
    let mut programs: HashMap<String, Program> = HashMap::new();
    let regex_full = Regex::new(r"(\w+) \(\d+\) -> (.*)").unwrap();
    let regex_left = Regex::new(r"(\w+) \((\d+)\)").unwrap();
    for cap in regex_full.captures_iter(input) {
        programs.insert(
            String::from(&cap[1]),
            Program::new(
                0,
                Some(
                    cap[2]
                        .replace(" ", "")
                        .split(",")
                        .map(|el| String::from(el))
                        .collect(),
                ),
            ),
        );
    }
    for cap in regex_left.captures_iter(input) {
        programs.insert(
            String::from(&cap[1]),
            Program::new(
                cap[2].parse::<usize>().unwrap(),
                if programs.contains_key(&cap[1]) {
                    programs.get(&cap[1]).unwrap().disc.clone()
                } else {
                    None
                },
            ),
        );
    }
    find_incorrect_program_weight(&root(input).unwrap(), &programs)
}

fn calculate_program_weight(program: &Program, tree: &HashMap<String, Program>) -> usize {
    program.weight
        + if program.disc.is_some() {
            program.disc.as_ref().unwrap().iter().fold(0, |acc, next| {
                acc + calculate_program_weight(tree.get(next).unwrap(), tree)
            })
        } else {
            0
        }
}

fn find_incorrect_program_weight(root: &str, tree: &HashMap<String, Program>) -> Option<usize> {
    fn recursive_search(
        new_root: &str,
        tree: &HashMap<String, Program>,
        last: i32,
        difference: i32,
    ) -> Option<usize> {
        let program = tree.get(new_root).unwrap();
        let program_disc = program.disc.as_ref();
        match program_disc {
            Some(disc) => {
                let mut weights: HashMap<usize, usize> = HashMap::new();
                for p in disc {
                    let weight = calculate_program_weight(tree.get(p).unwrap(), tree);
                    if weights.contains_key(&weight) && weights.len() > 1 {
                        let incorrect_program_weights = weights
                            .iter()
                            .filter(|(k, _)| **k != weight)
                            .next()
                            .unwrap();
                        let incorrect_program = disc
                            .iter()
                            .filter(|p| {
                                tree.get(*p).unwrap().weight == incorrect_program_weights.1.clone()
                            })
                            .next()
                            .unwrap();
                        return recursive_search(
                            incorrect_program,
                            tree,
                            incorrect_program_weights.1.clone() as i32,
                            weight as i32 - incorrect_program_weights.0.clone() as i32,
                        );
                    }
                    weights.insert(weight, tree.get(p).unwrap().weight);
                }
                Some((last + difference) as usize)
            }
            None => None,
        }
    }
    let root_disc = tree.get(root).unwrap().disc.as_ref();
    match root_disc {
        Some(d) => {
            let mut weights: HashMap<usize, usize> = HashMap::new();
            for program in d {
                let weight = calculate_program_weight(tree.get(program).unwrap(), tree);
                if weights.contains_key(&weight) && weights.len() > 1 {
                    let incorrect_program_weights = weights
                        .iter()
                        .filter(|(k, _)| **k != weight)
                        .next()
                        .unwrap();
                    let incorrect_program = d
                        .iter()
                        .filter(|p| {
                            tree.get(*p).unwrap().weight == incorrect_program_weights.1.clone()
                        })
                        .next()
                        .unwrap();
                    return recursive_search(
                        incorrect_program,
                        tree,
                        incorrect_program_weights.1.clone() as i32,
                        weight as i32 - incorrect_program_weights.0.clone() as i32,
                    );
                }
                weights.insert(weight, tree.get(program).unwrap().weight);
            }
            return None;
        }
        None => None,
    }
}
