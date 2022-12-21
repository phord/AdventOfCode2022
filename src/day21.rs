#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashMap;

//------------------------------ PARSE INPUT

#[derive(Clone, Copy, Debug)]
enum Op {
    Plus,
    Minus,
    Times,
    Divide,
}
#[derive(Clone, Debug)]
enum Expr {
    Some(Op, Box<Expr>, Box<Expr>),
    Variable(&'static str),
    Literal(i64),
    Unknown(&'static str),
}

fn parse(input: &'static str) -> HashMap<&'static str, Expr> {
    input.lines()
        .map(|x| x.split(" ").collect())
        .map(|l:Vec<&'static str>| {
        let monk = &l[0][..4];

        let eq = if l.len() == 2 {
            Expr::Literal(l[1].parse::<i64>().unwrap())
        } else {
            if l.len() == 4 {
                let a = Expr::Variable(l[1]);
                let b = Expr::Variable(l[3]);
                match l[2] {
                    "+" => Expr::Some(Op::Plus,Box::new(a),Box::new(b)),
                    "-" => Expr::Some(Op::Minus,Box::new(a),Box::new(b)),
                    "*" => Expr::Some(Op::Times,Box::new(a),Box::new(b)),
                    "/" => Expr::Some(Op::Divide,Box::new(a),Box::new(b)),
                    _ => unimplemented!(),
                }
            } else {
                unreachable!();
            }
        };
        (monk, eq)
    }).collect()
}

//------------------------------ SOLVE
#[test] fn test_day21_part1() { assert_eq!(solve1(_SAMPLE), 152); }
#[test] fn test_day21_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }


fn eval( eq: &Expr, vars: &HashMap<&'static str, Expr>) -> i64 {
    let result = match eq {
        Expr::Some(Op::Plus, a, b) => eval(&*a, vars) + eval(&*b, vars),
        Expr::Some(Op::Minus, a, b) => eval(&*a, vars) - eval(&*b, vars),
        Expr::Some(Op::Times, a, b) => eval(&*a, vars) * eval(&*b, vars),
        Expr::Some(Op::Divide, a, b) => eval(&*a, vars) / eval(&*b, vars),

        // Expr::Some(Op::Plus, a, b) => Expr::Some(Op::Plus, eval(&*a, vars), eval(&*b, vars)),
        // Expr::Some(Op::Minus, a, b) => Expr::Some(Op::Minus, eval(&*a, vars), eval(&*b, vars)),
        // Expr::Some(Op::Times, a, b) => Expr::Some(Op::Times, eval(&*a, vars), eval(&*b, vars)),
        // Expr::Some(Op::Divide, a, b) => Expr::Some(Op::Divide, eval(&*a, vars), eval(&*b, vars)),

        Expr::Variable(x) => eval(&vars[x], &vars),
        Expr::Literal(x) => *x,
        Expr::Unknown(_) => panic!(),
    };
    result
}



fn prt( eq: &Expr, vars: &HashMap<&'static str, Expr>) -> i64 {
    let result = match eq {
        Expr::Some(Op::Plus, a, b) => eval(&*a, vars) + eval(&*b, vars),
        Expr::Some(Op::Minus, a, b) => eval(&*a, vars) - eval(&*b, vars),
        Expr::Some(Op::Times, a, b) => eval(&*a, vars) * eval(&*b, vars),
        Expr::Some(Op::Divide, a, b) => eval(&*a, vars) / eval(&*b, vars),

        Expr::Variable(x) => eval(&vars[x], &vars),
        Expr::Literal(x) => *x,
        Expr::Unknown(_) => panic!(),
    };
    result
}

fn solve1(input: &'static str) -> i64 {
    let mut sim = parse(input);

    eval(&Expr::Variable("root"), &sim)
}

fn solve2(input: &'static str) -> i64 {
    let mut sim = parse(input);
    eval(&Expr::Variable("root"), &sim)
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day21, part1)]
fn day21_part1(input: &'static str) -> i64 {
    let ans = solve1(input);
    assert_eq!(ans, 63119856257960);
    ans
}

#[allow(unused)]
// #[aoc(day21, part2)]
fn day21_part2(input: &'static str) -> i64 {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ TESTS

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

const _ANS1: i64 = 1;
const _ANS2: i64 = 2;