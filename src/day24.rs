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
    start: i32,
    finish: i32,
    width: i32,
    height: i32,
    bliz: Vec<Blizzard>,
    lcm: usize,
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

    Game {start, finish, height, width, bliz, lcm}
}

//------------------------------ SOLVE
#[derive(Eq, Hash, PartialEq, Clone)]
struct State {
    round: usize,
    pos: Point,
}
type Path = FnvHashSet<State>;
type Memo = FnvHashMap<State, Option<usize>>;
fn mov(pos: Point, dir: Point) -> Point {
    (pos.0 + dir.0, pos.1 + dir.1)
}
fn blizz_mov(game: &Game, state: &State, b: &Blizzard) -> Point {
    let (row, col) = b.pos;
    let (dr, dc) = b.dir;
    let row = (row + dr * state.round as i32 - 1).rem_euclid(game.height) + 1;
    let col = (col + dc * state.round as i32 - 1).rem_euclid(game.width) + 1;
    (row, col)
}

static DIRS: [(i32, i32); 5] = [ (0,1), (1, 0), (0,-1), (-1, 0), (0,0), ];
fn all_moves(game: &Game, pos: &Point) -> Vec<Point> {
    DIRS.iter()
        .map(|d| mov(*pos, *d))
        .filter(|p|
            (p.0 > 0 && p.0 <= game.height && p.1 > 0 && p.1 <= game.width) ||
            // (p.0 == 0 && p.1 == game.start) ||
            (p.0 == game.height+1 && p.1 == game.finish) )
        .collect()
}

fn draw(game: &Game, state: &State) {
    let blizz_pos = game.bliz.iter().map(|b| blizz_mov(game, &state, b)).collect::<FnvHashSet<Point>>();
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

fn nav(game: &Game, state: State, visited: &mut Path, memo: &mut Memo, best: &mut usize) -> Option<usize> {

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
            let blizz_pos = game.bliz.iter().map(|b| blizz_mov(game, &state, b)).collect::<FnvHashSet<Point>>();

            // If blizzard collides with us, we die
            if blizz_pos.contains(&state.pos) {
                None // We died.
            } else {
                // We lived!  Try to move and continue on our quest tomorrow.
                visited.insert(state.clone());
                let round = (state.round + 1) % game.lcm;
                let result = all_moves(game, &state.pos)
                    .into_iter()
                    .map(|pos| nav(game, State {round, pos}, visited, memo, best))
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
    let start = State {
        pos: (0, game.start),
        round: 0,
    };

    let mut memo: Memo = FnvHashMap::default();
    let mut path: Path = FnvHashSet::default();
    let mut best = 0;

    let ans = nav(&game, start, &mut path, &mut memo, &mut best);
    ans.unwrap()
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