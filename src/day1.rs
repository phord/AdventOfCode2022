#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

// Sum numbers separated by blank lines
fn elf_snacks(_input: &'static str) -> Vec<u64> {
    split_on(_input, "\n\n")
        .iter()
        .map(|x| split_to_ints(x).iter().sum())
        .collect()
}

fn solve(_input: &'static str, count: usize) -> u64 {
    let mut elf = elf_snacks(_input);

    elf.sort();
    elf.reverse();
    assert!(count <= elf.len());
    elf.iter().take(count).sum()
}

//------------------------------ PART 1

#[aoc(day1, part1)]
fn day1_part1(_input: &'static str) -> u64 {
    solve(_input, 1)
}

#[test]
fn test_day1_part1() {
    assert_eq!(day1_part1(_SAMPLE), _ANS1);
}

//------------------------------ PART 2

#[aoc(day1, part2)]
fn day1_part2(_input: &'static str) -> u64 {
    solve(_input, 3)
}

#[test]
fn test_day1_part2() {
    assert_eq!(day1_part2(_SAMPLE), _ANS2);
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

const _ANS1: u64 = 24000;
const _ANS2: u64 = 45000;