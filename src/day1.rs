#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;


#[allow(unused)]
const SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[allow(unused)]
const ANS1: u64 = 24000;

#[allow(unused)]
const ANS2: u64 = 45000;

// Sum numbers separated by blank lines
fn elf_snacks(_input: &'static str) -> Vec<u64> {
    let inp = split_to_lines(_input);
    let elf = group_between(inp, "");

    elf.iter()
        .map(|x| x.iter()
                  .map(|v| parse_u64(v))
                  .sum())
        .collect()
}

#[aoc(day1, part1)]
fn day1_part1(_input: &'static str) -> u64 {
    let mut elf = elf_snacks(_input);

    elf.sort();
    elf.reverse();
    elf[0]
}

#[test]
fn test_day1_part1() {
    let ans = day1_part1(SAMPLE);
    assert_eq!(ans, ANS1);
}

//------------------------------ PART 2

#[aoc(day1, part2)]
fn day1_part2(_input: &'static str) -> u64 {
    let mut elf = elf_snacks(_input);

    elf.sort();
    elf.reverse();
    elf[0] + elf[1] + elf[2]
}

#[test]
fn test_day1_part2() {
    let ans = day1_part2(SAMPLE);
    assert_eq!(ans, ANS2);
}
