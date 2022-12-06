#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashSet;

//------------------------------ SOLVE

fn solve(_input: &'static str, _part: usize) -> usize {
    let len = if _part == 1 { 4 } else { 14 };
    for i in 0.._input.len() {
        let sub: HashSet<char> = _input[i..i+len].chars().collect();
        if sub.len() == len {
            return i+len;
        }
    }
    0
}

//------------------------------ PART 1

#[aoc(day6, part1)]
fn day6_part1(_input: &'static str) -> usize {
    let ans = solve(_input, 1);
    assert_eq!(ans, 1766);
    ans
}

#[test]
fn test_day6_part1() {
    assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 1), 7);
    assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 1), 5);
    assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg", 1), 6);
    assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 1), 10);
    assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 1), 11);
}

//------------------------------ PART 2

#[aoc(day6, part2)]
fn day6_part2(_input: &'static str) -> usize {
    let ans = solve(_input, 2);
    assert_eq!(ans, 2383);
    ans
}

#[test]
fn test_day6_part2() {
    assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 2), 19);
    assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 2), 23);
    assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg", 2), 23);
    assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 2), 29);
    assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 2), 26);
}