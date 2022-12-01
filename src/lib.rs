use yaah::{aoc_lib, aoc_year};
use std::str;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

aoc_year!(2022);
aoc_lib!(with_benchmarks);

//______________________________________________________
//                               GENERIC INPUT SPLITTERS

// Split a string containing multiple lines into a vector of &[u8], one per line
#[allow(unused)]
pub fn split_to_lines(_input: &'static str) -> Vec<&[u8]> {
    _input
        .lines()
        .map(|x| x.as_bytes())
        .collect::<Vec<&[u8]>>()
}

// Split a string containing multiple lines into a vector of vectors of words broken on whitespace
#[allow(unused)]
pub fn split_to_words(_input: &'static str) -> Vec<Vec<&[u8]>> {
    _input
        .lines()
        .map(|s| s.split_whitespace()
                        .map(|x| x.as_bytes())
                        .collect())
        .collect::<Vec<Vec<&[u8]>>>()
}

// Split a string containing single integer values into a Vec<u64>
#[allow(unused)]
pub fn split_to_ints(_input: &'static str) -> Vec<u64> {
    _input.lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

//______________________________________________________
//                                               PARSERS
#[allow(unused)]
fn parse_u64(inp: &[u8]) -> u64 {
    let s = str::from_utf8(inp).expect("invalid utf8 bytes");
    let x = s.parse::<u64>();
    assert!(x.is_ok(), "not a number: '{}'", s);
    x.unwrap()
}

// Convert a binary string to a u64 value.
#[allow(unused)]
fn parse_binary(s: &[u8]) -> u64 {
    let mut res = 0;
    for c in s {
        res *= 2;
        if *c == b'1' { res += 1; }
    }
    res
}