use std::fs;

const LISTSIZE: usize = 256;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    println!("Silver: {}", silver(&input));
    println!("Gold: {}", gold(&input));
}

fn silver(input: &str) -> usize {
    let mut list_numbers: [u8; LISTSIZE] = {
        let mut tmp_list: [u8; LISTSIZE] = [0; LISTSIZE];
        for i in 0..LISTSIZE {
            tmp_list[i] = i as u8;
        }
        tmp_list
    };
    let mut current_position: usize = 0;
    let lengths_sequence: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut skip_size: usize = 0;
    lengths_sequence.iter().for_each(|length| {
        reverse_list(&mut list_numbers, current_position, *length as usize);
        current_position += length + skip_size;
        skip_size += 1;
    });
    list_numbers[0] as usize * list_numbers[1] as usize
}

fn gold(input: &str) -> String {
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
