#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(_input: &'static str) -> Vec<Vec<&[u8]>> {
    split_to_words(_input)
}

//------------------------------ SOLVE
//
//
/*
1 A X Rock
2 B Y Paper
3 C Z Scissors
A > Z
B > X
C > Y
 */

fn solve(_input: &'static str, part: usize) -> usize {
    let _inp = parse(_input);

    let mut score = 0;
    for game in _inp {
        let them = 1 + (game[0][0] - b'A') as usize;
        let mut us = 1 + (game[1][0] - b'X') as usize ;

        if part == 2 {
            if us == 2 { // draw
                us = them;  // draw
            } else if us == 1 { // lose
                us = if them == 1 { 3 } else if them == 2 { 1 } else { 2 };
            } else {   // win
                us = if them == 1 { 2 } else if them == 2 { 3 } else { 1 };
            }
        }

        score += us;
        if  them == us {
            score += 3;
        } else if
            (them == 1 && us == 3) ||
            (them == 2 && us == 1) ||
            (them == 3 && us == 2) {
                // score += 0;
            }
        else {
            score += 6;
        }
        println!("{} {} {}", them, us, score);
    }

    score
}

//------------------------------ PART 1

#[allow(unused)]
// Uncomment next line when solution is ready
#[aoc(day2, part1)]
fn day2_part1(_input: &'static str) -> usize {
    solve(_input, 1)
}

#[test]
fn test_day2_part1() {
    assert_eq!(day2_part1(_SAMPLE), _ANS1);
}

//------------------------------ PART 2

#[allow(unused)]
// Uncomment next line when solution is ready
#[aoc(day2, part2)]
fn day2_part2(_input: &'static str) -> usize {
    solve(_input, 2)
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