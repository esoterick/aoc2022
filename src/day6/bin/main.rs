use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    let p1 = include_str!("../../../inputs/day6/input.1.txt");
    println!(
        "part one: {}",
        part_one(&p1).expect("part one should return some fucking value")
    );

    println!(
        "part deux: {}",
        part_deux(&p1).expect("part one should return some fucking value")
    );

    Ok(())
}

fn part_one(s: &str) -> Option<usize> {
    find_marker(s, 4)
}

fn part_deux(s: &str) -> Option<usize> {
    find_marker(s, 14)
}

fn find_marker(s: &str, u: usize) -> Option<usize> {
    s.as_bytes()
        .windows(u)
        .position(|w| w.iter().unique().count() == u)
        .map(|p| p + u)
}

#[test]
fn test_example_one() {
    assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
}

#[test]
fn test_example_two() {
    assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
}

#[test]
fn test_example_three() {
    assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
}

#[test]
fn test_example_four() {
    assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
}

#[test]
fn test_example_five() {
    assert_eq!(part_deux("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
}

#[test]
fn test_example_six() {
    assert_eq!(part_deux("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
}

#[test]
fn test_example_seven() {
    assert_eq!(part_deux("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
}

#[test]
fn test_example_eight() {
    assert_eq!(part_deux("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
}

#[test]
fn test_example_nine() {
    assert_eq!(part_deux("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
}
