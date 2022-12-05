#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(_input: &'static str) -> Vec<(u64,u64,u64,u64)> {
    _input.lines().map(|x| {
        let p: Vec<&str> = x.split(',').collect();
        let x: Vec<&str> = p[0].split('-').collect();
        let y: Vec<&str> = p[1].split('-').collect();

        (parse_u64(x[0].as_bytes()),parse_u64(x[1].as_bytes()),parse_u64(y[0].as_bytes()),parse_u64(y[1].as_bytes()),)
    }).collect()
}

//------------------------------ SOLVE

fn solve(_input: &'static str, _part: usize) -> usize {
    let _inp = parse(_input);
    println!("{:?}", _inp);

    let mut count = 0;
    for (a,b,x,y) in _inp {
        if (a >= x && b<=y )
         || (a <= x && b>=y ) {
            count += 1;
            println!("{:?}", (a,b,x,y));
         }
    }
    count
}

fn solve2(_input: &'static str, _part: usize) -> usize {
    let _inp = parse(_input);

    let mut count = 0;

    for (a,b,x,y) in _inp {
        if (a <= x && b>=x )
         || (a <= y && b>=y )
        || (x <= a && y>=b ) {
             count += 1;
             println!("{}. {:?}", count, (a,b,x,y));
         }
    }
    count
}

//------------------------------ PART 1

#[allow(unused)]
#[aoc(day4, part1)]
fn day4_part1(_input: &'static str) -> usize {
    let ans = solve(_input, 1);
    // assert_eq!(ans, _);
    ans
}

#[test]
fn test_day4_part1() {
    assert_eq!(day4_part1(_SAMPLE), _ANS1);
}

//------------------------------ PART 2

#[allow(unused)]
#[aoc(day4, part2)]
fn day4_part2(_input: &'static str) -> usize {
    let ans = solve2(_input, 2);
    // assert_eq!(ans, ___);
    ans
}

#[test]
fn test_day4_part2() {
    assert_eq!(day4_part2(_SAMPLE), _ANS2);
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
4-8,2-16";

const _ANS1: usize = 2;
const _ANS2: usize = 4;