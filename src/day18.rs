#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<Vec<i32>> {
    input.lines().map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect()).collect()
}

//------------------------------ SOLVE

fn is_adjacent(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    (0..3).map(|x:usize| (a[x]-b[x]).abs()).sum::<i32>() == 1
}

fn solve1(input: &'static str, part: usize) -> usize {
    let p = parse(input);
    let mut count_adj = 0;
    for a in &p {
        for b in &p {
            if is_adjacent(a, b) {
                count_adj += 1;
            }
        }
    }


    p.len() * 6 - count_adj
}

fn solve2(input: &'static str, part: usize) -> usize {
    let p = parse(input);
    let mut count_adj = 0;
    for a in &p {
        for b in &p {
            if is_adjacent(a, b) {
                count_adj += 1;
            }
        }
    }


    p.len() * 6 - count_adj
}
//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day18, part1)]
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

#[test] fn test_day18_part1() { assert_eq!(solve1(_SAMPLE), 64); }
#[test] fn test_day18_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

const _ANS1: usize = 1;
const _ANS2: usize = 2;