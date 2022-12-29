use core::fmt;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

#[derive(Clone, Copy)]
struct Crate(char);

struct Piles(Vec<Vec<Crate>>);

impl Piles {
    fn apply(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let i = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(i);
        }
    }

    fn apply_rev(&mut self, ins: Instruction) {
        let mut tmp: Vec<Crate> = vec![];
        for _ in 0..ins.quantity {
            let i = self.0[ins.src].pop().unwrap();
            tmp.push(i);
        }
        for c in tmp.iter().rev() {
            self.0[ins.dst].push(*c);
        }
    }
}

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }

        Ok(())
    }
}

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn main() -> color_eyre::Result<()> {
    let pa = include_str!("../../../inputs/day5/assert.1.txt");
    assert!(
        part_one(&pa) == "CMZ".to_string(),
        "part one should fucking work"
    );

    assert!(
        part_deux(&pa) == "MCD".to_string(),
        "part deux should fucking work"
    );

    let p1 = include_str!("../../../inputs/day5/input.1.txt");
    println!("{}", part_one(&p1));
    println!("{}", part_deux(&p1));

    Ok(())
}

fn part_one(s: &str) -> String {
    let mut lines = s.lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();
    let mut piles = Piles(transpose_rev(crate_lines));
    // println!("{piles:?}");

    // we've consumed the "numbers line" but not the separating line
    assert!(lines.next().unwrap().is_empty());

    let instructions: Vec<_> = lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();
    for ins in instructions {
        // println!("{ins:?}");
        piles.apply(ins);
        // println!("{piles:?}")
    }

    format!(
        "{}",
        piles.0.iter().map(|pile| pile.last().unwrap()).join("")
    )
}

fn part_deux(s: &str) -> String {
    let mut lines = s.lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();
    let mut piles = Piles(transpose_rev(crate_lines));
    // println!("{piles:?}");

    // we've consumed the "numbers line" but not the separating line
    assert!(lines.next().unwrap().is_empty());

    let instructions: Vec<_> = lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();
    for ins in instructions {
        // println!("{ins:?}");
        piles.apply_rev(ins);
        // println!("{piles:?}")
    }

    format!(
        "{}",
        piles.0.iter().map(|pile| pile.last().unwrap()).join("")
    )
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_space(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_block(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_space, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    separated_list1(tag(" "), parse_block)(i)
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

// convert from 1-indexed to 0-indexed
fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}
