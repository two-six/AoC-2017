use std::collections::HashMap;

const INITIAL_SQUARE: &str = ".#./..#/###";

#[derive(Debug, Clone, PartialEq)]
struct Square {
    square: Vec<Vec<bool>>,
}

impl Square {
    fn new() -> Square {
        Square { square: Vec::new() }
    }

    fn count_turned_on(&self) -> usize {
        self.square
            .iter()
            .map(|row| {
                row.iter()
                    .fold(0, |acc, next| if *next { acc + 1 } else { acc })
            })
            .fold(0, |acc, row| acc + row)
    }

    fn rotate(&mut self) {
        let mut tmp_square = vec![Vec::new(); self.square.len()];
        for i in 1..=self.square.len() {
            for j in 0..self.square.len() {
                tmp_square[j].push(self.square[self.square.len() - i][j]);
            }
        }
        self.square = tmp_square;
    }

    fn flip(&mut self) {
        let mut tmp_square = vec![vec![false; self.square.len()]; self.square.len()];
        for i in 0..self.square.len() {
            for j in 0..self.square.len() {
                tmp_square[i][j] = self.square[i][self.square.len() - 1 - j];
            }
        }
        self.square = tmp_square;
    }

    fn from(pattern: &str) -> Square {
        let n = pattern
            .chars()
            .fold(0, |acc, c| if c == '/' { acc + 1 } else { acc })
            + 1;
        let mut result = vec![Vec::new(); n];
        pattern
            .split('/')
            .map(|line| line.chars())
            .enumerate()
            .for_each(|(i, line)| {
                line.for_each(|c| {
                    result[i].push(if c == '#' { true } else { false });
                });
            });
        Square { square: result }
    }

    fn divide(&self) -> Board {
        let mut result = Board::new();
        if let Some(s) = self.square.get(0) {
            let n = s.len();
            if n % 2 == 0 {
                let n = n / 2;
                result.squares = vec![
                    vec![
                        Square {
                            square: vec![vec![false, false]; 2]
                        };
                        n
                    ];
                    n
                ];
                for i in 0..n {
                    for j in 0..n {
                        for y in 0..2 {
                            for x in 0..2 {
                                // println!("2 -> i: {}, j: {}, y: {}, x: {}", i, j, y, x);
                                result.squares[i][j].square[y][x] =
                                    self.square[i * 2 + y][j * 2 + x];
                            }
                        }
                    }
                }
            } else {
                let n = n / 3;
                result.squares = vec![
                    vec![
                        Square {
                            square: vec![vec![false, false, false]; 3]
                        };
                        n
                    ];
                    n
                ];
                for i in 0..n {
                    for j in 0..n {
                        for y in 0..3 {
                            for x in 0..3 {
                                // println!("3 -> i: {}, j: {}, y: {}, x: {}", i, j, y, x);
                                result.squares[i][j].square[y][x] =
                                    self.square[i * 3 + y][j * 3 + x];
                            }
                        }
                    }
                }
            }
        }
        result
    }

    fn turn_to_pattern(&self) -> String {
        let mut result: String = String::from("");
        self.square.iter().for_each(|row| {
            row.iter().for_each(|c| {
                if *c {
                    result += "#";
                } else {
                    result += ".";
                }
            });
            result += "/";
        });
        result.remove(result.len() - 1);
        result
    }
}

#[derive(Debug, Clone)]
struct Board {
    squares: Vec<Vec<Square>>,
}

impl Board {
    fn new() -> Board {
        Board {
            squares: Vec::new(),
        }
    }

    fn flatten(&self) -> Square {
        let n = self.squares.len();
        let square_len = self.squares.get(0).unwrap().get(0).unwrap().square.len();
        let mut result: Square = Square {
            square: vec![vec![false; square_len * n]; square_len * n],
        };
        self.squares.iter().enumerate().for_each(|(i, column)| {
            column.iter().enumerate().for_each(|(j, row)| {
                row.square.iter().enumerate().for_each(|(y, square_row)| {
                    square_row.iter().enumerate().for_each(|(x, square)| {
                        result.square[i * square_len + y][j * square_len + x] = *square;
                    });
                });
            });
        });
        result
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let patterns = create_rulebook(&input);
    let result = solve(&patterns);
    println!("Silver: {}", result.0);
    println!("Gold: {}", result.1);
}

fn create_rulebook(input: &str) -> HashMap<String, String> {
    let mut patterns: HashMap<String, String> = input
        .lines()
        .map(|line| {
            let result = line.split_once(" => ").unwrap();
            (result.0.to_owned(), result.1.to_owned())
        })
        .collect();
    let keys: Vec<String> = patterns.keys().map(|k| k.clone()).collect();
    for key in keys {
        let pattern = patterns.get(&key).unwrap().clone();
        let mut square_tmp = Square::from(&key);
        square_tmp.flip();
        patterns.insert(square_tmp.turn_to_pattern(), pattern.to_owned());
        square_tmp.flip();
        for _ in 0..3 {
            square_tmp.rotate();
            patterns.insert(square_tmp.turn_to_pattern(), pattern.to_owned());
            square_tmp.flip();
            patterns.insert(square_tmp.turn_to_pattern(), pattern.to_owned());
            square_tmp.flip();
        }
    }
    patterns
}

fn solve(patterns: &HashMap<String, String>) -> (usize, usize) {
    let mut result = Square::from(INITIAL_SQUARE);
    let mut silver = 0;
    for i in 0..18 {
        let mut board = result.divide();
        board.squares = board
            .squares
            .iter()
            .map(|row| {
                row.iter()
                    .map(|square| {
                        let mut result = Square::new();
                        let square_pattern = square.turn_to_pattern();
                        if patterns.contains_key(&square_pattern) {
                            result = Square::from(patterns.get(&square_pattern).unwrap());
                        }
                        if result == Square::new() {
                            panic!("couldn't find any pattern");
                        }
                        return result;
                    })
                    .collect()
            })
            .collect();
        result = board.flatten();
        if i == 4 {
            silver = result.count_turned_on();
        }
    }
    (silver, result.count_turned_on())
}
