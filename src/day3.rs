#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashSet;
use std::iter::FromIterator;

//------------------------------ PARSE INPUT

fn parse(_input: &'static str) -> Vec<Vec<usize>> {
    let mut sack = Vec::new();

    for line in _input.lines() {
        let row: Vec<usize> = line.as_bytes()
                                .iter()
                                .map(|ch| (*ch - if *ch >= b'a' {b'a' - 1} else {b'A' - 27}) as usize)
                                .collect();
        sack.push(row);
    }

    sack
}

//------------------------------ SOLVE

fn solve1(_input: &'static str, _part: usize) -> usize {
    let _inp = parse(_input);
    let mut total = 0;
    for bag in _inp {
        let len = bag.len()/2;
        let row2 = bag.clone();
        let a: HashSet<usize> = bag.iter().take(len).map(|x| *x).collect();
        let b: HashSet<usize> = row2.iter().skip(len).map(|x| *x).collect();

        let baz: HashSet<&usize> = a.intersection(&b).collect();
        for b in baz {
            total += *b;
        }
    }
    total
}

fn solve2(_input: &'static str, _part: usize) -> usize {
    let _inp = parse(_input);
    let mut total = 0;

    let groups = _inp.len() / 3;

    // For each group of 3 elves
    for g in 0..groups {
        let mut set: Vec<HashSet<usize>> = Vec::new();

        // For each elf
        for elf in 0..3 {
            let row = &_inp[g*3 + elf];
            set.push(HashSet::from_iter(row.iter().cloned()));
        }
        for b in set[0].iter()
                        .filter(|k| set[1].contains(k))
                        .filter(|k| set[2].contains(k)) {
            total += *b;
        }
    }
    total
}

//------------------------------ PART 1

#[allow(unused)]
#[aoc(day3, part1)]
fn day3_part1(_input: &'static str) -> usize {
    let ans = solve1(_input, 1);
    assert_eq!(ans, 8349);
    ans
}

#[test]
fn test_day3_part1() {
    assert_eq!(day3_part1(_SAMPLE), _ANS1);
}

//------------------------------ PART 2

#[allow(unused)]
#[aoc(day3, part2)]
fn day3_part2(_input: &'static str) -> usize {
    let ans = solve2(_input, 2);
    assert_eq!(ans, 2681);
    ans
}

#[test]
fn test_day3_part2() {
    assert_eq!(day3_part2(_SAMPLE), _ANS2);
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

const _ANS1: usize = 157;
const _ANS2: usize = 70;