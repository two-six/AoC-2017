const FACTORS: (usize, usize) = (16807, 48271);
// const GENERATORS: (usize, usize) = (65, 8921);
const GENERATORS: (usize, usize) = (679, 771);
const DIVIDER: usize = 2_147_483_647;
const PAIRS_NUMBER: usize = 40_000_000;

fn main() {
    println!("Silver: {}", silver());
    println!("Gold: {}", gold());
}

fn silver() -> usize {
    let mut result: usize = 0;
    let mut generators = GENERATORS;
    for _ in 0..PAIRS_NUMBER {
        next_value(&mut generators.0, &FACTORS.0, &DIVIDER);
        next_value(&mut generators.1, &FACTORS.1, &DIVIDER);
        if compare_last_16_bits(&generators.0, &generators.1) {
            result += 1;
        }
    }
    result
}

fn gold() -> usize {
    let mut result: usize = 0;
    let mut generators = GENERATORS;
    let mut i = 0;
    while i < 5_000_000 {
        next_value(&mut generators.0, &FACTORS.0, &DIVIDER);
        next_value(&mut generators.1, &FACTORS.1, &DIVIDER);
        while generators.0 % 4 != 0 {
            next_value(&mut generators.0, &FACTORS.0, &DIVIDER);
        }
        while generators.1 % 8 != 0 {
            next_value(&mut generators.1, &FACTORS.1, &DIVIDER);
        }
        if compare_last_16_bits(&generators.0, &generators.1) {
            result += 1;
        }
        i += 1;
    }
    result
}

fn compare_last_16_bits(generator_a: &usize, generator_b: &usize) -> bool {
    let mask = (1u32 << 16) - 1;
    *generator_a as u32 & mask == *generator_b as u32 & mask
}

fn next_value(generator: &mut usize, factor: &usize, divider: &usize) {
    *generator = (*generator * factor) % divider;
}
