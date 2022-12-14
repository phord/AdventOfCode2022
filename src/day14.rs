#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashSet;

//------------------------------ PARSE INPUT


type Grid = HashSet<(usize, usize)>;

#[allow(unused)]
fn sample() -> Vec<Vec<(usize, usize)>> {
    vec![vec![(498,4 ), ( 498,6 ), ( 496,6 )],
    vec![(503,4 ), ( 502,4 ), ( 502,9 ), ( 494,9)]]
}

fn parse(sample: &Vec<Vec<(usize, usize)>>) -> Grid {

    let mut g = Grid::new();
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
fn fall(g: &mut Grid, x: usize, y: usize, max: &usize, part: usize) -> bool {
    let mut x = x;
    let mut y = y;
    if g.contains(&(x,y)) {
        return false;
    }


    loop {
        assert!( y < *max+2 );
        if part == 1 {
            if y > *max {
                return false;
            }
        } else {
            if y == *max+1 {
                g.insert((x,y));
                return true;
            }
        }

        if !g.contains(&(x,y+1)) {
            y += 1;
        } else if x > 0 && !g.contains(&(x-1,y+1)) {
            y += 1;
            x -= 1;
        } else if !g.contains(&(x+1,y+1)) {
            y += 1;
            x += 1;
        } else {
            g.insert((x,y));
            return true;
        }
    }
}
fn solve(g:Grid, part: usize) -> usize {

    let max = g.iter().map(|(_,b)| b).max().unwrap();
    let mut g = g.clone();

    for count in 0.. {
        if ! fall(&mut g, 500, 0, max, part) {
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
fn day14_part1(_: &'static str) -> usize {
    let ans = solve1(parse(&input()));
    assert_eq!(ans, 692);
    ans
}

#[allow(unused)]
#[aoc(day14, part2)]
fn day14_part2(_: &'static str) -> usize {
    let ans = solve2(parse(&input()));
    assert_eq!(ans, 31706);
    ans
}

//------------------------------ TESTS

#[test] fn test_day14_part1() { assert_eq!(solve1(parse(&sample())), 24); }
#[test] fn test_day14_part2() { assert_eq!(solve2(parse(&sample())), 93); }

fn input() -> Vec<Vec<(usize, usize)>> {
    vec![
            vec![(480,150), (485,150)],
            vec![(459,100), (464,100)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(504,19), (508,19)],
            vec![(467,59), (472,59)],
            vec![(463,103), (468,103)],
            vec![(472,68), (477,68)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(470,103), (475,103)],
            vec![(479,36), (479,40), (477,40), (477,48), (489,48), (489,40), (483,40), (483,36)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(467,128), (467,132), (462,132), (462,138), (477,138), (477,132), (472,132), (472,128)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(452,112), (458,112), (458,111)],
            vec![(460,115), (460,117), (456,117), (456,125), (469,125), (469,117), (464,117), (464,115)],
            vec![(486,25), (490,25)],
            vec![(471,109), (476,109)],
            vec![(467,179), (467,180), (472,180), (472,179)],
            vec![(487,150), (492,150)],
            vec![(501,16), (505,16)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(479,36), (479,40), (477,40), (477,48), (489,48), (489,40), (483,40), (483,36)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(464,109), (469,109)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(489,22), (493,22)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(460,115), (460,117), (456,117), (456,125), (469,125), (469,117), (464,117), (464,115)],
            vec![(471,55), (471,56), (479,56)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(498,25), (502,25)],
            vec![(467,128), (467,132), (462,132), (462,138), (477,138), (477,132), (472,132), (472,128)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(498,13), (502,13)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(472,144), (477,144)],
            vec![(466,150), (471,150)],
            vec![(479,36), (479,40), (477,40), (477,48), (489,48), (489,40), (483,40), (483,36)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(507,22), (511,22)],
            vec![(467,128), (467,132), (462,132), (462,138), (477,138), (477,132), (472,132), (472,128)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(479,36), (479,40), (477,40), (477,48), (489,48), (489,40), (483,40), (483,36)],
            vec![(498,19), (502,19)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(458,68), (463,68)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(475,65), (480,65)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(492,19), (496,19)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(460,115), (460,117), (456,117), (456,125), (469,125), (469,117), (464,117), (464,115)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(467,128), (467,132), (462,132), (462,138), (477,138), (477,132), (472,132), (472,128)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(466,100), (471,100)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(464,62), (469,62)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(456,103), (461,103)],
            vec![(484,28), (484,30), (481,30), (481,33), (488,33), (488,30), (487,30), (487,28)],
            vec![(467,179), (467,180), (472,180), (472,179)],
            vec![(467,128), (467,132), (462,132), (462,138), (477,138), (477,132), (472,132), (472,128)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(467,106), (472,106)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(474,50), (474,51), (486,51)],
            vec![(468,65), (473,65)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(479,68), (484,68)],
            vec![(476,147), (481,147)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(453,106), (458,106)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(484,28), (484,30), (481,30), (481,33), (488,33), (488,30), (487,30), (487,28)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(484,28), (484,30), (481,30), (481,33), (488,33), (488,30), (487,30), (487,28)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(483,147), (488,147)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(501,22), (505,22)],
            vec![(465,68), (470,68)],
            vec![(479,144), (484,144)],
            vec![(484,28), (484,30), (481,30), (481,33), (488,33), (488,30), (487,30), (487,28)],
            vec![(484,28), (484,30), (481,30), (481,33), (488,33), (488,30), (487,30), (487,28)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(474,106), (479,106)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(471,55), (471,56), (479,56)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(475,141), (480,141)],
            vec![(460,115), (460,117), (456,117), (456,125), (469,125), (469,117), (464,117), (464,115)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(484,28), (484,30), (481,30), (481,33), (488,33), (488,30), (487,30), (487,28)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(457,109), (462,109)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(467,128), (467,132), (462,132), (462,138), (477,138), (477,132), (472,132), (472,128)],
            vec![(504,25), (508,25)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(495,22), (499,22)],
            vec![(492,25), (496,25)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(467,128), (467,132), (462,132), (462,138), (477,138), (477,132), (472,132), (472,128)],
            vec![(460,106), (465,106)],
            vec![(469,147), (474,147)],
            vec![(450,109), (455,109)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(452,112), (458,112), (458,111)],
            vec![(461,65), (466,65)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(467,179), (467,180), (472,180), (472,179)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(460,115), (460,117), (456,117), (456,125), (469,125), (469,117), (464,117), (464,115)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(462,97), (467,97)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(510,25), (514,25)],
            vec![(495,16), (499,16)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(458,81), (458,77), (458,81), (460,81), (460,73), (460,81), (462,81), (462,80), (462,81), (464,81), (464,77), (464,81), (466,81), (466,78), (466,81), (468,81), (468,73), (468,81)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(479,36), (479,40), (477,40), (477,48), (489,48), (489,40), (483,40), (483,36)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(472,176), (472,175), (472,176), (474,176), (474,174), (474,176), (476,176), (476,174), (476,176), (478,176), (478,175), (478,176), (480,176), (480,166), (480,176), (482,176), (482,172), (482,176), (484,176), (484,167), (484,176)],
            vec![(460,115), (460,117), (456,117), (456,125), (469,125), (469,117), (464,117), (464,115)],
            vec![(448,94), (448,87), (448,94), (450,94), (450,93), (450,94), (452,94), (452,91), (452,94), (454,94), (454,88), (454,94), (456,94), (456,91), (456,94), (458,94), (458,86), (458,94), (460,94), (460,86), (460,94), (462,94), (462,89), (462,94), (464,94), (464,90), (464,94)],
            vec![(479,36), (479,40), (477,40), (477,48), (489,48), (489,40), (483,40), (483,36)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(464,163), (464,158), (464,163), (466,163), (466,154), (466,163), (468,163), (468,153), (468,163), (470,163), (470,160), (470,163), (472,163), (472,162), (472,163), (474,163), (474,159), (474,163), (476,163), (476,155), (476,163), (478,163), (478,159), (478,163)],
            vec![(473,150), (478,150)],
            vec![(471,62), (476,62)],
            vec![(484,28), (484,30), (481,30), (481,33), (488,33), (488,30), (487,30), (487,28)],
            vec![(479,36), (479,40), (477,40), (477,48), (489,48), (489,40), (483,40), (483,36)],
            vec![(478,109), (483,109)],
            vec![(474,50), (474,51), (486,51)],
            vec![(460,115), (460,117), (456,117), (456,125), (469,125), (469,117), (464,117), (464,115)]]
}