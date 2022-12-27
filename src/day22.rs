use fnv::FnvHashMap;
#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashMap;
use colored::*;
use termion::cursor;
use std::{thread, time};


//------------------------------ PARSE INPUT

type Grid = HashMap<Point, Cell>;
type Point = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Floor,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Same,
}
use nom::{
    IResult,
    character::complete::{multispace0, digit1, one_of},
    combinator::opt,
};

fn parse_dist(input: &'static str) -> IResult<&'static str, usize> {
    let (i, _) = multispace0(input)?;
    let (i, arg1) = digit1(i)?;
    Ok((i, arg1.parse::<usize>().unwrap()))
}

fn parse_dir(input: &'static str) -> IResult<&'static str, Direction> {
    let (i, _) = multispace0(input)?;
    let (i, d) = opt(one_of("RL"))(i)?;
    let dir = match d {
        Some('R') => Direction::Right,
        Some('L') => Direction::Left,
        None => Direction::Same,
        _ => panic!(),
    };
    Ok((i, dir))
}

fn parse(input: &'static str) -> (Grid, Vec<(usize, Direction)>) {

    let mut map = Grid::new();
    for (row, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        for (col, ch) in line.chars().enumerate().filter(|(_, ch)| *ch != ' ') {
            map.insert((row as i32, col as i32),
                match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Floor,
                    _ => panic!(),
                });
        }
    }
    let mut i = input.lines().last().unwrap();
    let mut plan = Vec::new();
    while !i.is_empty() {
        let (ii,ds) = parse_dist(i).unwrap();
        let (ii,dr) = parse_dir(ii).unwrap();
        i = ii;
        plan.push((ds,dr));
    }
    (map, plan)
}

// Part1 : Wrap around across empty areas
fn next(map: &Grid, pos: Point, dir: &Direction) -> Option<Point> {
    let (row, col) = pos;
    let maxrow = map.iter().filter(|((_,c),_)| *c == col).map(|((r,_),_)| *r).max().unwrap();
    let minrow = map.iter().filter(|((_,c),_)| *c == col).map(|((r,_),_)| *r).min().unwrap();
    let maxcol = map.iter().filter(|((r,_),_)| *r == row).map(|((_,c),_)| *c).max().unwrap();
    let mincol = map.iter().filter(|((r,_),_)| *r == row).map(|((_,c),_)| *c).min().unwrap();

    let next = match dir {
        Direction::Up => ( if row == minrow { maxrow } else {row-1}, col),
        Direction::Down => ( if row == maxrow { minrow } else {row+1}, col),
        Direction::Left => ( row, if col == mincol { maxcol } else {col-1}),
        Direction::Right => ( row, if col== maxcol { mincol } else {col+1}),
        Direction::Same => panic!(),
    };
    match map[&next] {
        Cell::Floor => Some(next),
        _ => None,
    }
}

type Cube = Vec<Grid>;
fn split_faces(map: &Grid, width: i32) -> Cube {
    let mut cube = Vec::new();
    for row in 0.. {
        let row = row * width as i32;
        for col in (0..3).rev() {
            let col = col * width as i32;
            let face = map
                .iter()
                .filter(|((r,c),_)| *r >= row && *r < row + width && *c >= col && *c < col + width )
                .map(|((r,c), f)| ((*r-row, *c-col), *f) )
                .collect::<Grid>();
            match face.len() {
                2500 => cube.push(face),
                0 => {}, // no face, no biggie
                _ => panic!("Unexpected face size: {}", face.len()),
            }
            if cube.len() == 6 {
                return cube;
            }
        }
    }
    unreachable!();
}

fn move_car(game: &Game, face: usize, pos: &Point, dir: &Direction) -> (usize, Point, Direction) {
    let (row, col) = pos;

    let mut row = *row;
    let mut col = *col;
    let mut dir = *dir;
    let mut face = face;

    match dir {
        Direction::Up => row -= 1,
        Direction::Down => row += 1,
        Direction::Left => col -= 1,
        Direction::Right => col += 1,
        Direction::Same => panic!(),
    };

    if row < 0 || row >= game.width || col < 0 || col >= game.width {
        // Walked off the edge.  Pick a new face.
        let (f, rotation) = game.adj[&face][&dir];

        row = (row + game.width) % game.width;
        col = (col + game.width) % game.width;
        face = f;
        let max_pos = game.width - 1;
        match rotation {
            0   => {},
            90  => { let tmp = row; row = max_pos - col; col = tmp; }             // 2,20 => 20,48
            180 => { row = max_pos - row; col = max_pos - col; }                  // 2,20 => 48,30
            -90 => { let tmp = row; row = col; col = max_pos - tmp; }             // 2,20 => 48,2
            _ => panic!("Bad rotation: {}", rotation),
        }
        match rotation {
            0   => {},
            90  => { dir = turn_left(dir); },
            180 => { dir = turn_left(dir); dir = turn_left(dir); },
            -90 => { dir = turn_right(dir)},
            _ => panic!("Bad rotation: {}", rotation),
        }
    }
    let next = (row, col);
    (face, next, dir)
}

// Slow lookup, but rarely needed
fn map_to_face(game: &Game, pos: &Point) -> Option<usize> {
    let (r,c) = pos;
    let r = r % game.width;
    let c = c % game.width;
    let p = (r,c);

    for face in 1..=6 {
        if *pos == pos_to_map(game, face, &p) {
            return Some(face);
        }
    }
    return None;
}

fn pos_to_map(game: &Game, face: usize, pos: &Point) -> Point {
    let (r, c) = match face {
        1 => (0,2),
        2 => (0,1),
        3 => (1,1),
        4 => (2,1),
        5 => (2,0),
        6 => (3,0),
        _ => panic!("Bad face"),
    };

    let (row, col) = pos;
    (row + game.width * r, col + game.width * c)
}

fn next2(game: &Game, face: usize, pos: Point, dir: &Direction) -> Option<(usize, Point, Direction)> {

    let (next_face, next, dir) = move_car(game, face, &pos, dir);
    let map = &game.faces[next_face - 1];

    match map[&next] {
        Cell::Floor => Some((next_face, next, dir)),
        _ => None,
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
        Direction::Same => panic!(),
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
        Direction::Same => panic!(),
    }
}

fn is_floor(cell: &Cell) -> bool {
    match cell {
        Cell::Floor => true,
        _ => false
    }
}

//------------------------------ SOLVE

fn solve1(input: &'static str) -> i32 {
    let (map, plan) = parse(input);

    let col = map.iter().filter(|((r,_),cell)| *r == 0 && is_floor(cell)).map(|((_,c),_)| *c).min().unwrap();
    let mut pos = (0, col);
    let mut dir = Direction::Right;

    for (dist, turn) in plan {
        for _ in 0..dist {
            match next(&map, pos, &dir) {
                Some(p) => pos = p,
                None => {},
            };
        }
        dir = match turn {
            Direction::Right => turn_right(dir),
            Direction::Left => turn_left(dir),
            Direction::Same => dir,
            _ => panic!(),
        };
    }

    score(&dir, &pos)
}

struct Game {
    map: Grid,
    faces: Cube,
    adj: Adjacency,
    width: i32,
}

type Adjacency = FnvHashMap<usize, FnvHashMap<Direction, (usize, i32)>>;

fn get_adjacency() -> Adjacency {
    const ADJACENT:[[(usize, i32); 4]; 6] = [
    //     Up        Left        Right        Down    Face
        [ (6,   0), (2,    0),  (4, 180), (3, -90)],  // 1
        [ (6, -90), (5,  180),  (1,   0), (3,   0)],  // 2
        [ (2,   0), (5,   90),  (1,  90), (4,   0)],  // 3
        [ (3,   0), (5,    0),  (1, 180), (6, -90)],  // 4
        [ (3, -90), (2,  180),  (4,   0), (6,   0)],  // 5
        [ (5,   0), (2,   90),  (4,  90), (1,   0)],  // 6
    ];

    let mut amap: Adjacency = FnvHashMap::default();

    for (f, map) in ADJACENT.iter().enumerate() {
        let f = f + 1; // 1-based face numbers
        let mut mm = FnvHashMap::default();
        for (d, path) in map.iter().enumerate() {
            let d = match d {
                0 => Direction::Up,
                1 => Direction::Left,
                2 => Direction::Right,
                3 => Direction::Down,
                _ => unreachable!(),
            };
            mm.insert(d, *path);
        }
        amap.insert(f, mm);
    }
    amap
}

type History = (usize, Point, Direction);
fn solve2(input: &'static str) -> i32 {
    let (map, plan) = parse(input);

    let width = 50;
    let faces = split_faces(&map, width).clone();

    let game = Game { map, faces, adj: get_adjacency(), width };

    // Starting point and direction
    let col = game.map.iter().filter(|((r,_),cell)| *r == 0 && is_floor(cell)).map(|((_,c),_)| *c).min().unwrap();
    let mut pos = (0, col % game.width);
    let mut dir = Direction::Right;

    let mut face = map_to_face(&game, &(0, col)).unwrap();

    let mut history: Vec<History> = Vec::new();
    let max_history = 20;
    for (dist, turn) in plan {
        for _ in 0..dist {
            match next2(&game, face, pos, &dir) {
                Some((f, p, d)) => {
                    face = f; pos = p; dir = d;
                    history.push((face, pos, dir));
                    if history.len() == max_history {
                        history = history[1..].to_vec();
                    }
                    display_cube_face(&game, &history);
                },
                None => {},
            };
        }
        dir = match turn {
            Direction::Right => turn_right(dir),
            Direction::Left => turn_left(dir),
            Direction::Same => dir,
            _ => panic!(),
        };
    }

    score(&dir, &pos_to_map(&game, face, &pos))
}

fn score(dir: &Direction, pos: &Point) -> i32 {
    let numdir = match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        Direction::Same => panic!(),
    };

    1004 + 1000 * pos.0 + 4 * pos.1 + numdir
}

fn get_cell(map: &Grid, car: &Vec<&History>, cur: &Point) -> ColoredString {
    if let Some((_, _, dir)) = car.iter().find(|(_, pos, _)| *cur == *pos) {
        let car = match dir {
                Direction::Right => ">",
                Direction::Down => "V",
                Direction::Left => "<",
                Direction::Up => "^",
                Direction::Same => panic!(),
        };
        return car.bright_yellow();
    }
    if map.contains_key(&cur) {
        match map[&cur] {
            Cell::Floor => ".".green(),
            Cell::Wall => "#".red(),
        }
    } else {
        " ".black()
    }

}

fn draw_face(game: &Game, face: &usize, history: &Vec<History>) -> Vec<Vec<ColoredString>> {
    let history = history.iter().filter(|(f, _, _)| f == face).collect::<Vec<&History>>();
    let map = &game.faces[face-1];
    (0..game.width).map(|row|
        (0..game.width).map(|col| {
            let cur = (row, col);
            get_cell(map, &history, &cur)
        }).collect::<Vec<_>>()
    ).collect()
}

#[allow(unused)]
fn display_cube_face(game: &Game, history: &Vec<History>) {

    let (face, pos, dir) = history.last().unwrap();
    println!("{}", cursor::Goto(1, 1)); //, clear::All);
    thread::sleep(time::Duration::from_millis(30));

    println!();

    let (r,c) = pos;
    let pos = &(r % game.width, c % game.width);

    let front = draw_face(game, &face, history);
    // FIXME: Rotate front face to match expected orientation

    // FIXME: Find correct orientation of left face
    let left_face = game.adj[&face][&Direction::Left];
    let left = draw_face(game, &left_face.0, history);

    // FIXME: Find correct orientation of top face
    let top_face = game.adj[&face][&Direction::Up];
    let top = draw_face(game, &top_face.0, history);

    for row in 0..game.width {
        println!();
        // First half of left face
        for skew in 0..row {
            let row = row;
            print!("{}", left[(row-skew) as usize][skew as usize]);
        }
        // Edge
        print!("{}", "\\\\".white());
        // Top face
        for col in 0..game.width {
            let c = &top[row as usize][col as usize];
            print!("{}{}", c, c);
        }
    }

    for row in 0..=game.width {
        println!();
        // Second half of left face
        for skew in 0..game.width {
            let row = row;
            let p = game.width + row - skew;
            if p >= game.width {
                print!(" ");
            } else {
                print!("{}", left[(p) as usize][(game.width - p + row) as usize]);
            }
        }
        // Edge
        print!("{}", "||".white());
        if row == 0 {
            // Edge
            print!("{}", "====================================================================================================  ".white());
            print!("front={} top={} left={}  ", face, &top_face.0, &left_face.0);
        } else {
            let row = row - 1;
            for col in 0..game.width {
                let c = &front[row as usize][col as usize];
                print!("{}{}", c, c);
            }
        }
        if row == 0 {
            print!("{}, {}      ", pos.0, pos.1);
        }
    }
}

//------------------------------ RUNNERS

#[allow(unused)]
// #[aoc(day22, part1)]
fn day22_part1(input: &'static str) -> i32 {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
#[aoc(day22, part2)]
fn day22_part2(input: &'static str) -> i32 {
    let ans = solve2(input);
    assert_eq!(ans, 182170);
    ans
}

//------------------------------ TESTS

#[test] fn map_makes_sense() {

    let amap = get_adjacency();

    for face in 1..=6 {
        for (dir, (target, rotate)) in amap[&face].iter() {
            let rotate = (rotate + 180) % 360;
            let mut odir = *dir;
            for _ in 0..rotate/90 {
                odir = turn_left(odir);
            }

            // dbg!((face, dir, target, odir));

            let (src, src_rotate) = amap[target][&odir];
            assert_eq!(src, face);
            let src_rotate = (180 + 360 - src_rotate) % 360;
            assert_eq!(rotate, src_rotate);
        }
    }
}
#[test] fn test_day22_part1() { assert_eq!(solve1(_SAMPLE), 6032); }
// #[test] fn test_day22_part2() { assert_eq!(solve2(_SAMPLE), 5031); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";