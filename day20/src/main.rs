use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Particle {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
}

impl Particle {
    fn new(
        position: (i64, i64, i64),
        velocity: (i64, i64, i64),
        acceleration: (i64, i64, i64),
    ) -> Particle {
        Particle {
            position,
            velocity,
            acceleration,
        }
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.position.0 == other.position.0
            && self.position.1 == other.position.1
            && self.position.2 == other.position.2
    }
}

impl Eq for Particle {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let particles = parse(&input);
    println!("Silver: {}", silver(&particles));
    println!("Gold: {}", gold(&particles));
}

fn gold(particles: &Vec<Particle>) -> usize {
    let mut largest_n = 0;
    for i in 0..particles.len() {
        for j in i + 1..particles.len() {
            if let Some(n) = find_n(&particles[i], &particles[j]) {
                if n > largest_n {
                    largest_n = n;
                }
            }
        }
    }
    simulate(&particles, largest_n as usize).len()
}

fn silver(particles: &Vec<Particle>) -> usize {
    particles
        .iter()
        .enumerate()
        .min_by(|(_, x), (_, y)| {
            (x.acceleration.0.abs() + x.acceleration.1.abs() + x.acceleration.2.abs())
                .cmp(&(y.acceleration.0.abs() + y.acceleration.1.abs() + y.acceleration.2.abs()))
        })
        .map(|(i, _)| i)
        .unwrap()
}

fn simulate(particles: &Vec<Particle>, n: usize) -> Vec<Particle> {
    let mut result = remove_duplicates(&particles);
    for i in 1..n + 1 {
        result.iter_mut().for_each(|p| {
            p.position.0 += p.velocity.0 + i as i64 * p.acceleration.0;
            p.position.1 += p.velocity.1 + i as i64 * p.acceleration.1;
            p.position.2 += p.velocity.2 + i as i64 * p.acceleration.2;
        });
        result = remove_duplicates(&result);
    }
    result
}

fn remove_duplicates(particles: &Vec<Particle>) -> Vec<Particle> {
    let mut result: Vec<Particle> = Vec::new();
    for i in 0..particles.len() {
        let mut unique = true;
        for j in 0..particles.len() {
            if particles[i] == particles[j] && i != j {
                unique = false;
                break;
            }
        }
        if unique {
            result.push(particles[i]);
        }
    }
    result
}

fn find_n(p1: &Particle, p2: &Particle) -> Option<i64> {
    let mut prev_diff = i64::max_value();
    let mut x0 = p1.position.0;
    let mut y0 = p1.position.1;
    let mut z0 = p1.position.2;
    let mut x1 = p2.position.0;
    let mut y1 = p2.position.1;
    let mut z1 = p2.position.2;
    for n in 1.. {
        if x0 == x1 && y0 == y1 && z0 == z1 {
            return Some(n);
        }
        x0 += p1.velocity.0 + n * p1.acceleration.0;
        y0 += p1.velocity.1 + n * p1.acceleration.1;
        z0 += p1.velocity.2 + n * p1.acceleration.2;
        x1 += p2.velocity.0 + n * p2.acceleration.0;
        y1 += p2.velocity.1 + n * p2.acceleration.1;
        z1 += p2.velocity.2 + n * p2.acceleration.2;
        let diff = (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs();
        if diff >= prev_diff {
            break;
        }
        prev_diff = diff;
    }
    None
}

fn parse(input: &str) -> Vec<Particle> {
    let regex =
        Regex::new(r"p=<(.*?),(.*?),(.*?)>,\s+v=<(.*?),(.*?),(.*?)>,\s+a=<(.*?),(.*?),(.*?)>")
            .unwrap();
    let mut result: Vec<Particle> = Vec::new();
    for cap in regex.captures_iter(input) {
        let position = (
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
            cap[3].parse::<i64>().unwrap(),
        );
        let velocity = (
            cap[4].parse::<i64>().unwrap(),
            cap[5].parse::<i64>().unwrap(),
            cap[6].parse::<i64>().unwrap(),
        );
        let acceleration = (
            cap[7].parse::<i64>().unwrap(),
            cap[8].parse::<i64>().unwrap(),
            cap[9].parse::<i64>().unwrap(),
        );
        result.push(Particle::new(position, velocity, acceleration));
    }
    result
}
