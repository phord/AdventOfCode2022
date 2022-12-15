#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use fnv::FnvHashSet;

type Point = (i32, i32);
type Grid = FnvHashSet<Point>;

fn str_sample() -> &'static str {
    "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
}

fn pi32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn parse_str(s: &'static str) -> Vec<Vec<Point>> {
    let s: Vec<Vec<&str>> = s.lines().map(|l| l.split(" -> ").collect()).collect();
    let s: Vec<Vec<Point>> = s.iter()
        .map(|row| row.iter()
            .map(|pair| { let (x, y) = pair.split_once(',').unwrap(); (pi32(x), pi32(y))}
            ).collect()
        ).collect();
    s
}

#[allow(unused)]
fn sample() -> Vec<Vec<(i32, i32)>> {
    parse_str(str_sample())
}

fn parse(sample: &Vec<Vec<(i32, i32)>>) -> Grid {

    let mut g = Grid::default();
    for line in sample {
        let mut start = line[0];
        for next in line {
            let (x,y) = start;
            let (nx,ny) = *next;
            if x == nx {
                // produce y cells
                let range = if y > ny { ny..=y } else { y..=ny};
                for y in range { g.insert((x,y)); }
            } else {
                // produce x cells
                let range = if x > nx { nx..=x } else { x..=nx};
                for x in range { g.insert((x,y)); }
            }
            start = *next;
        }
    }
    g
}

//------------------------------ SOLVE
fn fall(g: &mut Grid, max: &i32, path: &mut Vec<Point>, part: usize) -> bool {
    let (mut x, mut y) = path.pop().unwrap_or((500,0));
    if g.contains(&(x,y)) {
        return false;
    }

    loop {
        if y > *max {
            if part == 1 {          // part 1: Fall into the abyss
                return false;
            }
            break;                  // part 2: hit the infinitely wide floor
        }

        let prev = (x,y);
        match [ 0, -1, 1 ].iter().find(|dx| !g.contains(&(x + *dx,y + 1))) {
            None => break,
            Some(dx) => x += dx,
        }
        y += 1;
        path.push(prev);
    }
    g.insert((x,y));
    return true;
}

fn solve(g:Grid, part: usize) -> usize {

    let max = g.iter().map(|(_,b)| b).max().unwrap();
    let mut g = g.clone();

    let mut path = vec![(500, 0)];
    for count in 0.. {
        if ! fall(&mut g, max, &mut path, part) {
            return count;
        }
    }
    part
}

fn solve1(g:Grid) -> usize { solve(g, 1) }
fn solve2(g:Grid) -> usize { solve(g, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day14, part1)]
fn day14_part1(input: &'static str) -> usize {
    let ans = solve1(parse(&parse_str(input)));
    assert_eq!(ans, 692);
    ans
}

#[allow(unused)]
#[aoc(day14, part2)]
fn day14_part2(input: &'static str) -> usize {
    let ans = solve2(parse(&parse_str(input)));
    assert_eq!(ans, 31706);
    ans
}

//------------------------------ TESTS

#[test] fn test_day14_part1() { assert_eq!(solve1(parse(&sample())), 24); }
#[test] fn test_day14_part2() { assert_eq!(solve2(parse(&sample())), 93); }
