use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let contents = fs::read_to_string(input_path).expect("to be able to read our input file");
    let mut elves: Vec<i32> = contents
        .trim()
        .split("\n\n")
        .map(|s| {
            s.to_string()
                .split("\n")
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();
    elves.sort();

    let mut max = 0;
    for (pos, e) in elves.iter().enumerate() {
        if e > &elves[max] {
            max = pos
        }
    }
    println!("part 1: {}", elves[max]);

    let c = elves.len() - 3;
    let top3 = &elves[c..];

    println!("part 2: {}", &top3.iter().sum::<i32>());
}
