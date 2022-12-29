use itertools::Itertools;
use std::ops::RangeInclusive;
use std::{cmp, fs};

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() -> color_eyre::Result<()> {
    let a: String = "inputs/day4/assert.1.txt".to_string();
    assert!(calculate_part_one(&a) == 2, "part one should fucking work");
    assert!(
        calculate_part_deux(&a) == 4,
        "part deux should fucking work"
    );

    let b: String = "inputs/day4/input.1.txt".to_string();
    let result = calculate_part_one(&b);
    println!("result: {}", result);

    let result = calculate_part_deux(&b);
    println!("result: {}", result);

    Ok(())
}

fn calculate_part_deux(file_name: &String) -> usize {
    dbg!(file_name);

    // let ids: Vec<((i32, i32), (i32, i32))> = fs::read_to_string(file_name)
    let lines = fs::read_to_string(file_name).unwrap().trim().to_string();

    dbg!(&lines);

    let pairs = &lines
        .lines()
        .map(|l| {
            l.split(",")
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("range start/end should be i32"))
                        .collect_tuple::<(i32, i32)>()
                        .map(|(start, end)| start..=end)
                        .expect("each range should have a start and end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each line must have a pair of ranges")
        })
        .filter(|(a, b)| a.overlaps_or_is_overlapped(b))
        .count();

    return *pairs;
}

fn calculate_part_one(file_name: &String) -> i32 {
    dbg!(file_name);

    // let ids: Vec<((i32, i32), (i32, i32))> = fs::read_to_string(file_name)
    let lines = fs::read_to_string(file_name)
        .expect("day3 assert file to exist")
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    dbg!(&lines);

    let pairs: Vec<(&str, &str)> = lines
        .iter()
        .map(|x| x.split(",").collect_tuple().expect("to make tuple"))
        .collect();
    dbg!(&pairs);

    let ids: Vec<((i32, i32), (i32, i32))> = pairs.iter().map(|x| create_ranges(*x)).collect();
    dbg!(&ids);

    let overlaps: Vec<i32> = ids.iter().map(|x| find_overlap(*x)).collect();
    dbg!(&overlaps);

    return overlaps.iter().sum();
}

fn create_ranges(v: (&str, &str)) -> ((i32, i32), (i32, i32)) {
    return (create_range(v.0), create_range(v.1));
}

fn create_range(s: &str) -> (i32, i32) {
    let range: Vec<&str> = s.split("-").collect();

    let l = range[0].to_string().parse::<i32>().unwrap();
    let r = range[1].to_string().parse::<i32>().unwrap();

    return (l, r);
}

fn find_overlap(elves: ((i32, i32), (i32, i32))) -> i32 {
    let ll = elves.0 .0;
    let lr = elves.0 .1;
    let rl = elves.1 .0;
    let rr = elves.1 .1;

    if ll <= rl && lr >= rr {
        return 1;
    }

    if rl <= ll && rr >= lr {
        return 1;
    }

    return 0;
}

// fn find_partial_overlap(elves: ((i32, i32), (i32, i32))) -> i32 {
//     let ll = elves.0 .0;
//     let lr = elves.0 .1;
//     let rl = elves.1 .0;
//     let rr = elves.1 .1;

//     if on_segment(ll, lr, rl) > 0 {
//         return 1;
//     }
//     if on_segment(lr, rl, rr) > 0 {
//         return 1;
//     }
//     if on_segment(rl, rr, ll) > 0 {
//         return 1;
//     }
//     if on_segment(rr, ll, lr) > 0 {
//         return 1;
//     }

//     return 0;
// }

// fn on_segment(p: i32, q: i32, r: i32) -> i32 {
//     if q <= cmp::max(p, r) && q >= cmp::min(p, r) {
//         return 1;
//     }
//     return 0;
// }
