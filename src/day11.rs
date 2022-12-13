use yaah::aoc;

//------------------------------ PARSE INPUT

enum Operation {
    Plus(usize),
    Times(usize),
    Squared,
}

struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: usize,
    tmon: usize,
    fmon: usize,
}

#[allow(unused)]
fn sample() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98],
            op: Operation::Times(19),
            test: 23,
            tmon: 2,
            fmon: 3
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            op: Operation::Plus(6),
            test: 19,
            tmon: 2,
            fmon: 0
        },
        Monkey {
            items: vec![79, 60, 97],
            op: Operation::Squared,
            test: 13,
            tmon: 1,
            fmon: 3
        },
        Monkey {
            items: vec![74],
            op: Operation::Plus(3),
            test: 17,
            tmon: 0,
            fmon: 1
        },
    ]
}

fn parse(_: &'static str) -> Vec<Monkey> {
    vec![
    Monkey {
        items: vec![57, 58],
        op: Operation::Times(19),
        test: 7,
        tmon: 2,
        fmon: 3,
    },
    Monkey {
        items: vec![66, 52, 59, 79, 94, 73],
        op: Operation::Plus(1),
        test: 19,
        tmon: 4,
        fmon: 6,
    },
    Monkey {
        items: vec![80],
        op: Operation::Plus(6),
        test: 5,
        tmon: 7,
        fmon: 5,
    },
    Monkey {
        items: vec![82, 81, 68, 66, 71, 83, 75, 97],
        op: Operation::Plus(5),
        test: 11,
        tmon: 5,
        fmon: 2,
    },
    Monkey {
        items: vec![55, 52, 67, 70, 69, 94, 90],
        op: Operation::Squared,
        test: 17,
        tmon: 0,
        fmon: 3,
    },
    Monkey {
        items: vec![69, 85, 89, 91],
        op: Operation::Plus(7),
        test: 13,
        tmon: 1,
        fmon: 7,
    },
    Monkey {
        items: vec![75, 53, 73, 52, 75],
        op: Operation::Times(7),
        test: 2,
        tmon: 0,
        fmon: 4,
    },
    Monkey {
        items: vec![94, 60, 79],
        op: Operation::Plus(2),
        test: 3,
        tmon: 1,
        fmon: 6,
    }]
}

//------------------------------ SOLVE

fn solve(input: &'static str, part: usize) -> u64 {
    let mut gang = parse(input);
    // let mut gang = sample();
    let mut items: Vec<Vec<usize>> = gang.iter_mut().map(|m| m.items.drain(..).collect()).collect();
    let mm:usize = gang.iter().map(|x| x.test).product();

    let mut count: Vec<u64> = vec![0; items.len()];

    #[allow(unused)]
    for round in 0..if part==1 {20} else {10000} {
        for turn in 0..gang.len() {
            let m = &gang[turn];
            let mut new_items = items.clone();
            {
                new_items[turn] = Vec::new();
            }
            // println!("Monkey {}:", turn);
            for i in items[turn].iter() {
                // println!("Monkey inspects an item with a worry level of {}.", i);
                count[turn] += 1;
                let mut level = match m.op {
                    Operation::Plus(n) => i + n,
                    Operation::Times(n) => i * n,
                    Operation::Squared => i * i,
                } ;
                // match m.op {
                //     Operation::Plus(n) => println!("  Worry level increases by {} to {}.", n, level),
                //     Operation::Times(n) => println!("  Worry level is multiplied by {} to {}.", n, level),
                //     Operation::Squared => println!("  Worry level is multiplied by itself to {}.", level),
                // };

                if part == 1 {
                    level = level / 3;
                    // println!("  Monkey gets bored with item. Worry level is divided by 3 to {}.", level);
                }

                let next = if level % m.test == 0 { m.tmon } else { m.fmon };
                // println!("  Current worry level is {}divisible by {}.", if level % m.test == 0 {""} else {"not "}, m.test);
                // println!("  Item with worry level {} is thrown to monkey {}.", level, next);

                if part == 2 {
                    // let new_level = level.lcm(&modulo);
                    // println!("  Worry level is reduced from {} to {}.", level, new_level);
                    // level = new_level;
                    level %= mm;
                }

                new_items[next].push(level);
            }
            items = new_items;
        }

        // if round == 0 || round == 19 || (round+1) % (if part == 1 { 1 } else { 1000 }) == 0 {
        //     println!("== After round {} ==\n", round+1);
        //     for (n, c) in count.iter().enumerate() {
        //         println!("Monkey {} inspected items {} times.", n, c);
        //     }
        //     //         println!("After round {}, the monkeys are holding items with these worry levels:", round+1);
        //     // for (m, i) in items.iter().enumerate() {
        //     //     println!("Monkey {}: {:?}", m, i);
        //     // }
        //     println!();
        // }
    }
    // for (n, c) in count.iter().enumerate() {
    //     println!("Monkey {} inspected items {} times.", n, c);
    // }

    count.sort();
    count.reverse();
    let a=count[0];
    let b=count[1];
    // println!("Two most active monkeys inspected {} and {} items.  Score = {}", a, b, a*b);

    a*b
}

fn solve1(input: &'static str) -> u64 { solve(input, 1) }
fn solve2(input: &'static str) -> u64 { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day11, part1)]
fn day11_part1(input: &'static str) -> u64 {
    let ans = solve1(input);
    assert_eq!(ans, 50830);
    ans
}

#[allow(unused)]
#[aoc(day11, part2)]
fn day11_part2(input: &'static str) -> u64 {
    let ans = solve2(input);
    assert_eq!(ans, 14399640002);
    ans
}

//------------------------------ TESTS

#[test] fn test_day11_part1() { assert_eq!(solve1(_SAMPLE), _ANS1); }
#[test] fn test_day11_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "1234";

const _ANS1: u64 = 50830;
const _ANS2: u64 = 14399640002;