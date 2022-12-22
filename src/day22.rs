#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashMap;

//------------------------------ PARSE INPUT


type Grid = HashMap<Point, Cell>;
type Point = (usize, usize);

enum Cell {
    Floor,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Same,
}
use nom::{
    IResult, //error::Error,
    bytes::complete::{tag},
    character::complete::{alpha1, multispace0, digit1, one_of},
    branch::alt,
    combinator::opt,
};

fn parse_dist(input: &'static str) -> IResult<&'static str, usize> {
    let (i, _) = multispace0(input)?;
    let (i, arg1) = digit1(i)?;
    Ok((i, arg1.parse::<usize>().unwrap()))
}

fn parse_dir(input: &'static str) -> IResult<&'static str, Direction> {
    let (i, _) = multispace0(input)?;
    let (i, d) = opt(one_of("RL"))(i)?;
    let dir = match d {
        Some('R') => Direction::Right,
        Some('L') => Direction::Left,
        None => Direction::Same,
        _ => panic!(),
    };
    Ok((i, dir))
}

fn parse(input: &'static str) -> (Grid, Vec<(usize, Direction)>) {

    let mut map = Grid::new();
    for (row, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        for (col, ch) in line.chars().enumerate().filter(|(_, ch)| *ch != ' ') {
            map.insert((row, col),
                match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Floor,
                    _ => panic!(),
                });
        }
    }
    let mut i = input.lines().last().unwrap();
    let mut plan = Vec::new();
    while !i.is_empty() {
        let (ii,ds) = parse_dist(i).unwrap();
        let (ii,dr) = parse_dir(ii).unwrap();
        i = ii;
        plan.push((ds,dr));
    }
    (map, plan)
}

fn next(map: &Grid, pos: Point, dir: &Direction) -> Option<Point> {
    let (row, col) = pos;
    let maxrow = map.iter().filter(|((_,c),_)| *c == col).map(|((r,_),_)| *r).max().unwrap();
    let minrow = map.iter().filter(|((_,c),_)| *c == col).map(|((r,_),_)| *r).min().unwrap();
    let maxcol = map.iter().filter(|((r,_),_)| *r == row).map(|((_,c),_)| *c).max().unwrap();
    let mincol = map.iter().filter(|((r,_),_)| *r == row).map(|((_,c),_)| *c).min().unwrap();

    let next = match dir {
        Direction::Up => ( if row == minrow { maxrow } else {row-1}, col),
        Direction::Down => ( if row == maxrow { minrow } else {row+1}, col),
        Direction::Left => ( row, if col == mincol { maxcol } else {col-1}),
        Direction::Right => ( row, if col== maxcol { mincol } else {col+1}),
        Direction::Same => panic!(),
    };
    match map[&next] {
        Cell::Floor => Some(next),
        _ => None,
    }
}

fn next2(map: &Grid, pos: Point, dir: &Direction) -> Option<(Point, Direction)> {
    let width = if map.len() > 200 { 50 } else { 4 };

    let (row, col) = pos;
    let rface = row / width;
    let cface = col / width;

    let mut row = row;
    let mut col = col;
    let mut dir = dir;

    match dir {
        Direction::Up => {
            if row == 0 {
                // 1-2: Back to bottom
                col = width - 1 - (col % width);
                row = width;
                dir = &Direction::Down;
            } else if row == width * 2 && cface == 3{
                // 6-4: right to top
                row = width * 2 - 1 - (col % width);
                col = width * 3 - 1;
                dir = &Direction::Left;
            } else if row == width && cface == 1 {
                // 3-1: left to back
                row = col % width;
                col = width * 2;
                dir = &Direction::Right;
            } else if row == width && cface == 0 {
                // 2-1: bottom to back
                row = 0;
                col = width - col;
                dir = &Direction::Down;
            } else {
                    row -= 1;
            }
        },
        Direction::Down => {
            if row == width * 3 - 1 && cface == 2 {
                // 5-2: Front to bottom
                col = width - 1 - (col % width);
                row = width * 2 - 1;
                dir = &Direction::Up;
            } else if row == width*3 - 1 && cface == 3 {
                // 6-2: Right to bottom
                row = width * 2 - col % width - 1;
                col = 0;
                dir = &Direction::Left;
            } else if row == width*2 - 1 && cface == 1 {
                // 3-5: Left to front
                row = width * 3 - col % width - 1;
                col = width * 2;
                dir = &Direction::Right;
            } else if row == width*2 - 1 && cface == 0 {
                // 2-5: Bottom to front
                row = width * 3 - 1;
                col = width * 3 - col - 1;
                dir = &Direction::Up;
            } else {
                row += 1;
            }
        },
        Direction::Left => {
            if col == width * 2 && rface == 0 {
                // 1-3: Back to left
                col = width + row;
                row = width ;
                dir = &Direction::Up;
            } else if col == 0 {
                // 2-6 : Bottom to right
                col = width * 3 - 1 - row % width;
                row = width * 3 - 1;
                dir = &Direction::Up;
            } else if row == width*2 - 1 && cface == 1 {
                // 5-3: Front to left
                col = width * 2 - 1 - row % width;
                row = width * 2 - 1;
                dir = &Direction::Up;
            } else {
                col -= 1;
            }
        },
        Direction::Right => {
            if col == width * 3 - 1 && rface == 0 {
                // 1-6: Back to right
                col = width * 4 - 1;
                row = width * 3 - row;
                dir = &Direction::Left;
            } else if col == width * 3 - 1 && rface == 1 {
                // 4-6 : Top to right
                col = width * 4 - 1 - row % width;
                row = width * 2;
                dir = &Direction::Down;
            } else if row == width * 4 - 1 && cface == 2 {
                // 6-1: Right to back
                col = width * 3 - 1;
                row = width - 1 - row % width;
                dir = &Direction::Left;
            } else {
                col += 1;
            }

        }
        Direction::Same => panic!(),
    };
    let next = (row, col);

    if !map.contains_key(&next) {
        dbg!(pos);
        dbg!(dir);
        dbg!(next);
    }

    match map[&next] {
        Cell::Floor => Some((next, *dir)),
        _ => None,
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
        Direction::Same => panic!(),
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
        Direction::Same => panic!(),
    }
}

fn is_floor(cell: &Cell) -> bool {
    match cell {
        Cell::Floor => true,
        _ => false
    }
}

//------------------------------ SOLVE

fn solve1(input: &'static str) -> usize {
    let (map, plan) = parse(input);
    let dir = Direction::Up;

    let col = map.iter().filter(|((r,_),cell)| *r == 0 && is_floor(cell)).map(|((_,c),_)| *c).min().unwrap();
    let mut pos = (0, col);
    let mut dir = Direction::Right;

    // println!("{:?} {:?}", pos, dir);
    for (dist, turn) in plan {
        for _ in 0..dist {
            match next(&map, pos, &dir) {
                Some(p) => pos = p,
                None => {},
            };
        }
        dir = match turn {
            Direction::Right => turn_right(dir),
            Direction::Left => turn_left(dir),
            Direction::Same => dir,
            _ => panic!(),
        };
        // println!("{:?} {:?}", pos, dir);
    }

    score(&dir, &pos)
}

fn solve2(input: &'static str) -> usize {
    let (map, plan) = parse(input);

    let col = map.iter().filter(|((r,_),cell)| *r == 0 && is_floor(cell)).map(|((_,c),_)| *c).min().unwrap();
    let mut pos = (0, col);
    let mut dir = Direction::Right;

    // println!("{:?} {:?}", pos, dir);
    for (dist, turn) in plan {
        for _ in 0..dist {
            match next2(&map, pos, &dir) {
                Some((p, d)) => { pos = p; dir = d; },
                None => {},
            };
        }
        dir = match turn {
            Direction::Right => turn_right(dir),
            Direction::Left => turn_left(dir),
            Direction::Same => dir,
            _ => panic!(),
        };
        // println!("{:?} {:?}", pos, dir);
    }

    score(&dir, &pos)
}

fn score(dir: &Direction, pos: &Point) -> usize {
    let numdir = match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        Direction::Same => panic!(),
    };

    1004 + 1000 * pos.0 + 4 * pos.1 + numdir
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day22, part1)]
fn day22_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
#[aoc(day22, part2)]
fn day22_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ TESTS

#[test] fn test_day22_part1() { assert_eq!(solve1(_SAMPLE), 6032); }
#[test] fn test_day22_part2() { assert_eq!(solve2(_SAMPLE), 5031); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
const _ANS1: usize = 1;
const _ANS2: usize = 2;