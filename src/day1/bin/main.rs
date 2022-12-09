use std::{env, fs, ops::Index};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let contents = fs::read_to_string(input_path).expect("to be able to read our input file");
    println!("{:?}", contents);
    let mut elves: Vec<i32> = contents
        .trim()
        .split("\n\n")
        .map(|s| {
            s.to_string()
                .split("\n")
                .map(|x| x.parse::<i32>().expect("kjashgdja"))
                .sum::<i32>()
        })
        .collect();
    elves.sort();
    println!("{:?}", elves);

    let mut max = 0;
    for (pos, e) in elves.iter().enumerate() {
        if e > &elves[max] {
            println!("elf[{}]: {} > {}", pos, e, &elves[max]);
            max = pos
        }
    }
    println!("idx {}", max);
    println!("ans {}", elves[max]);

    let c = elves.len() - 3;
    let top3 = &elves[c..];

    for e in top3.iter() {
        println!("elf: {}", e);
    }

    println!("ans {}", &top3.iter().sum::<i32>());
}
