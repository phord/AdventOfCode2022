#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashMap;
use std::rc::Rc;

//------------------------------ PARSE INPUT

#[derive(Clone, Copy, Debug)]
enum Op {
    Plus,
    Minus,
    Times,
    Divide,
    Equals,
}
#[derive(Clone, Debug)]
enum Expr {
    Some(Op, Rc<Expr>, Rc<Expr>),
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
                    "+" => Expr::Some(Op::Plus,Rc::new(a),Rc::new(b)),
                    "-" => Expr::Some(Op::Minus,Rc::new(a),Rc::new(b)),
                    "*" => Expr::Some(Op::Times,Rc::new(a),Rc::new(b)),
                    "/" => Expr::Some(Op::Divide,Rc::new(a),Rc::new(b)),
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


fn apply (a: &Expr, b: &Expr, f: fn(i64, i64) -> i64) -> Expr {
    match (a, b) {
        (Expr::Literal(x), Expr::Literal(y)) => Expr::Literal(f(*x,*y)),
        (x,y) => Expr::Some(Op::Times, Rc::new(x.clone()), Rc::new(y.clone())),
    }
}

fn try_eval( eq: &Expr, vars: &HashMap<&'static str, Expr>) -> Expr {
    let result = match eq {
        Expr::Some(Op::Plus, a, b) => apply(&try_eval(&*a, vars), &try_eval(&*b, vars), |x,y| x + y),
        Expr::Some(Op::Minus, a, b) => apply(&try_eval(&*a, vars), &try_eval(&*b, vars), |x,y| x - y),
        Expr::Some(Op::Times, a, b) => apply(&try_eval(&*a, vars), &try_eval(&*b, vars), |x,y| x * y),
        Expr::Some(Op::Divide, a, b) => apply(&try_eval(&*a, vars), &try_eval(&*b, vars), |x,y| x / y),
        Expr::Some(Op::Equals, _, _) => unimplemented!(),

        Expr::Variable(x) => try_eval(&vars[x], &vars),
        Expr::Literal(x) => Expr::Literal(*x),
        Expr::Unknown(x) => Expr::Unknown(x),
    };
    result
}


fn eval( eq: &Expr, vars: &HashMap<&'static str, Expr>) -> i64 {
    let e = try_eval(eq, vars);
    match e {
        Expr::Literal(x) => x,
        _ => panic!(),
    }
}



fn prt( eq: &Expr, vars: &HashMap<&'static str, Expr>) -> String {
    let result = match eq {
        Expr::Some(Op::Plus, a, b) => "( ".to_string() + &prt(&*a, vars) + " + " + &prt(&*b, vars) + " )",
        Expr::Some(Op::Minus, a, b) => "( ".to_string() + &prt(&*a, vars) + " - " + &prt(&*b, vars) + " )",
        Expr::Some(Op::Times, a, b) => "( ".to_string() + &prt(&*a, vars) + " * " + &prt(&*b, vars) + " )",
        Expr::Some(Op::Divide, a, b) => "( ".to_string() + &prt(&*a, vars) + " / " +  &prt(&*b, vars) + " )",
        Expr::Some(Op::Equals, a, b) => "( ".to_string() + &prt(&*a, vars) + " = " +  &prt(&*b, vars) + " )",

        Expr::Variable(x) => prt(&vars[*x], vars),
        Expr::Literal(x) => format!(" {} ", x),
        Expr::Unknown(x) => x.to_string(),
    };
    result
}

fn solve1(input: &'static str) -> i64 {
    let mut sim = parse(input);

    eval(&Expr::Variable("root"), &sim)
}
#[test] fn test_day21_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

fn solve2(input: &'static str) -> i64 {
    let mut sim = parse(input);

    sim.insert("humn", Expr::Unknown("humn"));

    match &sim["root"] {
        Expr::Some(_, a, b) =>  println!("{} == {}", prt(&try_eval(&*a, &sim), &sim), prt(&try_eval(&*b, &sim), &sim) ),
        _ => panic!(),
    };

    // println!("{}", prt(&Expr::Variable("root"), &sim));
    0
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
#[aoc(day21, part2)]
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