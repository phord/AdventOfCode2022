#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ PARSE INPUT
// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
// Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.

//  Each ore robot costs 2 ore.
//  Each clay robot costs 3 ore.
//  Each obsidian robot costs 3 ore and 8 clay.
//  Each geode robot costs 3 ore and 12 obsidian.

// Goal: Maximize geode robots
//  (2, 0, 0), (3, 0, 0), (3, 8, 0) , (3, 0, 12),
// Geode robot needs [3, 0, 12]
// Obsidian needs [3, 8, 0]  ->
//    -> Geode needs [6, 8, 12]
// Clay needs [3, 0, 0]
//      -> Obsidian needs [6, 8, 0]  ->
//          -> Geode needs [9, 8, 12]
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


fn nav( bp: &Vec<Vec<i32>>, ratio: &Vec<i32>, robots: &mut Vec<i32>, inv: &mut Vec<i32>, clock: i32) -> i32 {

    // What robots can I build next with the resources I'm already getting?
    // How long does it take to build each robot?
    let options:Vec<i32> = bp.iter().map(|plan| {
            time_needed(plan, &robots, &inv)
        })
        .collect();

    if clock > 20 {
        println!("{}  {:?} {:?}  {:?}", clock, &robots, &inv, &options);
    }
    if options.iter().all(|t| *t >= clock ) {
        // println!("Fini: {:?}", inv[3] + robots[3] * clock);
        // No more robots to build. Count total geodes.
        inv[3] + robots[3] * clock
    } else {
        // Heuristic: Build robots we need more of, and not ones we have too many of
        let robot_ratio = robots.iter().enumerate().map(|(i, x)| *x as f64 / ratio[i] as f64 ).collect::<Vec<f64>>();
        let avgr: f64 = robot_ratio.iter().filter(|x| **x > 0.0).sum::<f64>() / (robot_ratio.len()-1) as f64;
        let robot_scale = robot_ratio.iter().map(|x| *x as f64 / avgr ).collect::<Vec<f64>>();
        println!("{}  ratio: {:?} {:?}  {:?}  {:?}", clock, &robots, &ratio, &robot_ratio, &robot_scale);

        let options:Vec<_> = options.iter().enumerate()
            .filter(|(_, time)| **time <= clock)
            .filter(|(i, _)| *i == 3 || robot_scale[*i] < 5.0 )
            // .map(|(b, t)| (robots[b]/b, b, t))
            .collect();

        // Neither of these matter because we're still doing an exhaustive search :-(
        // heuristic: Build robots we have fewer of first
        // Heuristic: Build higher-order robots first
        // options.sort();
        options.iter()
        .map(| (bot, time) | {
            // Build a robot in the future.
            // We won't benefit until t+1, so skip ahead
            let time = **time + 1;
            for i in 0..4 {
                inv[i] +=  time * robots[i] - bp[*bot][i];
            }
            robots[*bot] += 1;
            let result = nav(bp, ratio, robots, inv, clock - time);
            robots[*bot] -= 1;
            for i in 0..4 {
                inv[i] -=  time * robots[i] - bp[*bot][i];
            }

            result
        }).max().unwrap()
    }
        // Wait to build it, then build it, then descend
}

#[test] fn test_day19_part1() { assert_eq!(solve1(), _ANS1); }

fn solve1() -> usize {
    // let bp = sample();
    let bp = parse();
    let clock = 24;
    let mut robots = vec![1, 0, 0, 0];
    let mut inventory = vec![0, 0, 0, 0];

    let mut total = 0;
    for (id, plan) in bp.iter().enumerate() {
        let plan = plan.iter().map(|tpl| vec![tpl.0, tpl.1, tpl.2, 0]).collect::<Vec<Vec<i32>>>();

        println!("Plan: {:?}", &plan);
        let score = nav(&plan, &vec![], &mut robots, &mut inventory, clock);
        let id = id+1;
        let quality = id as i32 * score;
        total += quality;
        println!("{}. Score: {}  Quality: {}  Total: {}", id, score, quality, total);
    }
    total as usize
}

#[test] fn test_day19_part2() { assert_eq!(solve2(), _ANS2); }

fn solve2() -> usize {
    let bp = sample();
    let bp = parse();
    let clock = 24; // 32;
    let mut robots = vec![1, 0, 0, 0];
    let mut inventory = vec![0, 0, 0, 0];

    let mut total = 1;
    for (id, plan) in bp.iter().enumerate() {
        let plan = plan.iter().map(|tpl| vec![tpl.0, tpl.1, tpl.2, 0]).collect::<Vec<Vec<i32>>>();
        let mut ratio = plan.iter().fold(vec![0,0,0,0], |a, t| a.iter().zip(t).map(|(x,y)| x+y).collect());
        ratio[3] = 10;
        println!("Plan: {:?}  ratio: {:?}", &plan, &ratio);
        let score = nav(&plan, &ratio, &mut robots, &mut inventory, clock);
        let id = id+1;
        total *= score;
        println!("{}. Score: {}  Total: {}", id, score, total);
    }
    total as usize
}


//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day19, part1)]
fn day19_part1(input: &'static str) -> usize {
    let ans = solve1();
    assert_eq!(ans, 33);
    ans
}

#[allow(unused)]
#[aoc(day19, part2)]
fn day19_part2(input: &'static str) -> usize {
    let ans = solve2();
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

const _ANS1: usize = 1;
const _ANS2: usize = 2;