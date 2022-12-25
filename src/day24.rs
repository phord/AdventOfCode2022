use std::collections::VecDeque;

#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use fnv::FnvHashSet;
use fnv::FnvHashMap;
use num::Integer;
use itertools::Itertools;

//------------------------------ PARSE INPUT

type Point = (i32, i32);
type Point3 = (i32, i32, i32);
struct Game {
    start: i32,
    finish: i32,
    width: i32,
    height: i32,
    lcm: usize,
    nodes:FnvHashSet<Point3>,
    edges:FnvHashMap<Point3, Vec<Point3>>,
    blizz_pos:Vec<FnvHashSet<Point>>,
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

    // Build map of all blizzard positions
    let blizz_pos:Vec<FnvHashSet<Point>> = (0..=lcm as i32).map(|round| bliz.iter().map(
        |blizzard| blizz_mov(height, width, round, &blizzard)).collect::<FnvHashSet<Point>>())
        .collect();

    // Sanity check that the cycle repeats
    assert!(blizz_pos[0] == blizz_pos[lcm]);
    assert!(blizz_pos[0] != blizz_pos[lcm-1]);

    // A set of all valid positions to move in every round (640k pos)
    let nodes:FnvHashSet<Point3> =
        (0..lcm as i32).flat_map(|round| {
            let b = &blizz_pos[round as usize];
            (1..=height).flat_map(move |row| (1..=width).map(move |col| (round, row, col)))
                .filter(|p| !b.contains(&(p.1, p.2)) )
                .chain(std::iter::once((round, 0, start)))
                .chain(std::iter::once((round, height+1, finish)))
        })
        .collect();

    println!("Nodes: {}", nodes.len());

    // A set of edges between every possible position (1.4m edges)
    let edges:FnvHashMap<Point3, Vec<Point3>> = nodes.iter()
        .filter(|src| src.1 <= height)  // Remove finish cell as a source node
        .map(|src| (*src, {
            let (r, y, x) = src;
            let round = (r+1) % lcm as i32;
            DIRS.iter().map(move |d| (round, y + d.0, x + d.1))
            .filter(|dest| (*y == 0 || dest.1 > 0) && nodes.contains(&dest))
            .collect()
        }))
        .collect();

    println!("Edges: {}", edges.len());

    Game {start, finish, height, width, lcm, nodes, edges, blizz_pos}
}

//------------------------------ SOLVE
#[derive(Eq, Hash, PartialEq, PartialOrd, Clone)]
struct State {
    round: usize,
    pos: Point,
}
type Path = FnvHashSet<State>;
type Memo = FnvHashMap<State, Option<usize>>;
fn mov(pos: Point, dir: Point) -> Point {
    (pos.0 + dir.0, pos.1 + dir.1)
}
fn blizz_mov(height: i32, width: i32, round: i32, b: &Blizzard) -> Point {
    let (row, col) = b.pos;
    let (dr, dc) = b.dir;
    let row = (row + dr * round - 1).rem_euclid(height) + 1;
    let col = (col + dc * round - 1).rem_euclid(width) + 1;
    (row, col)
}

static DIRS: [(i32, i32); 5] = [ (0,1), (1, 0), (0,-1), (-1, 0), (0,0), ];
fn all_moves_orig(game: &Game, pos: &Point) -> Vec<Point> {
    DIRS.iter()
        .map(|d| mov(*pos, *d))
             .filter(|p|
            (p.0 > 0 && p.0 <= game.height && p.1 > 0 && p.1 <= game.width) ||
            // (p.0 == 0 && p.1 == game.start) ||
            (p.0 == game.height+1 && p.1 == game.finish) )
        .collect()
}
fn all_moves(game: &Game, state: &State) -> Vec<State> {
    println!("--- {:?} ---", (state.round, state.pos));
    let blizz_pos = &game.blizz_pos[(state.round + 1) % game.lcm];
    let points:Vec<_> = all_moves_orig(game, &state.pos).into_iter()
            .filter(|p| !blizz_pos.contains(&p)).collect();

    let p3 = state.round % game.lcm;
    let pos = (p3 as i32, state.pos.0, state.pos.1);
    let round = state.round + 1;
    let states:Vec<Point3> = game.edges[&pos].clone();
    println!("states: {:?}", states.iter().map(|x| (x.1,x.2)).collect::<Vec<_>>());
    println!("points: {:?}", points);
    // game.edges[&pos].iter().map(|node| State { round, pos: (node.1, node.2)}).collect()
    points.into_iter().map(|p| State {pos: p, round: state.round + 1}).collect()
}

fn draw(game: &Game, state: &State) {
    let blizz_pos = &game.blizz_pos[state.round % game.lcm];
    for row in 0..=game.height+1 {
        print!("#");
        for col in 1..=game.width {
            let p = (row, col);
            print!("{}",
                if p == state.pos {"E"}
                else if row == 0 || (row == game.height+1 && col != game.finish) {"#"}
                else if blizz_pos.contains(&p) {"X"}
                else {"."});
        }
        println!("#");
    }
}


fn manhattan(a: &Point, b: &Point) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}


fn astar_score(game: &Game, state: &State) -> usize {
    manhattan(&state.pos, &(game.height+1, game.finish))
}

fn astar_heuristic(game: &Game, state: &State) -> Vec<State> {
    // Do we need to catch loops?
    // if memo.contains_key(&state) {
    //     // We saw this movie before
    //     memo[&state]

    all_moves(game, &state)
}

type Item = (usize, State);
type FringeQueue = Vec<Item>;

fn collect_fringe( game: &Game, nodes: Vec<State>, prev: &[Item]) -> FringeQueue {
    let mut new = nodes.into_iter().map(|state|
            ( state.round + astar_score(game, &state), state) )
        .collect::<Vec<Item>>();
    new.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    new.dedup();
    vec![prev.to_vec().into_iter(), new.into_iter()].into_iter().kmerge().collect()
}

fn astar(game: &Game, state: State) -> usize {
    let mut fringe = collect_fringe (game, vec![state], &[]);

    let mut count = 0;
    let beam = 1000000;
    loop {
        let (_, state) = &fringe[0];
        if count % 1 == 0 {
            println!("Round {}: qd={}", state.round, fringe.len()+1);
        }
        count += 1;
        if state.pos == (game.height+1, game.finish) {
            return state.round;
        }
        let next_paths:Vec<State> = fringe.iter().take(beam)
            .flat_map(|(_, state)| astar_heuristic(game, &state)).collect();
        fringe = collect_fringe(game, next_paths, &vec![]);
    }
}

// DFS
fn dfs(game: &Game, state: State, visited: &mut Path, memo: &mut Memo, best: &mut usize) -> Option<usize> {

    if state.round == 0 {
        println!("{:?} {}", state.pos, visited.len());
    }
    if state.pos == (game.height+1, game.finish) {
        // println!("Round {}: {:?}", state.round, &visited.iter().map(|x| x.pos).collect::<Vec<Point>>());
        // if visited.len() == 18 {
        //     let mut path = visited.iter().collect::<Vec<_>>();
        //     path.sort_by(|x,y| x.round.partial_cmp(&y.round).unwrap());
        //     for s in path {
        //         println!("\nMinute {}", s.round+1);
        //         draw(game, s);
        //     }
        // }
        Some(visited.len())
    } else if *best > 0 && *best < visited.len() {
        // Some other path did better already
        None
    } else {
        // Premature optimization is the root of all ... something.
        if memo.contains_key(&state) {
            // We saw this movie before
            memo[&state]
        } else if visited.contains(&state) {
            // We're walking in circles.  Die.
            None
        } else {
            // Get blizzard positions at time "now"
            let blizz_pos = &game.blizz_pos[state.round % game.lcm];

            // If blizzard collides with us, we die
            if blizz_pos.contains(&state.pos) {
                None // We died.
            } else {
                // We lived!  Try to move and continue on our quest tomorrow.
                visited.insert(state.clone());
                let round = (state.round + 1) % game.lcm;
                let result = all_moves(game, &state)
                    .into_iter()
                    .map(|state| dfs(game, state, visited, memo, best))
                    .filter_map(|x| x)
                    .min();
                visited.remove(&state);
                memo.insert(state, result);
                if let Some(val) = &result { if *val < *best { *best = *val; }}
                result
            }
        }
    }
}


fn solve(input: &'static str, part: usize) -> usize {

    // Intuition:  We can memoize the game positions and avoid repeating them like this:
    //   The whole board repeats every lcm(width, height) rounds.
    //   Track the round number, mod it with the lcm and save it with our current position.
    //   If we ever reach that spot again, we can prune it as a cycle.

    let game = parse(input);
    dbg!(game.lcm);
    dbg!(game.width);
    dbg!(game.height);
    let start = State {
        pos: (0, game.start),
        round: 0,
    };

    // let mut memo: Memo = FnvHashMap::default();
    // let mut path: Path = FnvHashSet::default();
    // let mut best = 0;
    // let ans = dfs(&game, start, &mut path, &mut memo, &mut best);

    let ans = astar(&game, start);
    ans
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day24, part1)]
fn day24_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
// Uncomment next line when solution is ready
// #[aoc(day24, part2)]
fn day24_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
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
const _ANS2: usize = 2;