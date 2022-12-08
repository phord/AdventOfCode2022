#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(_input: &'static str) -> Vec<u64> {
    split_to_ints(_input)
}

//------------------------------ SOLVE

fn solve(_input: &'static str, _part: usize) -> usize {
    let _inp = parse(_input);
    _part
}

//------------------------------ PART 1

#[allow(unused)]
// Uncomment next line when solution is ready
// #[aoc(day8, part1)]
fn day8_part1(_input: &'static str) -> usize {
    let ans = solve(_input, 1);
    // assert_eq!(ans, ___);
    ans
}

#[test]
fn test_day8_part1() {
    assert_eq!(solve(_SAMPLE, 1), _ANS1);
}

//------------------------------ PART 2

#[allow(unused)]
// Uncomment next line when solution is ready
// #[aoc(day8, part2)]
fn day8_part2(_input: &'static str) -> usize {
    let ans = solve(_input, 2);
    // assert_eq!(ans, ___);
    ans
}

#[test]
fn test_day8_part2() {
    assert_eq!(solve(_SAMPLE, 2), _ANS2);
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "1234";

const _ANS1: usize = 1;
const _ANS2: usize = 2;