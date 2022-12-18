#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use fnv::FnvHashSet;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<Vec<i32>> {
    input.lines().map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect()).collect()
}

//------------------------------ SOLVE

fn is_adjacent(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    (0..3).map(|x:usize| (a[x]-b[x]).abs()).sum::<i32>() == 1
}

fn find_pockets(p: &Vec<Vec<i32>>) -> usize {

    let minx = p.iter().map(|x| x[0]).min().unwrap();
    let maxx = p.iter().map(|x| x[0]).max().unwrap();

    let miny = p.iter().map(|x| x[1]).min().unwrap();
    let maxy = p.iter().map(|x| x[1]).max().unwrap();

    let minz = p.iter().map(|x| x[2]).min().unwrap();
    let maxz = p.iter().map(|x| x[2]).max().unwrap();

    let mut xgaps:FnvHashSet<(i32, i32, i32)> = FnvHashSet::default();
    let mut ygaps:FnvHashSet<(i32, i32, i32)> = FnvHashSet::default();
    let mut zgaps:FnvHashSet<(i32, i32, i32)> = FnvHashSet::default();

    for y in miny..=maxy {
        for z in minz..=maxz {
            let column:Vec<i32> = p.iter().filter(|c| c[1]==y && c[2] == z).map(|c| c[0]).collect();
            if let Some(min) = column.iter().min() {
                if let Some(max) = column.iter().max() {
                    for i in *min..*max {
                        if !column.contains(&i) {
                            xgaps.insert((i,y,z));
                        }
                    }
                }
            }
        }
    }

    for x in minx..=maxx {
        for z in minz..=maxz {
            let column:Vec<i32> = p.iter().filter(|c| c[0]==x && c[2] == z).map(|c| c[1]).collect();
            if let Some(min) = column.iter().min() {
                if let Some(max) = column.iter().max() {
                    for i in *min..*max {
                        if !column.contains(&i) {
                            ygaps.insert((x,i,z));
                        }
                    }
                }
            }
        }
    }

    for y in miny..=maxy {
        for x in minx..=maxx {
            let column:Vec<i32> = p.iter().filter(|c| c[1]==y && c[0] == x).map(|c| c[2]).collect();
            if let Some(min) = column.iter().min() {
                if let Some(max) = column.iter().max() {
                    for i in *min..*max {
                        if !column.contains(&i) {
                            zgaps.insert((x,y,i));
                        }
                    }
                }
            }
        }
    }

    let x_and_y:FnvHashSet<(i32, i32, i32)> = xgaps.intersection(&ygaps).map(|x| *x).collect();
    let x_and_y_and_z:FnvHashSet<(i32, i32, i32)> = x_and_y.intersection(&zgaps).map(|x| *x).collect();


    let p = &x_and_y_and_z;

    let mut count_adj = 0;
    for a in p {
        for b in p {
            if is_adjacent(&vec![a.0, a.1, a.2], &vec![b.0, b.1, b.2]) {
                count_adj += 1;
            }
        }
    }

    x_and_y_and_z.len() * 6 - count_adj

}


fn solve1(input: &'static str) -> usize {
    let p = parse(input);
    let mut count_adj = 0;
    for a in &p {
        for b in &p {
            if is_adjacent(a, b) {
                count_adj += 1;
            }
        }
    }


    p.len() * 6 - count_adj
}

fn solve2(input: &'static str) -> usize {
    let p = parse(input);
    let mut count_adj = 0;
    for a in &p {
        for b in &p {
            if is_adjacent(a, b) {
                count_adj += 1;
            }
        }
    }

    let pocks = find_pockets(&p);
    dbg!(&pocks);
    p.len() * 6 - count_adj - pocks
}
//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day18, part1)]
fn day18_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
#[aoc(day18, part2)]
fn day18_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ TESTS

#[test] fn test_day18_part1() { assert_eq!(solve1(_SAMPLE), 64); }
#[test] fn test_day18_part2() { assert_eq!(solve2(_SAMPLE), 58); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

const _ANS1: usize = 1;
const _ANS2: usize = 2;