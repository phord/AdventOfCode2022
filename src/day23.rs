#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashSet;

//------------------------------ PARSE INPUT

type Point = (i32, i32);
fn parse(input: &'static str) -> HashSet<Point> {
    let mut foo: HashSet<Point> = HashSet::default();
    for (row, l) in  input.lines().enumerate() {
        for (col, ch) in l.chars().enumerate() {
            if ch == '#' {
                foo.insert((row as i32, col as i32));
            }
        }
    }

    foo
}

//------------------------------ SOLVE

fn adj( map: &HashSet<Point>, pos: &Point, targets: &Vec<Point>) -> bool {
    targets.iter().any(|ofs| {
        let p = (pos.0 + ofs.0, pos.1 + ofs.1);
        map.contains(&p)})
}

fn print(map: &HashSet<Point>) {
    let maxrow = map.iter().map(|(r,_)| *r).max().unwrap();
    let minrow = map.iter().map(|(r,_)| *r).min().unwrap();
    let maxcol = map.iter().map(|(_,c)| *c).max().unwrap();
    let mincol = map.iter().map(|(_,c)| *c).min().unwrap();
    println!("----");

    for row in minrow..=maxrow {
        for col in mincol..=maxcol {
            if map.contains(&(row,col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn solve(input: &'static str, part: usize) -> (i32, i32) {
    let neighbors = vec![
        (-1,-1), (-1, 0), (-1, 1),
        (0,-1),          (0, 1),
        (1,-1), (1, 0), (1, 1),
    ];

    let north = vec![(-1,-1), (-1, 0), (-1, 1),];
    let south = vec![(1,-1), (1, 0), (1, 1),];
    let west = vec![(-1,-1), (0, -1), (1, -1),];
    let east = vec![(-1,1), (0, 1), (1, 1),];

    let mut dirs = vec![north, south, west, east];
    let mut map = parse(input);

    // print(&map);

    let mut size = 0;
    let mut rounds = 0;

    for round in 1.. {
        // Step 1: Decide where to move
        let proposed:Vec<(Point, Point)> = map.iter()
            .filter(|pos| {let n = adj(&map, &pos, &neighbors);
            // if !n { println!("{:?} has no neighbors", *pos);}
            n})
            .map(|pos| {
                let foo = dirs.iter()
                        .filter(|d| {let foo = !adj(&map, &pos, d);
                            // if !foo {println!("{:?} can't move to {:?}", pos, d);}
                            foo})
                        .map(|d| { let d = d[1];
                                // println!("{:?} ==> {:?}", pos, d);
                                 (pos.clone(), (pos.0+d.0, pos.1+d.1))})
                        .collect::<Vec<(Point, Point)>>();
                if foo.is_empty() { //println!("{:?} has no place to go", pos);
                                    None } else {
                    let (pos, d) = foo[0]; //println!("{:?} decides to move {:?}", pos, d);
                    Some(foo[0])}
                        })
                        .flatten()
            .collect();

        let mut ndirs:Vec<Vec<Point>> = dirs[1..].iter().map(|x| x.clone()).collect();
        ndirs.push(dirs[0].clone());
        dirs = ndirs.clone();

        // Step 2: Move
        let mut moved = false;
        for (old, new) in &proposed {
            if !proposed.iter().any(|(me, tgt)| *me != *old && *tgt == *new) {
                map.remove(&old);
                map.insert(*new);
                moved = true;
            }
        }
        if !moved && rounds == 0 {
            rounds = round;
        }
        // print(&map);

        if round == 10 {
            let maxrow = map.iter().map(|(r,_)| *r).max().unwrap();
            let minrow = map.iter().map(|(r,_)| *r).min().unwrap();
            let maxcol = map.iter().map(|(_,c)| *c).max().unwrap();
            let mincol = map.iter().map(|(_,c)| *c).min().unwrap();

            size = (maxrow - minrow + 1) * (maxcol - mincol + 1) - map.len() as i32;
        }
        if size > 0 && rounds > 0 {
            break;
        }
    }
    (size, rounds)

}

fn solve1(input: &'static str) -> i32 { solve(input, 1).0 }
fn solve2(input: &'static str) -> i32 { solve(input, 2).1 }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day23, part1)]
fn day23_part1(input: &'static str) -> i32 {
    let ans = solve1(input);
    assert_eq!(ans, 3762);
    ans
}

#[allow(unused)]
#[aoc(day23, part2)]
fn day23_part2(input: &'static str) -> i32 {
    let ans = solve2(input);
    assert_eq!(ans, 997);
    ans
}

//------------------------------ TESTS

// #[test] fn test_day23_part1a() { assert_eq!(solve1(_SAMPLE2), 110); }
#[test] fn test_day23_part1() { assert_eq!(solve1(_SAMPLE), 110); }
#[test] fn test_day23_part2() { assert_eq!(solve2(_SAMPLE), 20); }

//------------------------------ SAMPLE DATA

const _SAMPLE2: &str = ".....
..##.
..#..
.....
..##.
.....";
const _SAMPLE: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

const _ANS1: i32 = 1;
const _ANS2: i32 = 2;