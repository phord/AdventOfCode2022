#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<i64> {
    input.lines().map(|s| s.parse::<i64>().unwrap()).collect()
}
use std::iter;

//------------------------------ SOLVE

fn solve(input: &'static str, part: usize) -> i64 {
    let code = parse(input);
    let l = code.len() as i64;

    let mult = if part == 2 {811589153_i64} else {1};
    let loops = if part == 2 {10} else {1};

    let mut new:Vec<(usize, i64)> = code.iter().map(|x| *x as i64 * mult).enumerate().collect();
    for _ in 0..loops {
        for i in 0..l as usize {
            let pos = new.iter().position(|(x, _)| *x == i).unwrap();
            let cell = &new[pos];
            let c = cell.1;
            let cl = c.rem_euclid(l - 1) as usize;
            let p1 = &new[..pos];
            let p2 = &new[pos+1..];
            if cl > p2.len() {
                // c moves into p1
                let cl = cl - p2.len();
                let it1 = p1[..cl].iter();
                let it2 = p1[cl..].iter();
                new = it1.chain(iter::once(cell)).chain(it2).chain(p2.iter()).map(|x| *x).collect();
            } else {
                // c moves into p2
                let it1 = p2[..cl].iter();
                let it2 = p2[cl..].iter();
                new = p1.iter().chain(it1.chain(iter::once(cell)).chain(it2)).map(|x| *x).collect();
            }
        }
    }
    let zero = new.iter().position(|x| x.1 == 0).unwrap();
    [1000, 2000, 3000].iter().map(|x| new[(zero + x) % l as usize].1).sum()
}
#[test] fn test_day20_part1() { assert_eq!(solve1(_SAMPLE), _ANS1); }

fn solve1(input: &'static str) -> i64 { solve(input, 1) }
fn solve2(input: &'static str) -> i64 { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day20, part1)]
fn day20_part1(input: &'static str) -> i64 {
    let ans = solve1(input);
    assert_eq!(ans, 4224);
    ans
}

#[allow(unused)]
#[aoc(day20, part2)]
fn day20_part2(input: &'static str) -> i64 {
    let ans = solve2(input);
    assert_eq!(ans, 861907680486);
    ans
}

//------------------------------ TESTS

#[test] fn test_day20_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "1
2
-3
3
-2
0
4";

const _ANS1: i64 = 3;
const _ANS2: i64 = 1623178306;