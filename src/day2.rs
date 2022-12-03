#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(_input: &'static str) -> Vec<(usize, usize)> {
    split_to_words(_input)
        .iter()
        .map(|game| ((game[0][0] - b'A') as usize, (game[1][0] - b'X') as usize) )
        .collect()
}

//------------------------------ SOLVE

fn draw(them: usize) -> usize {
    them
}

fn win(them: usize) -> usize {
    (them + 1) % 3
}

fn lose(them: usize) -> usize {
    (them + 2) % 3
}

fn solve(_input: &'static str, part: usize) -> usize {
    parse(_input)
        .iter()
        .map(|(them, mut us)|
        {
            let them = *them;
            if part == 2 {
                // For part 2, "us" tells us what to do
                //   0 = lose, 1 = draw, 2 = win
                us = match us {
                    0 => lose(them),
                    1 => draw(them),
                    2 => win(them),
                    _ => panic!(),
                };
            }

            1 + us +
                if draw(them) == us { 3 }
                else if win(them) == us { 6 }
                else { 0 }
        })
        .sum()
}

//------------------------------ PART 1

#[aoc(day2, part1)]
fn day2_part1(_input: &'static str) -> usize {
    let ans = solve(_input, 1);
    assert_eq!(ans, 12645);
    ans

}

#[test]
fn test_day2_part1() {
    assert_eq!(day2_part1(_SAMPLE), _ANS1);
}

//------------------------------ PART 2

#[aoc(day2, part2)]
fn day2_part2(_input: &'static str) -> usize {
    let ans = solve(_input, 2);
    assert_eq!(ans, 11756);
    ans
}

#[test]
fn test_day2_part2() {
    assert_eq!(day2_part2(_SAMPLE), _ANS2);
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "A Y
B X
C Z";

const _ANS1: usize = 15;
const _ANS2: usize = 12;