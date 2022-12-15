#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use fnv::FnvHashSet;

//------------------------------ PARSE INPUT

type Point = (i32, i32);
// "Sensor at x=2, y=18"  -> (2,18)
fn split_xy(s: &'static str) -> Point {
    let s: Vec<&str> = s.split("=").collect();
    let p = s[1].split(", ").collect::<Vec<_>>();
    (p[0].parse::<i32>().unwrap(), s[2].parse::<i32>().unwrap())
}
fn parse(input: &'static str) -> Vec<Vec<Point>> {
    let s: Vec<Vec<&str>> = input.lines().map(|l| l.split(":").collect()).collect();
    let s: Vec<Vec<Point>> = s.iter()
        .map(|row| row.iter()
            .map(|pair| { split_xy(pair)}
            ).collect()
        ).collect();
    s
}

//------------------------------ SOLVE

fn mandist(a: Point, b:Point) -> i32 {
    let (ax,ay) = a;
    let (bx,by) = b;

    (ay-by).abs() + (ax-bx).abs()
}

fn coverage(s: Point, dist: i32, row: &i32) -> (i32, i32) {
    let (x,y) = s;
    let rdist = (y-row).abs();
    if rdist > dist {
        (0,0)
    } else {
        let d = dist - rdist;
        let start = x-d;
        let end = x+d;
        (start, end+1)
    }
}

fn row_coverage(map: &Vec<Vec<Point>>, row: i32) -> Vec<(i32, i32)> {

    let map:Vec<(Point, i32)> = map.iter().map(|row| {
        let (a,b) = (row[0], row[1]);
        (a, mandist(a,b))
    }).collect();


    let v = map.iter().map(|(s,dist)| coverage(*s, *dist, &row)).collect();
    v
}

fn solve1(input: &'static str, row: i32) -> usize {
    let map = parse(input);

    let beacons:Vec<Point> = map.iter().map(|row| row[1]).collect();
    let v = row_coverage(&map, row);

    let mut set = FnvHashSet::default();
    for (start, end) in v {
        for x in start..end {
            set.insert(x);
        }
    }

    for b in beacons.iter().filter(|(_,y)| *y == row) {
        let (x, _) = b;
        set.remove(x);
    }

    set.len()
}

fn solve2(input: &'static str, range: i32) -> u64 {
    let map = parse(input);

    for row in 0..=range {
        let mut v = row_coverage(&map, row);
        v.sort();

        // println!("{}. {:?}", row, v);
        let mut end = 0;
        for (a,b) in v {
            if a <= end && b > end {
                end = b;
                if end > range {
                    break;
                }
            } else if a > end {
                assert_eq!(a-1, end);
                println!("{:?}", (end, row));
                return 4000000u64 * (end as u64) + row as u64;
            }
        }
    }
    panic!("Not found");
}

// fn solve1(input: &'static str) -> usize { solve(input, 1) }
// fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day15, part1)]
fn day15_part1(input: &'static str) -> usize {
    let ans = solve1(input, 2000000i32);
    assert_eq!(ans, 4873353);
    ans
}

#[allow(unused)]
#[aoc(day15, part2)]
fn day15_part2(input: &'static str) -> u64 {
    let ans = solve2(input, 4000000);
    assert_eq!(ans, 11600823139120u64);
    ans
}

//------------------------------ TESTS

#[test] fn test_day15_part1() { assert_eq!(solve1(_SAMPLE, 10), 26); }
#[test] fn test_day15_part2() { assert_eq!(solve2(_SAMPLE, 20), 56000011); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

const _ANS1: usize = 1;
const _ANS2: usize = 2;