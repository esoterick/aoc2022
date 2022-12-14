use itertools::Itertools;
use std::fs;

fn main() -> color_eyre::Result<()> {
    let a: String = "inputs/day3/assert.1.txt".to_string();
    assert!(calculate_day3(&a) == 157, "fak");
    assert!(calculate_part2(&a) == 70, "fak part deux");

    let b: String = "inputs/day3/input.1.txt".to_string();
    let result = calculate_day3(&b);
    println!("result: {}", result);

    let result = calculate_part2(&b);
    println!("result: {}", result);

    Ok(())
}

fn calculate_part2(file_name: &String) -> i32 {
    let sacks: Vec<String> = fs::read_to_string(file_name)
        .expect("day3 assert file to exist")
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    dbg!(&sacks);

    return find_group_badges(sacks).iter().sum();
}

fn find_group_badges(v: Vec<String>) -> Vec<i32> {
    let mut badges: Vec<i32> = Vec::new();

    for (a, b, c) in v.iter().tuples() {
        for x in a.chars() {
            if b.contains(x) && c.contains(x) {
                // dbg!(a, b, c, x);
                badges.push(get_priority(x));
                break;
            }
        }
    }

    dbg!(&badges);

    return badges;
}

fn calculate_day3(file_name: &String) -> i32 {
    let sacks: Vec<String> = fs::read_to_string(file_name)
        .expect("day3 assert file to exist")
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    dbg!(&sacks);

    let pockets: Vec<(String, String)> = sacks.iter().map(|p| split_pockets(p)).collect();
    dbg!(&pockets);

    let matches: Vec<i32> = pockets.iter().map(|x| find_match(x)).collect();
    dbg!(&matches);

    return matches.iter().sum();
}

fn split_pockets(sack: &String) -> (String, String) {
    let (left, right) = sack.split_at(sack.len() / 2);
    return (
        left.chars().sorted().unique().collect::<String>(),
        right.chars().sorted().unique().collect::<String>(),
    );
}

fn find_match(pockets: &(String, String)) -> i32 {
    let left = &pockets.0;
    let right = &pockets.1;

    for c in left.chars() {
        if right.contains(c) {
            return get_priority(c);
        }
    }
    return 0;
}

fn get_priority(c: char) -> i32 {
    return match c {
        'a'..='z' => (c as i32) - 96,
        'A'..='Z' => (c as i32) - 38,
        _ => 0,
    };
}
