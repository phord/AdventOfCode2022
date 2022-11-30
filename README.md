# Advent of Code 2022 solutions

- Run the last solution with: `cargo run --release`
- Run all solutions with: `cargo run --release -- -a`

- Run the benchmarks with: `cargo bench --bench aoc-bench`


Solutions are in lib.rs.  Add a new solution like this:

    #[aoc(day1, part2)]
    fn day1_part2(_input: &'static str) -> usize {
        let ANS = 123;

        ANS
    }

Unit testing:

Store sample data in the code directly.  It's not available to download.

Use #\[test] to define tests for functions.

    static SAMPLE_DAY1: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    #[test]
    fn test_day1_part1() {
        let ans = day1_part1(SAMPLE_DAY1);
        assert_eq!(ans, 7);
    }


Boilerplate to add a whole day:

    //----------------------------------------------
    #[aoc(day3, part1)]
    fn day3_part1(_input: &'static str) -> u64 {
        0
    }

    #[aoc(day3, part2)]
    fn day3_part2(_input: &'static str) -> u64 {
        0
    }

    #[allow(unused)]
    const SAMPLE_DAY3: &str = "";

    #[test]
    fn test_day3_part1() {
        let ans = day3_part1(SAMPLE_DAY3);
        assert_eq!(ans, 150);
    }

    #[test]
    fn test_day3_part2() {
        let ans = day3_part2(SAMPLE_DAY3);
        assert_eq!(ans, 150);
    }
