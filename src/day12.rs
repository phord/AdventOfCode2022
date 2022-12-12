use std::collections::HashMap;

#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<&[u8]> {
    input
        .lines()
        .map(|x| x.as_bytes())
        .collect()
}

//------------------------------ SOLVE

fn elevation(grid: &Vec<&[u8]>, pos: (usize, usize)) -> u8 {
    let (x, y) = pos;
    let c = grid[y][x];
    match c {
        b'E' => b'z',
        b'S' => b'a',
        _ => c,
    }
}

fn finish(grid: &Vec<&[u8]>, pos: (usize, usize)) -> bool {
    let (x, y) = pos;
    b'S' == grid[y][x]
}

fn move_to(grid: &Vec<&[u8]>, pos: (usize, usize), dir: &Dir) -> Option<(usize, usize)> {
    let (x, y) = pos;
    let h = grid.len();
    let w = grid[0].len();

    let (dx, dy) = match dir {
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
        Dir::Up => (0, -1),
        Dir::Down => (0, 1),
    };

    if x == 0 && dx == -1 {
        None
    } else if x == w-1 && dx == 1 {
        None
    } else if y == 0 && dy == -1 {
        None
    } else if y == h-1 && dy == 1 {
        None
    } else {
        let new = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
        let d = elevation(grid, pos);
        let c = elevation(grid, new);
        if d > c + 1 {
            None
        } else {
            Some(new)
        }
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn better_path(pos: (usize, usize), depth: usize, visited: &mut HashMap<(usize, usize), usize>) -> bool {
    if let Some(&d) = visited.get(&pos) {
        if d <= depth {
            return false;
        }
    }
    visited.insert(pos, depth);
    true
}

fn nav(grid: &Vec<&[u8]>, pos: (usize, usize), depth: usize, visited: &mut HashMap<(usize, usize), usize>) -> Option<usize> {
    if !better_path(pos, depth, visited) {
        None
    } else if finish(grid, pos) {
        Some(depth)
    } else {
        let distance:Vec<Option<usize>> = [Dir::Up, Dir::Down, Dir::Left, Dir::Right].iter().map(|dir| {
            match move_to(grid, pos, dir) {
                Some(new) => {
                    nav(grid, new, depth+1, visited)
                },
                None => None,
            }
        }).collect();

        let distance: Vec<usize> = distance
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        distance.iter().cloned().min()
    }
}


fn solve(input: &'static str, part: usize) -> usize {
    let grid = parse(input);

    let mut pos = (0usize,0usize);

    for (y,row) in grid.iter().enumerate() {
        for (x,col) in row.iter().enumerate() {
            if *col == b'E' {
                pos = (x,y)
            }
        }
    }

    let mut visited = HashMap::new();
    nav(&grid, pos, 0, &mut visited);

    let search = if part == 2 { |x| x==b'S' || x==b'a' } else { |x| x==b'S' };
    let it = visited.iter()
        .filter(|((x,y), _)| search(grid[*y][*x]))
        .map(|(_, dist)| dist)
        .min()
        .unwrap();
    *it
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day12, part1)]
fn day12_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    assert_eq!(ans, 339);
    ans
}

#[allow(unused)]
#[aoc(day12, part2)]
fn day12_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    assert_eq!(ans, 332);
    ans
}

//------------------------------ TESTS

#[test] fn test_day12_part1() { assert_eq!(solve1(_SAMPLE), _ANS1); }
#[test] fn test_day12_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

const _ANS1: usize = 31;
const _ANS2: usize = 29;