#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(_input: &'static str) -> Vec<Vec<usize>> {
    split_to_byte_words(_input).iter().map(|row| row.iter().map(|x| (x[0] - b'0') as usize).collect()).collect()
}

//------------------------------ SOLVE

fn solve1(_input: &'static str) -> usize {
    let mut _inp = parse(_input);
    let height = _inp.len();
    let width = _inp[0].len();

    let mut found = vec![vec![0; width]; height];

    for _angle in 0..4 {
        found = rotate(found);

        for (y, row) in _inp.iter().enumerate() {
            let mut max = 0;
            for (x,c) in row.iter().enumerate() {
                if *c >= max {
                    max = *c + 1;
                    found[y][x] = 1;
                }
            }
        }
        _inp = rotate(_inp);
    }
    found.iter().flatten().sum()
}

fn solve2(_input: &'static str) -> usize {
    let mut grid = parse(_input);
    let height = grid.len();
    let width = grid[0].len();

    let mut measured = vec![vec![vec![0usize; 4]; width]; height];

    for angle in 0..4 {
        for (y, row) in grid.iter().enumerate() {
            let mut distance = vec![0; 10];
            for (x, &height) in row.iter().enumerate() {
                measured[y][x][angle] = distance[height];
                for d in height+1..10 {
                    distance[d] += 1;
                }
                for d in 0..height+1 {
                    distance[d] = 1;
                }
            }
        }
        grid = rotate(grid);
        measured = rotate(measured);
    }

    measured.iter().flatten().map(|x| x.iter().product())
        .max().unwrap()
}

//------------------------------ PART 1

#[allow(unused)]
#[aoc(day8, part1)]
fn day8_part1(_input: &'static str) -> usize {
    let ans = solve1(_input);
    assert_eq!(ans, 1698);
    ans
}

#[test]
fn test_day8_part1() {
    assert_eq!(solve1(_SAMPLE), _ANS1);
}

//------------------------------ PART 2

#[allow(unused)]
#[aoc(day8, part2)]
fn day8_part2(_input: &'static str) -> usize {
    let ans = solve2(_input);
    assert_eq!(ans, 672280);
    ans
}

#[test]
fn test_day8_part2() {
    assert_eq!(solve2(_SAMPLE), _ANS2);
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "30373
25512
65332
33549
35390";

const _ANS1: usize = 21;
const _ANS2: usize = 8;