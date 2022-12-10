#[allow(unused)]
use yaah::aoc;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<(&str, i32)> {
    input.lines()
        .map(|line| {
            let x: Vec<&str> = line.split(" ").collect();
            if x.len() < 2 {(x[0], 0)} else {(x[0], x[1].parse().unwrap())}
        })
        .collect()
}

//------------------------------ SOLVE

fn solve(input: &'static str) -> (i32, String) {
    let mut reg_x = 1;
    let mut cycle = 0;
    let mut sum = 0;

    let mut pos = 0;
    let mut screen: String = "\n".to_string();

    for op in parse(input) {
        #[allow(unused)]
        let mut cycles = 0;
        let mut next_x = reg_x;
        match op {
            ("noop", _) => cycles = 1,
            ("addx", value) => { cycles = 2; next_x = reg_x + value; }
            _ => panic!("Unknown opcode {:?}", op),
        };

        for _ in 0..cycles {
            screen += if (reg_x-pos).abs() < 2 { "#" } else {"."};
            cycle += 1;
            pos += 1;
            if pos % 40 == 0 {
                screen += "\n";
                pos = 0;
            }

            if cycle % 40 == 20 {
                sum += cycle * reg_x;
            }
        }
        reg_x = next_x;
    }
    // println!("{}", screen);
    (sum, screen)
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day10, part1)]
fn day10_part1(input: &'static str) -> i32 {
    let ans = solve(input);
    assert_eq!(ans, (13060, SOLN.to_string()));
    0
}

//------------------------------ TESTS

// #[test] fn test_day10_part1a() { assert_eq!(solve1(_SAMPLE0), _ANS1); }
#[test] fn test_day10_part1() { assert_eq!(solve(_SAMPLE), (_ANS1, _ANS2.to_string())); }

//------------------------------ SAMPLE DATA

const _SAMPLE0: &str = "noop
addx 3
addx -5";

const _SAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

const _ANS1: i32 = 13140;
const _ANS2: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

const SOLN: &str = "
####...##.#..#.###..#..#.#....###..####.
#.......#.#..#.#..#.#..#.#....#..#....#.
###.....#.#..#.###..#..#.#....#..#...#..
#.......#.#..#.#..#.#..#.#....###...#...
#....#..#.#..#.#..#.#..#.#....#.#..#....
#.....##...##..###...##..####.#..#.####.
";