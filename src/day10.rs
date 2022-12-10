#[allow(unused)]
use yaah::aoc;

//------------------------------ PARSE INPUT

fn parse_to_timeline(input: &'static str) -> Vec<i32> {
    let mut reg = 1;
    input.lines()
        .flat_map(|line| {
            if line.starts_with("noop") { vec![reg] }
            else if line.starts_with("addx") {
                let x = vec![reg, reg];
                reg += line[5..].parse::<i32>().unwrap();
                x
            } else {
                panic!("Unexpected: {}", line);
            }
        }).collect()
}

//------------------------------ SOLVE

fn solve(input: &'static str) -> (i32, String) {
    let inp = parse_to_timeline(input);
    let sum = (20..=220).step_by(40).map(|i| inp[i-1] * (i as i32)).sum();
    let screen = inp.iter().enumerate()
                    .flat_map(|(i,x)| {
                        let i = (i % 40) as i32;
                        [   if i == 0 {"\n"} else {""},
                            if (x-i).abs() <= 1 {"#"} else {"."},
                        ]}).collect::<String>();

    (sum, screen+"\n")
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day10, part1)]
fn day10_part1(input: &'static str) -> i32 {
    dbg!(parse_to_timeline(_SAMPLE0));
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