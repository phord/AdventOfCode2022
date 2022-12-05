#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn transpose_char(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = grid[0].len();
    let mut new_grid = vec![Vec::new(); height];

    for line in grid.iter() {
        for (x, cell) in line.iter().enumerate() {
            new_grid[x].push(*cell);
        }
    }
    new_grid
}

fn parse(_input: &'static str) -> (Vec<Vec<char>>, Vec<Vec<&'static str>>) {
    let x: Vec<&str> = _input.split("\n\n").collect();
    let stacks = x[0];
    let commands = x[1];

    let towers: Vec<Vec<char>> = stacks.lines().rev().skip(1).map(|line|
            line.chars().enumerate().filter(|(c,_)| c%4 == 1)
            .map(|(_,b)| b )
            .collect()).collect();

    let towers: Vec<Vec<char>> = transpose_char(towers).iter().map(|x|
                x.iter().filter(|x| **x != ' ')
                .map(|x| *x).collect()).collect();

    let foo = commands
        .lines()
        .map(|s| s.split_whitespace()
                  .collect())
        .collect::<Vec<Vec<&str>>>();

    (towers, foo)
}

//------------------------------ SOLVE

fn solve(_input: &'static str, _part: usize) -> String {
    let (mut towers, cmds) = parse(_input);

    for cmd in cmds {
        let qty = cmd[1].parse::<usize>().unwrap();
        let src = cmd[3].parse::<usize>().unwrap()-1;
        let dest = cmd[5].parse::<usize>().unwrap()-1;

        let mut d = towers[dest].clone();
        if _part == 1 {
            for _ in 0..qty {
                d.push(towers[src].pop().unwrap());
            }
        } else {
            let p = towers[src].len() - qty;
            for ch in &towers[src][p..] {
                d.push(*ch);
            }
            towers[src] = towers[src][..p].to_vec();
        }
        towers[dest] = d;
    }

    // dump(towers);

    String::from_iter(towers.iter().map(|l| l.last().unwrap()))
}

//------------------------------ PART 1

#[allow(unused)]
// Uncomment next line when solution is ready
#[aoc(day5, part1)]
fn day5_part1(_input: &'static str) -> String {
    let ans = solve(_input, 1);
    assert_eq!(ans, "MQSHJMWNH");
    ans
}

#[test]
fn test_day5_part1() {
    assert_eq!(day5_part1(_SAMPLE), _ANS1);
}

//------------------------------ PART 2

#[allow(unused)]
// Uncomment next line when solution is ready
#[aoc(day5, part2)]
fn day5_part2(_input: &'static str) -> String {
    let ans = solve(_input, 2);
    assert_eq!(ans, "LLWJRBHVZ");
    ans
}

#[test]
fn test_day5_part2() {
    assert_eq!(day5_part2(_SAMPLE), _ANS2);
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

const _ANS1: &str = "CMZ";
const _ANS2: &str = "MCD";