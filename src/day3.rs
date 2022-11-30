use yaah::aoc;
#[allow(unused)]
use crate::*;


#[allow(unused)]
const SAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

#[allow(unused)]
const ANS1: u64 = 198;

#[allow(unused)]
const ANS2: u64 = 230;

fn count_bits(inp: &Vec<&str>) -> Vec<usize> {
    let width = inp[0].len();
    let mut vec = vec![0; width];

    for x in inp {
        for (i, c) in x.chars().enumerate() {
            if c == '1' { vec[i] += 1 }
        }
    }
    vec
}

#[aoc(day3, part1)]
fn day3_part1(_input: &'static str) -> u64 {
    let inp = split_to_lines(_input);
    // println!("{:#?}", inp);
    let width = inp[0].len();
    let height = inp.len();
    let vec = count_bits(&inp);

    // println!("{:#?}", vec);

    let mut gamma = 0;
    for count in vec {
        gamma *= 2;
        if count > height/2 { gamma += 1; }
    }
    let epsilon = (1u64<<width) - 1 - gamma;
    // println!("{:#?}  {} ", gamma, epsilon);
    gamma * epsilon
}

#[test]
fn test_day3_part1() {
    let ans = day3_part1(SAMPLE);
    assert_eq!(ans, ANS1);
}

//------------------------------ PART 2


#[aoc(day3, part2)]
fn day3_part2(_input: &'static str) -> u64 {
    let inp = split_to_lines(_input);
    // println!("{:#?}", inp);
    let width = inp[0].len();

    // Oxygen
    let mut search = inp.clone();
    for p in 0..width {
        let vec = count_bits(&search);
        let height = search.len();
        let bit = if vec[p]*2 >= height {b'1'} else {b'0'};
        // println!("{}  count={:?}  bit={}  {:?}", p, vec, bit, search);
        search.retain(|&x| x.as_bytes()[p] == bit);
        if search.len() == 1 {break;}
    }
    let oxygen = binary(search[0]);
    // println!("Oxygen {:?}", search);

    // co2
    let mut search = inp.clone();
    for p in 0..width {
        let vec = count_bits(&search);
        let height = search.len();
        let bit = if vec[p]*2 >= height {b'0'} else {b'1'};
        // println!("{}  count={:?}  bit={}  {:?}", p, vec, bit, search);
        search.retain(|&x| x.as_bytes()[p] == bit);
        if search.len() == 1 {break;}
    }
    // println!("CO2 {:?}", search);
    let co2 = binary(search[0]);

    oxygen * co2
}

#[test]
fn test_day3_part2() {
    let ans = day3_part2(SAMPLE);
    assert_eq!(ans, ANS2);
}
