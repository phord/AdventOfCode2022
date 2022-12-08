#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<u64> {
    split_to_ints(input)
}

//------------------------------ SOLVE

fn solve(input: &'static str, part: usize) -> usize {
    parse(input);
    part
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
// Uncomment next line when solution is ready
// #[aoc(day18, part1)]
fn day18_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
// Uncomment next line when solution is ready
// #[aoc(day18, part2)]
fn day18_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ TESTS

#[test] fn test_day18_part1() { assert_eq!(solve1(_SAMPLE), _ANS1); }
#[test] fn test_day18_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "1234";

const _ANS1: usize = 1;
const _ANS2: usize = 2;