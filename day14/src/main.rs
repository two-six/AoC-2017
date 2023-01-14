// const HASH_INPUT: &str = "flqrgnkx";
const HASH_INPUT: &str = "nbysizxe";

fn main() {
    println!("Silver: {}", silver(HASH_INPUT));
    println!("Gold: {}", gold(HASH_INPUT));
}

fn silver(input: &str) -> usize {
    let mut result = 0;
    for i in 0..128 {
        result += turn_decimal(&knot_hash(&(input.to_owned() + "-" + &i.to_string())))
            .replace("0", "")
            .len();
    }
    result
}

fn gold(input: &str) -> usize {
    let mut board: Vec<(usize, usize)> = Vec::new();
    for i in 0..128 {
        let line = turn_decimal(&knot_hash(&(input.to_owned() + "-" + &i.to_string())));
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '1' {
                board.push((j, i));
            }
        });
    }
    let mut result = 0;

    while board.len() > 0 {
        let (x, y) = board[0];
        remove_group(&mut board, x, y);
        result += 1;
    }
    result
}

fn remove_group(board: &mut Vec<(usize, usize)>, x: usize, y: usize) {
    board.remove(board.iter().position(|&p| p == (x, y)).unwrap());
    if x < 127 && board.contains(&(x + 1, y)) {
        remove_group(board, x + 1, y);
    }

    if y < 127 && board.contains(&(x, y + 1)) {
        remove_group(board, x, y + 1);
    }

    if x > 0 && board.contains(&(x - 1, y)) {
        remove_group(board, x - 1, y);
    }

    if y > 0 && board.contains(&(x, y - 1)) {
        remove_group(board, x, y - 1);
    }
}

fn turn_decimal(hash: &str) -> String {
    hash.chars()
        .map(|c| format!("{:04b}", i32::from_str_radix(&c.to_string(), 16).unwrap()))
        .fold(String::from(""), |acc, c| acc + &c)
}

fn knot_hash(input: &str) -> String {
    const LISTSIZE: usize = 256;
    fn reverse_list(list: &mut [u8; LISTSIZE], start: usize, length: usize) {
        if length <= 1 {
            return;
        }
        let length = length - 1;
        for i in 0..length {
            if start + i >= start + (length - i) {
                break;
            }
            let tmp_value = list[(start + i) % LISTSIZE];
            list[(start + i) % LISTSIZE] = list[(start + (length - i)) % LISTSIZE];
            list[(start + (length - i)) % LISTSIZE] = tmp_value;
        }
    }
    let mut list_numbers: [u8; LISTSIZE] = {
        let mut tmp_list: [u8; LISTSIZE] = [0; LISTSIZE];
        for i in 0..LISTSIZE {
            tmp_list[i] = i as u8;
        }
        tmp_list
    };
    let lengths_sequence: Vec<usize> = [
        input.trim().bytes().map(|byte| byte as usize).collect(),
        vec![31, 73, 47, 23],
    ]
    .join(&17);

    // Run 64 rounds
    let mut current_position: usize = 0;
    let mut skip_size: usize = 0;
    for _ in 0..64 {
        lengths_sequence.iter().for_each(|length| {
            reverse_list(&mut list_numbers, current_position, *length as usize);
            current_position += length + skip_size;
            skip_size += 1;
        });
    }

    // Parsing list into hexadecimal string and returning it
    list_numbers
        .chunks(16)
        .map(|sparse| sparse.iter().fold(0, |acc, x| acc ^ x))
        .map(|x| {
            let hex = format!("{x:x}");
            if hex.len() == 1 {
                return String::from("0") + &hex;
            }
            hex
        })
        .fold(String::from(""), |acc, x| acc + &x)
}
