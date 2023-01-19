fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let components: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let parts = line.split_once('/').unwrap();
            (
                parts.0.parse::<usize>().unwrap(),
                parts.1.parse::<usize>().unwrap(),
            )
        })
        .collect();
    let result = solve(&components);
    println!("Silver: {}", result.0);
    println!("Gold: {}", result.1);
}

fn solve(components: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut silver = 0;
    components
        .iter()
        .filter(|&&c| c.0 == 0 || c.1 == 0)
        .for_each(|c| {
            let tmp = find_largest(&components, &Vec::new(), c, 0);
            if tmp > silver {
                silver = tmp;
            }
        });
    let mut gold = Vec::new();
    components
        .iter()
        .filter(|&&c| c.0 == 0 || c.1 == 0)
        .for_each(|c| {
            let tmp = find_longest_largest(&components, &Vec::new(), c, 0);
            if tmp.len() > gold.len() {
                gold = tmp;
            } else if tmp.len() == gold.len() {
                if tmp
                    .iter()
                    .map(|c| c.0 + c.1)
                    .fold(0, |acc, next| acc + next)
                    > gold
                        .iter()
                        .map(|c| c.0 + c.1)
                        .fold(0, |acc, next| acc + next)
                {
                    gold = tmp;
                }
            }
        });
    (
        silver,
        gold.iter()
            .map(|c| c.0 + c.1)
            .fold(0, |acc, next| acc + next),
    )
}

fn find_largest(
    components: &Vec<(usize, usize)>,
    used_components: &Vec<(usize, usize)>,
    prev: &(usize, usize),
    prev_el: usize,
) -> usize {
    let mut used_components = used_components.clone();
    used_components.push(*prev);
    let searched: usize = if prev.0 == prev_el { prev.1 } else { prev.0 };
    let mut result = 0;
    components
        .iter()
        .filter(|&&c| (c.0 == searched || c.1 == searched) && !used_components.contains(&c))
        .for_each(|c| {
            let tmp = find_largest(&components, &used_components, c, searched);
            if tmp > result {
                result = tmp;
            }
        });
    if result != 0 {
        return result;
    }
    used_components
        .iter()
        .map(|c| c.0 + c.1)
        .fold(0, |acc, next| acc + next)
}

fn find_longest_largest(
    components: &Vec<(usize, usize)>,
    used_components: &Vec<(usize, usize)>,
    prev: &(usize, usize),
    prev_el: usize,
) -> Vec<(usize, usize)> {
    let mut used_components = used_components.clone();
    used_components.push(*prev);
    let searched: usize = if prev.0 == prev_el { prev.1 } else { prev.0 };
    let mut result = Vec::new();
    components
        .iter()
        .filter(|&&c| (c.0 == searched || c.1 == searched) && !used_components.contains(&c))
        .for_each(|c| {
            let tmp = find_longest_largest(&components, &used_components, c, searched);
            if tmp.len() > result.len() {
                result = tmp;
            } else if tmp.len() == result.len() {
                if tmp
                    .iter()
                    .map(|c| c.0 + c.1)
                    .fold(0, |acc, next| acc + next)
                    > result
                        .iter()
                        .map(|c| c.0 + c.1)
                        .fold(0, |acc, next| acc + next)
                {
                    result = tmp;
                }
            }
        });
    if !result.is_empty() {
        return result;
    }
    used_components
}
