use std::rc::Rc;

#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use fnv::FnvHashSet;
use fnv::FnvHashMap;
use num::Integer;

//------------------------------ PARSE INPUT

type Point = (i32, i32);
struct Game {
    start: Point,
    finish: Point,
    width: i32,
    height: i32,
    lcm: usize,
    goal: Point,
    nodes:FnvHashSet<Rc<State>>,
    edges:FnvHashMap<Rc<State>, Vec<State>>,
    blizz_pos:Vec<FnvHashSet<Point>>,
}

impl Game {
    fn new(start: i32, finish: i32, width: i32, height: i32, blizz_pos: Vec<FnvHashSet<Point>>) -> Game {

        let mut game = Game {
            start: (0, start),
            finish: (height+1, finish),
            height,
            width,
            lcm: height.lcm(&width) as usize,
            goal: (height+1, finish),
            nodes: FnvHashSet::default(),
            edges: FnvHashMap::default(),
            blizz_pos};

        // A set of all valid positions to move in every round (640k pos)
        game.nodes =
            (0..game.lcm).flat_map(|round| {
                let b = &game.blizz_pos[round as usize];
                (1..=height)
                    .flat_map(move |row| (1..=width)
                        .map(move |col| Rc::new(State {round, pos: (row, col)})))
                    .filter(|state| !b.contains(&state.pos) )
                    .chain(std::iter::once(Rc::new(State {round, pos: (0, start)})))
                    .chain(std::iter::once(Rc::new(State {round, pos: (height+1, finish)})))
            })
            .collect();

        // A map of edges between every possible position (1.4m edges)
        static DIRS: [(i32, i32); 5] = [ (0,1), (1, 0), (0,-1), (-1, 0), (0,0), ];
        game.edges = game.nodes.iter()
            .map(|src| (src.clone(), {
                let round = (src.round + 1) % game.lcm;
                DIRS.iter().map(move |d| State {round, pos: (src.pos.0 + d.0, src.pos.1 + d.1)})
                .filter(|dest| game.nodes.contains(dest))
                .collect()
            }))
            .collect();

        game
    }
}

struct Blizzard {
    dir: Point,
    pos: Point,
}

fn parse(input: &'static str) -> Game {
    let mut start = 0;
    let mut finish = 0;
    let mut bliz: Vec<Blizzard> = Vec::new();

    let height = input.lines().count() as i32 - 2;
    let width = input.lines().next().unwrap().len() as i32 - 2;
    let lcm = height.lcm(&width) as usize;

    for (row, l) in  input.lines().enumerate() {
        for (col, ch) in l.chars().enumerate() {
            let pos = (row as i32, col as i32);
            match ch {
                '<' => bliz.push(Blizzard { dir: (0, -1), pos}),
                '>' => bliz.push(Blizzard { dir: (0, 1), pos}),
                '^' => bliz.push(Blizzard { dir: (-1, 0), pos}),
                'v' => bliz.push(Blizzard { dir: (1, 0), pos}),
                '.' => if row == 0 { start = pos.1; } else { finish = pos.1; }
                _ => {},
            }
        }
    }

    // Map of all blizzard positions
    let blizz_pos:Vec<FnvHashSet<Point>> = (0..=lcm as i32).map(|round| bliz.iter().map(
        |blizzard| blizz_mov(height, width, round, &blizzard)).collect::<FnvHashSet<Point>>())
        .collect();

    // Sanity check that the cycle repeats
    assert!(blizz_pos[0] == blizz_pos[lcm]);
    assert!(blizz_pos[0] != blizz_pos[lcm-1]);

    Game::new(start, finish, width, height, blizz_pos)
}

//------------------------------ SOLVE
#[derive(Eq, Hash, PartialEq, PartialOrd, Clone, Debug)]
struct State {
    round: usize,
    pos: Point,
}

fn blizz_mov(height: i32, width: i32, round: i32, b: &Blizzard) -> Point {
    let (row, col) = b.pos;
    let (dr, dc) = b.dir;
    let row = (row + dr * round - 1).rem_euclid(height) + 1;
    let col = (col + dc * round - 1).rem_euclid(width) + 1;
    (row, col)
}

#[allow(unused)]
fn draw(game: &Game, state: &State) {
    let blizz_pos = &game.blizz_pos[state.round % game.lcm];
    for row in 0..=game.height+1 {
        print!("#");
        for col in 1..=game.width {
            let p = (row, col);
            print!("{}",
                if p == state.pos {"E"}
                else if (row == 0 || row == game.height+1) && p != game.start && p != game.finish {"#"}
                else if blizz_pos.contains(&p) {"X"}
                else {"."});
        }
        println!("#");
    }
}

fn bfs(game: &Game, round: usize) -> usize {
    let mut count = 0;
    let state = State {round: round % game.lcm, pos: game.start};
    let mut pos = vec![&state];
    loop {
        if pos.iter().any(|state| state.pos == game.goal ) {
            return count;
        }

        count += 1;
        pos = pos.iter().flat_map(|state| &*game.edges[*state] ).collect();
        pos.sort_by(|x,y| x.pos.partial_cmp(&y.pos).unwrap());
        pos.dedup();
    }
}



fn solve(input: &'static str, part: usize) -> usize {
    let mut game = parse(input);

    if part == 1 {
        bfs(&game, 0)
    } else {
        let mut dist = bfs(&game, 0);

        std::mem::swap(&mut game.start, &mut game.goal);
        dist += bfs(&game, dist);

        std::mem::swap(&mut game.start, &mut game.goal);
        dist += bfs(&game, dist);

        dist
    }
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day24, part1)]
fn day24_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    assert_eq!(ans, 297);
    ans
}

#[allow(unused)]
#[aoc(day24, part2)]
fn day24_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    assert_eq!(ans, 856);
    ans
}

//------------------------------ TESTS

#[test] fn test_day24_part1() { assert_eq!(solve1(_SAMPLE), _ANS1); }
#[test] fn test_day24_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
// "#.#####
// #.....#
// #>....#
// #.....#
// #...v.#
// #.....#
// #####.#";

const _ANS1: usize = 18;
const _ANS2: usize = 54;