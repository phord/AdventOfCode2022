#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

//------------------------------ PARSE INPUT

fn parse(_input: &'static str) -> Vec<(Vec<usize>,Vec<usize>)> {
    let mut sack = Vec::new();

    for line in _input.lines() {
        let mut row = Vec::new();
        for ch in line.as_bytes() {
            row.push((1 + *ch - if *ch >= b'a' {b'a'} else {b'A' - 26}) as usize);
        }
        let len = row.len()/2;
        let row2 = row.clone();
        let a = row.iter().take(len).map(|x| *x).collect();
        let b = row2.iter().skip(len).map(|x| *x).collect();
        sack.push((a,b));
    }

    sack
}

//------------------------------ SOLVE

fn solve1(_input: &'static str, _part: usize) -> usize {
    let _inp = parse(_input);
    let mut total = 0;
    for bag in _inp {
        let (foo, bar) = bag.clone();

        let foo: HashSet<usize> = HashSet::from_iter(foo.iter().cloned());
        let bar: HashSet<usize> = HashSet::from_iter(bar.iter().cloned());

        let baz: HashSet<&usize> = foo.intersection(&bar).collect();
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

    for g in 0..groups {
        let mut set: Vec<HashSet<usize>> = Vec::new();

        for elf in 0..3 {
            let (foo, bar) = &_inp[g*3 + elf];

            let mut s: HashSet<usize> = HashSet::from_iter(foo.iter().cloned());
            let bar: HashSet<usize> = HashSet::from_iter(bar.iter().cloned());
            s.extend(&bar);

            set.push(s);
        }
        let ans = set[0].iter().filter(|k| set[1].contains(k)).filter(|k| set[2].contains(k));
        for b in ans {
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