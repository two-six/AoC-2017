use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    println!("Silver: {}", calculate(&input, 1));
    println!("Gold: {}", calculate(&input, input.len() / 2));
}

fn calculate(captcha: &str, steps: usize) -> i32 {
    let mut result: i32 = 0;
    let captcha_chars: Vec<u8> = captcha.bytes().collect();
    for i in 0..captcha_chars.len() - steps {
        if captcha_chars[i] == captcha_chars[i + steps] {
            result += captcha_chars[i].as_number();
        }
    }
    for i in captcha_chars.len() - steps..captcha_chars.len() {
        if captcha_chars[i] == captcha_chars[i - (captcha_chars.len() - steps)] {
            result += captcha_chars[i].as_number();
        }
    }
    result
}

trait ConvertBytes {
    fn as_number(self) -> i32;
}

impl ConvertBytes for u8 {
    fn as_number(self) -> i32 {
        (self - 48) as i32
    }
}
