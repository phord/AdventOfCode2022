#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashSet;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<(&str, usize)> {
    input.lines()
        .map(|line| {
            let words: Vec<&str> = line.split(" ").collect();
            (words[0], words[1].parse::<usize>().unwrap())} ).collect()
}

//------------------------------ SOLVE
fn sgn(x: i32) -> i32 {
    if x < 0 { -1} else if x > 0 { 1} else {0}
}

fn follow(y: i32, x: i32, ty: &mut i32, tx: &mut i32, ) {

    assert!((x-*tx).abs() <= 2);
    assert!((y-*ty).abs() <= 2);

    if (x-*tx).abs() == 2 || (y-*ty).abs() == 2 {
        // Not touching - must move

        // If the head is ever two steps directly up, down, left, or right from the tail,
        // the tail must also move one step in that direction so it remains close enough:
        //   Move one step towards head on one axis

        // Otherwise, if the head and tail aren't touching and aren't in the same row or column,
        // the tail always moves one step diagonally to keep up:
        //   Move one step towards head on both axes

        *tx += sgn(x-*tx);
        *ty += sgn(y-*ty);
    }
}

fn solve(input: &'static str, part: usize) -> usize {
    let inp = parse(input);

    let len = if part == 1 { 2 } else { 10 };

    let mut x = vec![0; len];
    let mut y = vec![0; len];
    let mut visited = HashSet::new();

    for (dir, dist) in inp {
        // println!("{:?}", (dir, dist));
        for _ in 0..dist {
            match dir {
                "U" => y[0] -= 1,
                "D" => y[0] += 1,
                "L" => x[0] -= 1,
                "R" => x[0] += 1,
                _ => panic!("Bad direction: {}", dir),
            };

            for i in 1..len {
                follow(y[i-1], x[i-1], &mut y[i], &mut x[i]);
                // println!("\n [[{}]]", i);
                // print(&x, &y);
            }
            // println!("Head: {:?}  Tail: {:?}", (y, x), (ty, tx));
            visited.insert((y[len-1],x[len-1]));
            // print(&x, &y);
            // println!();
        }
    }
    visited.len()
}

fn print(vx: &Vec<i32>, vy: &Vec<i32>) {
    let dy = -4..1;
    // let dy = -15..6;
    for y in dy {
        let dx = 0..7;
        // let dx = -15..6;
        for x in dx {
            let mut code = ".";
            for i in 0..vx.len() {
                if (x,y) == (vx[i], vy[i]) {
                    code = "";
                    match i {
                        0 => print!("{}", "H"),
                        n => print!("{}", n),
                    };
                    break;
                }
            }
            print!("{}", code);
        }
        println!();
    }
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day9, part1)]
fn day9_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    assert_eq!(ans, 6406);
    ans
}

#[allow(unused)]
#[aoc(day9, part2)]
fn day9_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    assert_eq!(ans, 2643);
    ans
}

//------------------------------ TESTS

#[test] fn test_day9_part1() { assert_eq!(solve1(_SAMPLE), _ANS1); }
#[test] fn test_day9_part2() { assert_eq!(solve2(_SAMPLE), 1); }
#[test] fn test_day9_part2b() { assert_eq!(solve2(_SAMPLE2), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

const _SAMPLE2: &str ="R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
const _ANS1: usize = 13;
const _ANS2: usize = 36;