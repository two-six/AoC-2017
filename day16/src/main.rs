use std::fs;

#[derive(Debug)]
enum DanceMoveType {
    Spin,
    Exchange,
    Partner,
}

#[derive(Debug)]
struct DanceMove {
    dance_type: DanceMoveType,
    values: (usize, usize),
}

impl DanceMove {
    fn new(input: &str) -> Option<DanceMove> {
        match input.chars().nth(0) {
            Some('s') => Some(DanceMove {
                dance_type: DanceMoveType::Spin,
                values: (input[1..].to_owned().parse::<usize>().unwrap(), 0),
            }),
            Some('x') => Some(DanceMove {
                dance_type: DanceMoveType::Exchange,
                values: {
                    let values = Self::parse_move(&input[1..]).unwrap();
                    (
                        values.0.parse::<usize>().unwrap(),
                        values.1.parse::<usize>().unwrap(),
                    )
                },
            }),
            Some('p') => Some(DanceMove {
                dance_type: DanceMoveType::Partner,
                values: {
                    let values = Self::parse_move(&input[1..]).unwrap();
                    (
                        values.0.bytes().next().unwrap() as usize - 97,
                        values.1.bytes().next().unwrap() as usize - 97,
                    )
                },
            }),
            _ => None,
        }
    }

    fn execute(&self, programs: &mut [usize; 16]) {
        match self.dance_type {
            DanceMoveType::Spin => {
                let split_value = self.values.0;
                let tmp_programs = programs.clone();
                for i in 0..split_value {
                    programs[i] = tmp_programs[16 - (split_value - i)];
                }
                for i in 0..16 - split_value {
                    programs[split_value + i] = tmp_programs[i];
                }
            }
            DanceMoveType::Exchange => {
                programs.swap(self.values.0, self.values.1);
            }
            DanceMoveType::Partner => {
                let values = (
                    programs.iter().position(|c| c == &self.values.0).unwrap(),
                    programs.iter().position(|c| c == &self.values.1).unwrap(),
                );
                programs.swap(values.0, values.1);
            }
        }
    }

    fn parse_move(input: &str) -> Option<(String, String)> {
        let values = (input.split('/').next(), input.split('/').last());
        match values {
            (Some(a), Some(b)) => Some((a.to_owned(), b.to_owned())),
            _ => None,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let dance_moves: Vec<DanceMove> = input
        .split(",")
        .map(|x| DanceMove::new(x).unwrap())
        .collect();

    let result = solve(&dance_moves);
    println!("Silver: {}", result.0);
    println!("Gold: {}", result.1);
}

fn solve(dance_moves: &Vec<DanceMove>) -> (String, String) {
    let mut programs: [usize; 16] = {
        let mut tmp_programs: [usize; 16] = [0; 16];
        for i in 1..16 {
            tmp_programs[i] = i;
        }
        tmp_programs
    };
    let init_programs = programs.clone();
    dance_moves.iter().for_each(|dance_move| {
        dance_move.execute(&mut programs);
    });
    let silver = convert_to_string(&programs);
    let mut cycle = 1;
    while programs != init_programs {
        dance_moves.iter().for_each(|dance_move| {
            dance_move.execute(&mut programs);
        });
        cycle += 1;
    }
    for _ in 0..1_000_000_000 % cycle {
        dance_moves.iter().for_each(|dance_move| {
            dance_move.execute(&mut programs);
        });
    }
    (silver, convert_to_string(&programs))
}

fn convert_to_string(input: &[usize; 16]) -> String {
    let mut result: String = String::from("");
    input.iter().for_each(|c| {
        result.push(char::from_u32((c + 97) as u32).unwrap());
    });
    result
}
