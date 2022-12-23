#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashMap;
use colored::*;

//------------------------------ PARSE INPUT


type Grid = HashMap<Point, Cell>;
type Point = (i32, i32);

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
    character::complete::{multispace0, digit1, one_of},
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
            map.insert((row as i32, col as i32),
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
        Direction::Up => row -= 1,
        Direction::Down => row += 1,
        Direction::Left => col -= 1,
        Direction::Right => col += 1,
        Direction::Same => panic!(),
    };
    let next = (row, col);

    if !map.contains_key(&next) {
        // We've walked off an edge. Which edge is it and how do we fix this?
        // dbg!(pos);
        // dbg!(dir);
        // dbg!(next);

        // Shape of my input:
        //
        //           6  6
        //           ^  ^
        //           |  |
        //         22221111
        //         22221111
        //    5 <- 22221111 -> 4
        //         22221111
        //         3333  ^
        //    5 +- 3333  |
        //      |  3333 -+ 1
        //      v  3333
        //     55554444
        //2 -> 55554444 -> 1
        //     55554444
        //     55554444
        //     6666  ^
        //2 <- 6666  |
        //     6666 -+ 4
        //     6666
        //      |
        //      v
        //      1
        //
        // To walk around a cube map, when we enter a "gap" we need to
        // rotate 90 degrees about the quadrant corner.  For example,
        // when heading Down on face 1, we walk onto face 3 and turn_right
        // to go Left.  Similarly when we walk Up from F5, we turn_right
        // to head right on F3.  But walking Left off of F2 requires two
        // left-turns (to head Right) on F5.  Why left turns and how do we
        // decide to rotate twice to reach our common edge on F5?
        // I suppose I can map it all by hand.  Blast!

        match dir {
            Direction::Same => panic!(),
            Direction::Up => {
                if cface == 0 {
                    // 5->3
                    println!("  5->3");
                    row = width + col;
                    col = width;
                    dir = &Direction::Right;
                } else if cface == 1 {
                    // 2->6
                    println!("  2->6");
                    row = width * 3 + col % width;
                    col = 0;
                    dir = &Direction::Right;
                } else if cface == 2 {
                    // 1->6
                    println!("  1->6");
                    col = width - col % width - 1;
                    row = width * 4 - 1;
                    dir = &Direction::Up;
                } else {
                    panic!();
                }
            },
            Direction::Down => {
                if cface == 0 {
                    // 6->1
                    println!("  6->1");
                    col = width * 3 - col % width - 1;
                    row = 0;
                    dir = &Direction::Down;
                } else if cface == 1 {
                    // 4->6
                    println!("  4->6   {}, {}", row, col);
                    row = width * 3 + col % width;
                    col = width - 1;
                    dir = &Direction::Left;
                } else if cface == 2 {
                    // 1->3
                    println!("  1->3");
                    row = width + col % width;
                    col = width * 2 - 1;
                    dir = &Direction::Left;
                } else {
                    panic!();
                }
            },
            Direction::Right => {
                if rface == 0 {
                    // 1->4
                    println!("  1->4");
                    row = width * 3 - row % width - 1;
                    col = width * 2 - 1;
                    dir = &Direction::Left;
                } else if rface == 1 {
                    // 3->1
                    println!("  3->1");
                    col = width * 2 + row % width;
                    row = width - 1;
                    dir = &Direction::Up;
                } else if rface == 2 {
                    // 4->1
                    println!("  4->1");
                    col = width *3 - 1;
                    row = width - row % width - 1;
                    dir = &Direction::Left;
                } else if rface == 3 {
                    // 6->4
                    println!("  6->4");
                    col = width + row % width;
                    row = width * 3 - 1;
                    dir = &Direction::Up;
                } else {
                    panic!();
                }
            },
            Direction::Left => {
                if rface == 0 {
                    // 2->5
                    println!("  2->5");
                    col = 0;
                    row = width * 3 - row % width - 1;
                    dir = &Direction::Right;
                } else if rface == 1 {
                    // 3->5
                    println!("  3->5");
                    col = row % width;
                    row = width * 2;
                    dir = &Direction::Up;
                } else if rface == 2 {
                    // 5->2
                    println!("  5->2");
                    col = width;
                    row = width - row % width;
                    dir = &Direction::Right;
                } else if rface == 3 {
                    // 6->2
                    println!("  6->2");
                    col = width + row % width;
                    row = 0;
                    dir = &Direction::Up;
                } else {
                    panic!();
                }
            },
        }
    }

    let next = (row, col);
    if !map.contains_key(&next) {
        // We've walked off an edge. Which edge is it and how do we fix this?
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

fn solve1(input: &'static str) -> i32 {
    let (map, plan) = parse(input);

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

fn solve2(input: &'static str) -> i32 {
    let (map, plan) = parse(input);

    let col = map.iter().filter(|((r,_),cell)| *r == 0 && is_floor(cell)).map(|((_,c),_)| *c).min().unwrap();
    let mut pos = (0, col);
    let mut dir = Direction::Right;

    // display(&map, &pos, &dir);

    // println!("{:?} {:?}", pos, dir);
    for (dist, turn) in plan {
        for _ in 0..dist {
            match next2(&map, pos, &dir) {
                Some((p, d)) => {
                    pos = p; dir = d;
                    display_cube_face(&map, &pos, &dir, get_face(&pos));
                },
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

fn score(dir: &Direction, pos: &Point) -> i32 {
    let numdir = match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        Direction::Same => panic!(),
    };

    1004 + 1000 * pos.0 + 4 * pos.1 + numdir
}

fn display_small(map: &Grid, pos: &Point, dir: &Direction) {
    let maxrow = map.iter().map(|((r,_),_)| *r).max().unwrap();
    let minrow = map.iter().map(|((r,_),_)| *r).min().unwrap();
    let maxcol = map.iter().map(|((_,c),_)| *c).max().unwrap();
    let mincol = map.iter().map(|((_,c),_)| *c).min().unwrap();

    println!();

    let nearby = |p1: &Point, p2: &Point| -> bool { let (y1,x1) = p1; let (y2,x2) = p2; (y1-y2).abs() < 3 && (x1-x2).abs() < 3 };
    for row in (minrow..=maxrow).filter(|x| x%5 == 0) {
        for col in (mincol..=maxcol).filter(|x| x%5 == 0) {
            let cur = (row, col);
            if nearby(pos, &cur) {
                match dir {
                        Direction::Right => print!(">"),
                        Direction::Down => print!("V"),
                        Direction::Left => print!("<"),
                        Direction::Up => print!("^"),
                        Direction::Same => panic!(),
                };
            } else if map.iter().any(|(p,_)| nearby(p, &(row,col))) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn display(map: &Grid, pos: &Point, dir: &Direction) {
    let maxrow = map.iter().map(|((r,_),_)| *r).max().unwrap() - 3;
    let minrow = map.iter().map(|((r,_),_)| *r).min().unwrap();
    let maxcol = map.iter().map(|((_,c),_)| *c).max().unwrap();
    let mincol = map.iter().map(|((_,c),_)| *c).min().unwrap();

    println!();

    for row in (minrow..=maxrow) {
        for col in (mincol..=maxcol) {
            let cur = (row, col);
            if cur == *pos {
                let car = match dir {
                        Direction::Right => ">",
                        Direction::Down => "V",
                        Direction::Left => "<",
                        Direction::Up => "^",
                        Direction::Same => panic!(),
                };
                print!("{}", car.bright_yellow());
            } else if map.contains_key(&cur) {
                match map[&cur] {
                    Cell::Floor => print!("{}", ".".green()),
                    Cell::Wall => print!("{}", "#".red()),
                }
            } else {
                print!(" ");
            }
            print!(" ");
        }
        if row == 0 {
            print!("{}, {}", pos.0, pos.1);
        }
        println!();
    }
}

fn get_face(pos: &Point) -> usize {
    let width = 50;
    let (mut row, mut col) = pos;
    row /= width;
    col /= width;

    match (row, col) {
         (0, 2) => 1,
         (0, 1) => 2,
         (1, 1) => 3,
         (2, 1) => 4,
         (2, 0) => 5,
         (3, 0) => 6,
        _ => panic!(),
    }
}

fn get_cell(map: &Grid, pos: &Point, dir: &Direction, cur: &Point) -> ColoredString {
    if *cur == *pos {
        let car = match dir {
                Direction::Right => ">",
                Direction::Down => "V",
                Direction::Left => "<",
                Direction::Up => "^",
                Direction::Same => panic!(),
        };
        car.bright_yellow()
    } else if map.contains_key(&cur) {
        match map[&cur] {
            Cell::Floor => ".".green(),
            Cell::Wall => "#".red(),
        }
    } else {
        " ".black()
    }

}
fn display_cube_face(map: &Grid, pos: &Point, dir: &Direction, face: usize) {
    let maxrow = map.iter().map(|((r,_),_)| *r).max().unwrap() - 3;
    let minrow = map.iter().map(|((r,_),_)| *r).min().unwrap();
    let maxcol = map.iter().map(|((_,c),_)| *c).max().unwrap();
    let mincol = map.iter().map(|((_,c),_)| *c).min().unwrap();

    let width = 50;

    let start = match face {
        1 => (0, 2),
        2 => (0, 1),
        3 => (1, 1),
        4 => (2, 1),
        5 => (2, 0),
        6 => (3, 0),
        _ => panic!(),
    };

    let minrow = start.0 * width;
    let mincol = start.1 * width;
    let maxrow = minrow + width - 1;
    let maxcol = mincol + width - 1;
    println!();

    let side:Vec<Vec<ColoredString>> = (minrow..=maxrow).map(|row|
        (mincol..=maxcol).map(|col| {
            let cur = (row, col);
            get_cell(map, pos, dir, &cur)
        }).collect::<Vec<_>>()
    ).collect();

    for row in (minrow..=maxrow) {
        println!();
        for _ in 0..row-minrow {
            print!(" ");
        }
        for col in (mincol..=maxcol) {
            let cur = (row, col);
            let c = get_cell(map, pos, dir, &cur);
            print!("{}{}", c, c);
        }
    }

    println!();

    for row in (minrow..=maxrow) {
        for _ in 0..1 {
            println!();
            print!("                                                  ");
            for col in (mincol..=maxcol) {
                let cur = (row, col);
                let c = get_cell(map, pos, dir, &cur);
                print!("{}{}", c, c);
            }
        }
        if row == 0 {
            print!("{}, {}", pos.0, pos.1);
        }
    }
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day22, part1)]
fn day22_part1(input: &'static str) -> i32 {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
#[aoc(day22, part2)]
fn day22_part2(input: &'static str) -> i32 {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    // 13280 is wrong (too low)
    // 121335 is too low
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
const _ANS1: i32 = 1;
const _ANS2: i32 = 2;