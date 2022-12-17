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
#[test] fn test_day17_part1() { assert_eq!(solve1(_SAMPLE), 3068); }

fn solve(input: &'static str, part: usize) -> i32 {
    let wind = parse(input);
    let shapes = get_shapes();
    let shapes:Vec<Grid> = shapes.iter().map(|s| s.points.iter().map(|x| *x).collect::<Grid>()).collect();

    let mut si = 0;
    let mut wi = 0;

    let mut board:Grid = FnvHashSet::default();

    for _ in 0..2022 {
        let shape = &shapes[si];
        si = (si + 1) % shapes.len();
        let p:Point = (4 + height(&board), 2);

        let mut s = translate(shape, p);
        // println!("A new rock appears");
        // draw(&board, &s);
        loop {
            // Move left/right
            let dx = match wind[wi] {
                '<' => -1,
                '>' => 1,
                _ => panic!("Bad wind code {}", wind[wi]),
            };
            wi = (wi + 1) % wind.len();
            let ps = translate(&s, (0,dx));
            if !hit_wall(&ps) && !intersect(&board, &ps) {
                s = ps;
            }
            // println!("Wind pushes rock {}", dx);
            // draw(&board, &s);

            // Move down
            let ps = translate(&s, (-1,0));
            if !hit_floor(&ps) && !intersect(&board, &ps) {
                s = ps;
            } else {
                // rock comes to rest
                insert(&mut board, s);
                // println!("Rock falls one unit, causing it to come to rest");
                // draw(&board, &FnvHashSet::default());
                break;
            }
            // println!("Rock falls one unit");
            // draw(&board, &s);
        }
    }
    *height(&board)
}

fn solve1(input: &'static str) -> i32 { solve(input, 1) }
fn solve2(input: &'static str) -> i32 { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day17, part1)]
fn day17_part1(input: &'static str) -> i32 {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
// Uncomment next line when solution is ready
// #[aoc(day17, part2)]
fn day17_part2(input: &'static str) -> i32 {
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