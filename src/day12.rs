use std::collections::HashSet;

#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<&[u8]> {
    input.lines()
        .map(|x| x.as_bytes()).collect()
}

//------------------------------ SOLVE

fn height(grid: &Vec<&[u8]>, pos: (usize, usize)) -> u8 {
    let (x, y) = pos;
    let c = grid[y][x];
    match c {
        b'E' => b'z' + 1,
        b'S' => b'a' - 1,
        _ => c,
    }
}

fn finish(grid: &Vec<&[u8]>, pos: (usize, usize)) -> bool {
    let (x, y) = pos;
    b'E' == grid[y][x]
}

fn move_to(grid: &Vec<&[u8]>, pos: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
    let (x, y) = pos;
    let (dx, dy) = dir;
    let h = grid.len();
    let w = grid[0].len();


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
        let c = height(grid, pos);
        let d = height(grid, new);
        if d > c + 1 {
            None
        } else {
            Some(new)
        }
    }
}

#[test] fn test_day12_part1() { assert_eq!(solve1(_SAMPLE), _ANS1); }


fn nav(grid: &Vec<&[u8]>, pos: (usize, usize), visited: HashSet<(usize, usize)>) -> Option<usize> {

    if finish(grid, pos) {
        // println!("================= FINISH {} ==================", visited.len());
        Some(visited.len())
    } else {
        let distance:Vec<Option<usize>> = [(-1,0), (1,0), (0,-1), (0,1)].iter().map(|dir| {
            // println!("Distance: {:?}", dir);
            match move_to(grid, pos, *dir) {
                Some(new) => {
                    if !visited.contains(&new) {
                        // println!("Pos {:?} Depth: {}  Visit: {:?}", &pos, visited.len(), &new);
                        let mut visited = visited.clone();  // Seems expensive
                        visited.insert(pos);
                        nav(grid, new, visited)
                    } else {None}
                },
                None => None,
            }
        }).collect();

        // dbg!(&distance);

        let distance: Vec<usize> = distance.iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();

        distance.iter().cloned().min()
    }
}


fn solve(input: &'static str, _part: usize) -> usize {
    let grid = parse(input);

    let mut pos = (0usize,0usize);

    for (y,row) in grid.iter().enumerate() {
        for (x,col) in row.iter().enumerate() {
            if *col == b'S' {
                pos = (x,y)
            }
        }
    }

    println!("Start: {:?}", &pos);

    if let Some(dist) = nav(&grid, pos, HashSet::new()) {
        dist
    } else {
        panic!("No solution found");
    }
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day12, part1)]
fn day12_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
// Uncomment next line when solution is ready
// #[aoc(day12, part2)]
fn day12_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ TESTS

#[test] fn test_day12_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

const _ANS1: usize = 31;
const _ANS2: usize = 2;