#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(_input: &'static str) -> Vec<Vec<usize>> {
    split_to_byte_words(_input).iter().map(|row| row.iter().map(|x| (x[0] - b'0') as usize).collect()).collect()
}

//------------------------------ SOLVE

fn solve1(_input: &'static str, _part: usize) -> usize {
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
    found.iter().map(|row| row.iter().sum::<usize>()).sum()
}

fn solve2(_input: &'static str, _part: usize) -> usize {
    let mut _inp = parse(_input);
    let height = _inp.len();
    let width = _inp[0].len();

    let mut measured = vec![vec![0usize; 4]; width*height];
    let mut index = vec![vec![0; width]; height];

    let mut n = 0;
    for i in 0..height {
        for j in 0..width {
            index[i][j] = n;
            n += 1;
        }
    }

    for angle in 0..4 {
        for (y, row) in _inp.iter().enumerate() {
            let mut distance = vec![0; 10];
            for (x,c) in row.iter().enumerate() {
                let n = index[y][x];
                measured[n][angle] = distance[*c];
                // println!("{:?}", (x, y, *c, &distance, &measured[n]));
                for d in 0..10 {
                    distance[d] += 1;
                    if *c >= d {distance[d] = 1; }
                }
            }
        }
        _inp = rotate(_inp);
        index = rotate(index);
    }

    // dbg!(&measured);

    let measured: Vec<usize> = measured.iter().map(|x| x.iter().product()).collect();

    // dbg!(&measured);
    *measured.iter().max().unwrap()
}

//------------------------------ PART 1

#[allow(unused)]
#[aoc(day8, part1)]
fn day8_part1(_input: &'static str) -> usize {
    let ans = solve1(_input, 1);
    // assert_eq!(ans, ___);
    ans
}

#[test]
fn test_day8_part1() {
    assert_eq!(solve1(_SAMPLE, 1), _ANS1);
}

//------------------------------ PART 2

#[allow(unused)]
#[aoc(day8, part2)]
fn day8_part2(_input: &'static str) -> usize {
    let ans = solve2(_input, 2);
    // assert_eq!(ans, ___);
    ans
}

#[test]
fn test_day8_part2() {
    assert_eq!(solve2(_SAMPLE, 2), _ANS2);
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "30373
25512
65332
33549
35390";

const _ANS1: usize = 21;
const _ANS2: usize = 8;