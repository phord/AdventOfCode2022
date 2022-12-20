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
    for y in (0..*height).rev() {
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

fn find_pattern<T: std::cmp::PartialEq>( history: &Vec<T>) -> Option<(usize, usize)> {
    let max = history.len()/2;
    let min = history.len()/5;
    for width in (min..=max).rev() {
        let offset = history.len()-width*2;
        let end = offset + width;
        let end2 = end + width;
        if history[offset..end] == history[end..end2] {
            return Some((offset, width));
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

    for _ in 0..count {
        let shape = &st.shapes[st.si];
        st.si = (st.si + 1) % st.shapes.len();
        let h = *height(&st.board);
        let p:Point = (4 + h, 2);

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
                // Trim board levels more than 100 away from the top.
                st.board = st.board.iter().filter(|(y,_)| *y > h-100).into_iter().map(|x| *x).collect();
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
    let sample: &str = ">>><<><>><<<>><<<<<>><><>><>><>><><>>><>>><<><>>><<<>>><<<><<<>><>><<>>
";

    let wind = parse(sample);
    let shapes = get_shapes();
    let shapes:Vec<Grid> = shapes.iter().map(|s| s.points.iter().map(|x| *x).collect::<Grid>()).collect();
    let mut state = State {si: 0, wi:0, board: FnvHashSet::default(), wind, shapes, };

    dbg!(find_rock_pattern(&mut state));
}

fn find_rock_pattern(state: &mut State) -> Option<(usize, usize, i32)> {

    let mut history = Vec::new();
    let mut ph = 0;
    let rocks = state.wind.len();
    for _ in 0..rocks*4 {
        drop_rocks(state, 1);
        let nh = height(&state.board);
        history.push(nh-ph);
        ph = *nh;
    }
    // println!("{:?}", &history);
    match find_pattern(&history) {
        Some((offset, size)) => Some((offset, size, history[offset..offset+size].iter().sum())),
        _ => None
    }
}

 #[test] fn test_day17_part1() { assert_eq!(solve1(_SAMPLE), 3068); }
 fn solve1(input: &'static str) -> u64 {
    let wind = parse(input);
    let shapes = get_shapes();
    let shapes:Vec<Grid> = shapes.iter().map(|s| s.points.iter().map(|x| *x).collect::<Grid>()).collect();
    let mut state = State {si: 0, wi:0, board: FnvHashSet::default(), wind, shapes, };

    drop_rocks(&mut state, 2022);
    *height(&state.board) as u64
}

#[test] fn test_day17_part2() { assert_eq!(solve2(_SAMPLE), 1514285714288); }
fn solve2(input: &'static str) -> u64 {

    let rocks = 1_000_000_000_000;

    let wind = parse(input);
    let shapes = get_shapes();
    let shapes:Vec<Grid> = shapes.iter().map(|s| s.points.iter().map(|x| *x).collect::<Grid>()).collect();
    let mut state = State {si: 0, wi:0, board: FnvHashSet::default(), wind, shapes, };

    if let Some((offset, size, unit_height)) = find_rock_pattern(&mut state) {

        // We've dropped offset + size * 2 rocks so far.
        // We see a pattern repeating at least every size rocks.
        // Each next pattern results in unit_height more height.

        // Find out how many extra rocks we need to drop to make our target number and drop them
        let remainder = rocks - offset - size*2;
        let rem = remainder % size as usize;

        drop_rocks(&mut state, rem as i32);

        // How many patterns of rocks did we skip
        let virtualized = remainder / size as usize;

        let fh = *height(&state.board) as usize + virtualized * unit_height as usize;

        return fh as u64;
    }
    unreachable!();
}

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
    assert_eq!(ans, 1507692307690);
    ans
}

//------------------------------ TESTS


//------------------------------ SAMPLE DATA

const _SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

const _ANS1: usize = 1;
const _ANS2: usize = 2;