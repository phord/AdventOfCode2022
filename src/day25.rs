#[allow(unused)]
use yaah::aoc;

fn snafu(s: &'static str) -> usize {
    s.as_bytes().iter()
        .rev()
        .enumerate()
        .map(|(pos, ch)| num::pow(5, pos) * match ch {
            b'-' => -1,
            b'=' => -2,
            _ => (ch - b'0') as i64,
        }).sum::<i64>() as usize
}

fn to_snafu(x: usize) -> String {
    let mut s: String = "".to_string();
    let mut x = x;
    while x > 0 {
        s.push("012=-".chars().skip(x % 5).next().unwrap());
        x = (x+2) / 5;
    }
    let s = s.chars().rev().collect();
    s
}

fn parse(input: &'static str) -> Vec<usize> {
    input.lines().map(|x| snafu(x)).collect()
}

#[allow(unused)]
#[aoc(day25, part1)]
fn day25_part1(input: &'static str) -> String {
    let ans = to_snafu(parse(input).iter().sum());
    assert_eq!(ans, "2=000=22-0-102=-1001");
    ans
}

#[test] fn test_day25_snafu() {
    let dec = vec![ 1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37, ];
    assert_eq!(parse(_SAMPLE), dec);

    let mut it = _SAMPLE.lines();
    for x in dec {
        assert_eq!(to_snafu(x), it.next().unwrap());
    }
}

const _SAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
