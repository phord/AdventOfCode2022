#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use fnv::FnvHashSet;

//------------------------------ PARSE INPUT

type Point = (i32, i32);
type Grid = FnvHashSet<Point>;

struct Shape {
    points: Vec<Point>,
}

fn get_shapes () -> Vec<Shape> {
    vec![
        Shape { points: vec![(0,0), (0,1), (0,2), (0,3)]},          // ----
        Shape { points: vec![(2,1), (1,0), (1,1), (1,2), (0,1)]},   // +
        Shape { points: vec![(0,0), (0,1), (0,2), (1,2), (2,2)]},   // _|
        Shape { points: vec![(0,0), (1,0), (2,0), (3,0)]},          // |
        Shape { points: vec![(0,0), (1,0), (0,1), (1,1)]},          // ::
    ]
}



fn parse(input: &'static str) -> Vec<char> {
    input.lines().next().unwrap().chars().collect()
}

//------------------------------ SOLVE

fn translate(a: &Grid, p: Point) -> Grid {
    let (y,x) = p;
    a.iter().map(|(y0,x0)| (y0+y, x0+x)).collect()
}

fn insert(a: &mut Grid, b: Grid) {
    for p in b {
        a.insert(p);
    }
}

fn intersect(a: &Grid, b: &Grid) -> bool {
    if b.len() > a.len() {
        intersect(b, a)
    } else {
        b.iter().any(|p| a.contains(p))
    }
}

// Does this shape intersect with the walls?
fn hit_wall(a: &Grid) -> bool {
    a.iter().any(|(_,x)| *x < 0 || *x > 6)
}

// Does this shape intersect with the floor?
fn hit_floor(a: &Grid) -> bool {
    a.iter().any(|(y,_)| *y <= 0 )
}
fn height(a: &Grid) -> &i32 {
    a.iter().map(|(y,_)| y).max().unwrap_or(&0i32)
}

fn draw(a: &Grid, b: &Grid) {
    let height = *[height(&a), height(&b)].iter().max().unwrap();
    for iy in 0..*height {
        let y = *height-iy;
        print!("|");
        for x in 0..7 {
            let p = (y,x);
            if a.contains(&p) {
                print!("#");
            } else if b.contains(&p) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
}

enum Rocks {
    Count(usize, usize),
    Measure(usize),
}

fn find_pattern<T: std::cmp::PartialEq>( history: &Vec<T>) -> Option<(usize, usize)> {
    let max = history.len()/2;
    for width in (2..=max).rev() {
        let maxoff = history.len()-width*2;
        for offset in 0..=maxoff {
            let end = offset + width;
            let end2 = end + width;
            if history[offset..end] == history[end..end2] {
                return Some((offset, width));
            }
        }
    }
    None
}


#[test] fn test_pattern() {
    dbg!(find_pattern( &vec![1,2,3,4,5,6,7,8,9]));
    assert_eq!(find_pattern( &vec![1,2,3,4,5,6,7,8,9,4,5,6,7,8,9]), Some((3,6)));
    assert_eq!(find_pattern( &vec![1,2,3,4,5,6,4,5,6,7,8,9,4,5,6,4,5,6,7,8,9]), Some((3,9)));
}

struct State {
    si: usize,
    wi: usize,
    board: Grid,
    wind: Vec<char>,
    shapes: Vec<Grid>,
}

fn drop_rocks(st: &mut State, count: i32) {

    let quiet = true;

    for rock in 0..count {
        let shape = &st.shapes[st.si];
        st.si = (st.si + 1) % st.shapes.len();
        let p:Point = (4 + height(&st.board), 2);

        let mut s = translate(shape, p);
        if !quiet {
            println!("A new rock appears");
            draw(&st.board, &s);
        }
        loop {
            // Move left/right
            let dx = match st.wind[st.wi] {
                '<' => -1,
                '>' => 1,
                _ => panic!("Bad wind code {}", st.wind[st.wi]),
            };
            st.wi = (st.wi + 1) % st.wind.len();
            let ps = translate(&s, (0,dx));
            if !hit_wall(&ps) && !intersect(&st.board, &ps) {
                s = ps;
            }
            if !quiet {
                println!("Wind pushes rock {}", dx);
                draw(&st.board, &s);
            }

            // Move down
            let ps = translate(&s, (-1,0));
            if !hit_floor(&ps) && !intersect(&st.board, &ps) {
                s = ps;
            } else {
                // rock comes to rest
                insert(&mut st.board, s);
                if !quiet {
                    println!("Rock falls one unit, causing it to come to rest");
                    draw(&st.board, &FnvHashSet::default());
                }
                break;
            }
            if !quiet {
                println!("Rock falls one unit");
                draw(&st.board, &s);
            }
        }
    }
}
/**
 * Searching for a pattern
*/
#[test] fn test_rock_pattern() {
    const sample: &str = ">>><<><>><<<>><<<<<>><><>><>><>><><>>><>>><<><>>><<<>>><<<><<<>><>><<>>
";

    let wind = parse(sample);
    let shapes = get_shapes();
    let shapes:Vec<Grid> = shapes.iter().map(|s| s.points.iter().map(|x| *x).collect::<Grid>()).collect();
    let mut state = State {si: 0, wi:0, board: FnvHashSet::default(), wind, shapes, };

    dbg!(find_rock_pattern(&mut state));
}

fn find_rock_pattern(state: &mut State) -> Option<(usize, usize)> {

    let mut history = Vec::new();
    let mut ph = 0;
    let rocks = state.wind.len();
    for rock in 0..rocks*10 {
        drop_rocks(state, 1);
        let nh = height(&state.board);
        history.push((state.si, nh-ph));
        ph = *nh;
    }
    println!("{:?}", &history);
    let patt = find_pattern(&history);
    // match patt {
    //     None => {},
    //     Some((mut off, mut len)) => {
    //         println!("Pattern: ({}, {})", off, len);

    //         let mut run = history.clone();
    //         loop {
    //             run = run[off..off+len].iter().map(|x| *x).collect::<Vec<(usize, i32)>>();
    //             let p = find_pattern(&run);
    //             match p {
    //                 None => {println!("No sub-patterns!"); break;},
    //                 Some((0, l)) => {
    //                     println!("Subpattern: (0, {})", l);
    //                     off = 0;
    //                     len = l;
    //                 },
    //                 _ => {println!("Subpattern with offset {:?}", p); break;},
    //             }
    //         }
    //     },
    // }
    patt
}

 #[test] fn test_day17_part1() { assert_eq!(solve1(_SAMPLE), 3068); }
fn solve(input: &'static str, part: usize) -> u64 {

    let rocks:u64 = if part == 1 { 2022 } else { 1_000_000_000_000 };

    let wind = parse(input);
    let shapes = get_shapes();
    let shapes:Vec<Grid> = shapes.iter().map(|s| s.points.iter().map(|x| *x).collect::<Grid>()).collect();
    let mut state = State {si: 0, wi:0, board: FnvHashSet::default(), wind, shapes, };

    // 35 is empirically determined to be the number of rocks where the pattern repeats. But the first rocks
    // are special (up to 15).  So let's squash them by dropping 35 (for easy math) at the start.
    let pattern = (state.wind.len() * 5 * 7) as i32;
    drop_rocks(&mut state, pattern);
    let h = *height(&state.board);

    // Now drop 35 more so we can measure the height of our pattern
    drop_rocks(&mut state, pattern);
    let ph = *height(&state.board);

    // Find out how many extra rocks we need to drop to make our target number and drop them
    let remainder = rocks % pattern as u64;
    drop_rocks(&mut state, remainder as i32);

    // How many patterns of rocks did we skip
    let virtualized = rocks / pattern as u64 - 2;

    let fh = *height(&state.board) as u64 + virtualized * (ph - h) as u64;

    fh
}

fn solve1(input: &'static str) -> u64 { solve(input, 1) }
fn solve2(input: &'static str) -> u64 { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day17, part1)]
fn day17_part1(input: &'static str) -> u64 {
    let ans = solve1(input);
    assert_eq!(ans, 3055);
    ans
}

#[allow(unused)]
#[aoc(day17, part2)]
fn day17_part2(input: &'static str) -> u64 {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ TESTS

#[test] fn test_day17_part2() { assert_eq!(solve2(_SAMPLE), 0); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

const _ANS1: usize = 1;
const _ANS2: usize = 2;