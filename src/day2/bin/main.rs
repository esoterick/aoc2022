use core::fmt;
use std::{env, fs};

// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the
// round (0 if you lost, 3 if the round was a draw, and 6 if you won).

trait Score {
    fn score(&self) -> i32;
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(PartialEq, Debug, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Sissors = 3,
}

#[derive(PartialEq, Debug)]
struct Round {
    you: Shape,
    me: Shape,
    outcome: Outcome,
    points: i32,
}

impl Score for Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Sissors => 3,
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "You: {:?}, Me: {:?}: {:?} {}pts",
            self.you, self.me, self.outcome, self.points
        )
    }
}
//  X means you need to lose,
//  Y means you need to end the round in a draw,
//  Z means you need to win. Good luck!"
fn build_outcome(outcome_string: &str) -> Outcome {
    match outcome_string {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => Outcome::Loss,
    }
}

fn build_shape(shape_string: &str) -> Shape {
    match shape_string {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Sissors,
        _ => Shape::Rock,
    }
}

impl Shape {
    fn get_shape(&self, o: &Outcome) -> Shape {
        match *o {
            Outcome::Draw => self.clone(),
            Outcome::Win => match *self {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Sissors,
                Shape::Sissors => Shape::Rock,
            },
            Outcome::Loss => match *self {
                Shape::Rock => Shape::Sissors,
                Shape::Paper => Shape::Rock,
                Shape::Sissors => Shape::Paper,
            },
        }
    }

    fn beats(&self, you: &Shape) -> Outcome {
        match self {
            Shape::Rock => {
                if *you == Shape::Rock {
                    println!("rock -> rock");
                    return Outcome::Draw;
                }
                if *you == Shape::Paper {
                    println!("rock -> paper");
                    return Outcome::Loss;
                }
                println!("rock -> sissors");
                return Outcome::Win;
            }
            Shape::Paper => {
                if *you == Shape::Paper {
                    println!("paper -> paper");
                    return Outcome::Draw;
                }
                if *you == Shape::Rock {
                    println!("paper -> rock");
                    return Outcome::Win;
                }
                println!("paper -> sissors");
                return Outcome::Loss;
            }
            Shape::Sissors => {
                if *you == Shape::Sissors {
                    println!("sissors -> sissors");
                    return Outcome::Draw;
                }
                if *you == Shape::Paper {
                    println!("sissors -> paper");
                    return Outcome::Win;
                }
                println!("sissors -> rock");
                return Outcome::Loss;
            }
        }
    }

    fn points(&self, o: &Outcome) -> i32 {
        let shape_score: i32 = self.score();
        let outcome_score: i32 = o.score();
        shape_score + outcome_score
    }
}

fn build_round(round_string: String) -> Round {
    dbg!(&round_string);
    let r: Vec<&str> = round_string.split(" ").collect();

    let you = build_shape(r[0]);
    let me = build_shape(r[1]);

    let outcome = me.beats(&you);
    let points = me.points(&outcome);

    Round {
        you,
        me,
        outcome,
        points,
    }
}

fn build_round_part2(round_string: String) -> Round {
    dbg!(&round_string);
    let r: Vec<&str> = round_string.split(" ").collect();

    let you = build_shape(r[0]);
    let outcome = build_outcome(r[1]);

    let me = you.get_shape(&outcome);

    let points = me.points(&outcome);

    Round {
        you,
        me,
        outcome,
        points,
    }
}

fn main() -> color_eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let contents = fs::read_to_string(input_path)?;
    let c = contents.trim();
    let c2 = contents.trim();

    let rounds: Vec<Round> = c.split("\n").map(|r| build_round(r.to_string())).collect();

    let mut sum = 0;
    for (i, r) in rounds.iter().enumerate() {
        sum = sum + r.points;
        println!("{: >3} {:?}", i, r);
    }
    println!("ans 1: {}", sum);

    let rounds_2: Vec<Round> = c2
        .split("\n")
        .map(|r| build_round_part2(r.to_string()))
        .collect();

    let mut sum_2 = 0;
    for (i, r) in rounds_2.iter().enumerate() {
        sum_2 = sum_2 + r.points;
        println!("{: >3} {:?}", i, r);
    }

    println!("ans 2: {}", sum_2);

    Ok(())
}
