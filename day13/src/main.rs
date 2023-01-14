use std::fs;

const FIREWALL_LENGTH: usize = 93;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read from the file");
    let mut firewall: [(usize, usize, bool); FIREWALL_LENGTH] = [(0, 0, true); FIREWALL_LENGTH];

    input.lines().for_each(|line| {
        let values: Vec<usize> = line
            .split(": ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        firewall[*values.get(0).unwrap()] = (*values.get(1).unwrap(), 0, true);
    });
    println!("Silver: {}", silver(&firewall));
    println!("Gold: {}", gold(&firewall));
}

fn silver(firewall: &[(usize, usize, bool); FIREWALL_LENGTH]) -> usize {
    let mut result: usize = 0;
    for i in 0..FIREWALL_LENGTH as i32 {
        let (range, packet, up) = firewall.get(i as usize).unwrap();
        let (range, packet) = (*range as i32, *packet as i32);
        let cycle = ((range - 1) * 2) as i32;
        if range != 0
            && ((*up && ((2 * cycle - packet) - i) % cycle == 0)
                || (!(*up) && (packet - i) % cycle == 0))
        {
            result += i as usize * firewall.get(i as usize).unwrap().0;
        }
    }
    result
}

fn gold(firewall: &[(usize, usize, bool); FIREWALL_LENGTH]) -> usize {
    let mut i = 0;
    loop {
        if test_path(&firewall, i as i32) {
            return i;
        };
        i += 1;
    }
}

fn test_path(firewall: &[(usize, usize, bool); FIREWALL_LENGTH], delay: i32) -> bool {
    for i in 0..FIREWALL_LENGTH as i32 {
        let (range, packet, up) = firewall.get(i as usize).unwrap();
        let (range, packet) = (*range as i32, *packet as i32);
        let cycle = ((range - 1) * 2) as i32;
        if range != 0
            && ((*up && ((2 * cycle - packet) - (i + delay)) % cycle == 0)
                || (!(*up) && (packet - (i + delay)) % cycle == 0))
        {
            return false;
        }
    }
    true
}
