use yaah::aoc;
use crate::*;

#[derive(Debug)]
enum Movement {
    Forward(u64),
    Up(u64),
    Down(u64),
    Bad
}

#[allow(unused)]
fn split_to_movement(_input: &'static str) -> Vec<Movement> {
    let parsed = split_to_words(_input);
    parsed.iter().map(|x| {
        let dist = x[1].parse::<u64>().unwrap();
        match x[0] {
            "forward" => Movement::Forward(dist),
            "up" => Movement::Up(dist),
            "down" => Movement::Down(dist),
            _ => Movement::Bad,
        }
    }).collect()
}

#[aoc(day2, part1)]
fn day2_part1(_input: &'static str) -> u64 {
    let moves = split_to_movement(_input);
    // println!("{:?}", moves);
    let mut pos: u64 = 0;
    let mut depth: u64 = 0;

    for m in moves {
        match m {
            Movement::Forward(x) => pos += x,
            Movement::Up(x) => depth -= x,
            Movement::Down(x) => depth += x,
            Movement::Bad => println!("FAILED"),
        }
    }
    pos * depth
}

#[aoc(day2, part2)]
fn day2_part2(_input: &'static str) -> u64 {
    let moves = split_to_movement(_input);
    // println!("{:?}", moves);
    let mut pos: u64 = 0;
    let mut aim: u64 = 0;
    let mut depth: u64 = 0;

    for m in moves {
        match m {
            Movement::Forward(x) => {pos += x; depth += aim*x;},
            Movement::Up(x) => aim -= x,
            Movement::Down(x) => aim += x,
            Movement::Bad => println!("FAILED"),
        }
    }
    pos * depth
}

#[allow(unused)]
const SAMPLE_DAY2: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[test]
fn test_day2_part1() {
    let ans = day2_part1(SAMPLE_DAY2);
    assert_eq!(ans, 150);
}

#[test]
fn test_day2_part2() {
    let ans = day2_part2(SAMPLE_DAY2);
    assert_eq!(ans, 900);
}
