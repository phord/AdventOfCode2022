use yaah::{aoc, aoc_lib, aoc_year};

mod day1;
mod day2;
mod day3;

aoc_year!(2022);

#[allow(unused)]
pub fn split_to_lines(_input: &'static str) -> Vec<&str> {
    _input.lines().collect::<Vec<&str>>()
}

#[allow(unused)]
pub fn split_to_words(_input: &'static str) -> Vec<Vec<&str>> {
    _input.lines()
        .map(|s| s.split_whitespace().collect())
        .collect::<Vec<Vec<&str>>>()
}

#[allow(unused)]
pub fn split_to_ints(_input: &'static str) -> Vec<u64> {
    _input.lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn binary(s: &str) -> u64 {
    let mut res = 0;
    for c in s.chars() {
        res *= 2;
        if c == '1' { res += 1; }
    }
    res
}

aoc_lib!(with_benchmarks);
