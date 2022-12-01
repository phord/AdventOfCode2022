use yaah::aoc;
use crate::*;

#[aoc(day1, part1)]
fn day1_part1(_input: &'static str) -> usize {
    let inp = split_to_ints(_input);
    let (_, c) = inp.iter()
        .fold((&inp[0], 0), |(prev, count), x| (x, if x > prev {count + 1} else {count}));
    c
}

#[allow(unused)]
static SAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

#[test]
fn test_day1_part1() {
    let ans = day1_part1(SAMPLE);
    assert_eq!(ans, 7);
}

#[aoc(day1, part2)]
fn day1_part2(_input: &'static str) -> u64 {
    let inp = split_to_ints(_input);
    let a = inp.iter();
    let b = inp.iter().skip(1);
    let c = inp.iter().skip(2);

    let sums:Vec<u64> = a.zip(b.zip(c)).map(|(a, (b, c))| a+b+c).collect();

    let (_, c) = sums.iter()
        .fold((&sums[0], 0), |(prev, count), x| { (x, if x > prev {count + 1} else {count} ) });

    c
}

#[test]
fn test_day1_part2() {
    let ans = day1_part2(SAMPLE);
    assert_eq!(ans, 5);
}
