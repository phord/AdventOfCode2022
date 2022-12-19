#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT
// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
// Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
type Blueprint = Vec<(i32, i32, i32)>;
fn sample() -> Vec<Blueprint> {
    vec![
        vec![ (4, 0, 0), (2, 0, 0), (3, 14, 0),  (2, 0, 7),],
        vec![ (2, 0, 0), (3, 0, 0), (3, 8, 0) , (3, 0, 12),],
    ]
}

fn parse() -> Vec<Blueprint> {
    vec![
        vec![ (4, 0, 0), (4, 0, 0), (4, 12, 0), (4, 0, 19) ],
        vec![ (4, 0, 0), (4, 0, 0), (2, 11, 0), (2, 0, 7) ],
        vec![ (3, 0, 0), (3, 0, 0), (2, 13, 0), (3, 0, 12) ],
        vec![ (2, 0, 0), (3, 0, 0), (3, 18, 0), (2, 0, 19) ],
        vec![ (2, 0, 0), (4, 0, 0), (3, 19, 0), (4, 0, 13) ],
        vec![ (4, 0, 0), (4, 0, 0), (3, 7, 0), (4, 0, 11) ],
        vec![ (4, 0, 0), (4, 0, 0), (4, 15, 0), (4, 0, 17) ],
        vec![ (3, 0, 0), (4, 0, 0), (4, 13, 0), (3, 0, 7) ],
        vec![ (4, 0, 0), (4, 0, 0), (2, 12, 0), (3, 0, 15) ],
        vec![ (4, 0, 0), (3, 0, 0), (4, 18, 0), (4, 0, 11) ],
        vec![ (4, 0, 0), (4, 0, 0), (4, 8, 0), (2, 0, 15) ],
        vec![ (4, 0, 0), (3, 0, 0), (4, 8, 0), (3, 0, 7) ],
        vec![ (4, 0, 0), (3, 0, 0), (3, 10, 0), (2, 0, 10) ],
        vec![ (2, 0, 0), (3, 0, 0), (3, 13, 0), (2, 0, 20) ],
        vec![ (3, 0, 0), (4, 0, 0), (3, 19, 0), (3, 0, 8) ],
        vec![ (3, 0, 0), (3, 0, 0), (2, 16, 0), (2, 0, 18) ],
        vec![ (4, 0, 0), (4, 0, 0), (2, 9, 0), (3, 0, 19) ],
        vec![ (4, 0, 0), (4, 0, 0), (2, 11, 0), (4, 0, 8) ],
        vec![ (3, 0, 0), (4, 0, 0), (3, 12, 0), (3, 0, 17) ],
        vec![ (3, 0, 0), (3, 0, 0), (2, 14, 0), (3, 0, 17) ],
        vec![ (4, 0, 0), (4, 0, 0), (2, 15, 0), (3, 0, 16) ],
        vec![ (4, 0, 0), (4, 0, 0), (2, 16, 0), (4, 0, 16) ],
        vec![ (3, 0, 0), (4, 0, 0), (4, 19, 0), (4, 0, 11) ],
        vec![ (4, 0, 0), (4, 0, 0), (4, 18, 0), (4, 0, 9) ],
        vec![ (4, 0, 0), (3, 0, 0), (2, 17, 0), (3, 0, 16) ],
        vec![ (3, 0, 0), (4, 0, 0), (2, 20, 0), (4, 0, 7) ],
        vec![ (2, 0, 0), (2, 0, 0), (2, 8, 0), (2, 0, 14) ],
        vec![ (3, 0, 0), (4, 0, 0), (3, 20, 0), (3, 0, 14) ],
        vec![ (4, 0, 0), (3, 0, 0), (4, 20, 0), (4, 0, 8) ],
        vec![ (3, 0, 0), (4, 0, 0), (4, 18, 0), (3, 0, 13) ],
    ]
}

//------------------------------ SOLVE

fn time_needed(cost: &Vec<i32>, robots: &Vec<i32>, inv: &Vec<i32>) -> i32 {
    (0..4).map(|i| {
        let c = if cost[i] > inv[i] { cost[i] - inv[i] } else { 0 };
        let t = if robots[i] > 0 { (c + robots[i] - 1) / robots[i] } else { if c > 0 {99} else {0} };
        t
    }).max().unwrap()
}


fn nav( bp: &Vec<Vec<i32>>, robots: Vec<i32>, inv: Vec<i32>, clock: i32) -> i32 {

    // What robots can I build next with the resources I'm already getting?
    // How long does it take to build each robot?
    let options:Vec<i32> = bp.iter().map(|plan| {
            time_needed(plan, &robots, &inv)
        })
        .collect();

    if clock > 10 {
        println!("{}  {:?} {:?}  {:?}", clock, &robots, &inv, &options);
    }
    if options.iter().all(|t| *t >= clock ) {
        // println!("Fini: {:?}", inv[3] + robots[3] * clock);
        // No more robots to build. Count total geodes.
        inv[3] + robots[3] * clock
    } else {
        let mut options:Vec<_> = options.iter().enumerate()
            .filter(|(_, time)| **time <= clock)
            .map(|(b, t)| (robots[b], b, t))
            .collect();

        // heuristic: Build robots we have fewer of first
        options.sort();
        options.iter()
        .map(| (_, bot, time) | {
            // Build a robot in the future.
            // We won't benefit until t+1, so skip ahead
            let time = **time + 1;
            let finv:Vec<i32> = (0..4).map(|i| {
                inv[i] + time * robots[i] - bp[*bot][i]
            }).collect();

            let frob:Vec<i32> = (0..4).map(|i| {
                robots[i] + if i == *bot { 1 } else { 0 }
            }).collect();

            nav(bp, frob, finv, clock - time)
        }).max().unwrap()
    }
        // Wait to build it, then build it, then descend
}

#[test] fn test_day19_part1() { assert_eq!(solve1(_SAMPLE), _ANS1); }

fn solve(input: &'static str, part: usize) -> usize {
    let bp = sample();
    let clock = 24;
    let robots = vec![1, 0, 0, 0];
    let inventory = vec![0, 0, 0, 0];

    let mut total = 0;
    for (id, plan) in bp.iter().enumerate() {
        let plan = plan.iter().map(|tpl| vec![tpl.0, tpl.1, tpl.2, 0]).collect::<Vec<Vec<i32>>>();
        println!("Plan: {:?}", &plan);
        let score = nav(&plan, robots.clone(), inventory.clone(), clock);
        let id = id+1;
        let quality = id * score;
        total += quality;
        println!("{}. Score: {}  Quality: {}  Total: {}", id, score, quality, total);
    }
    0
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
// Uncomment next line when solution is ready
// #[aoc(day19, part1)]
fn day19_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
// Uncomment next line when solution is ready
// #[aoc(day19, part2)]
fn day19_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ TESTS

#[test] fn test_day19_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

const _ANS1: usize = 1;
const _ANS2: usize = 2;