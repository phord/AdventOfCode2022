#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


//------------------------------ PARSE INPUT
// Nom reference: https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md
use nom::{
    IResult, //error::Error,
    bytes::complete::{tag},
    character::complete::{alpha1, multispace0, digit1, one_of},
    branch::alt,
};

// Parse monkey name and eat the colon
// "sllz: 4" => (" 4", "sllz")
fn parse_monkey_name(input: &'static str) -> IResult<&'static str, &'static str> {
    let (i, name) = alpha1(input)?;
    let (i, _) = tag(":")(i)?;
    Ok((i, name))
}

// Parse a number as an expression
// "123" -> Expr::Literal(123)
fn parse_expr_literal(input: &'static str) -> IResult<&'static str, Expr> {
    let (i, _) = multispace0(input)?;
    let (i, arg1) = digit1(i)?;
    Ok((i, Expr::Literal(arg1.parse().unwrap())))
}

// Parse a variable as an expression
// "abcd" -> Expr::Variable("abcd")
fn parse_expr_variable(input: &'static str) -> IResult<&'static str, Expr> {
    let (i, _) = multispace0(input)?;
    let (i, arg1) = alpha1(i)?;
    Ok((i, Expr::Variable(arg1)))
}

// Parse an operator
// "+" -> Op::Plus
fn parse_expr_operator(input: &'static str) -> IResult<&'static str, Op> {
    let (i, _) = multispace0(input)?;
    let (i, op) = one_of("+-*/")(i)?;
    let op = match op {
        '+' => Op::Plus,
        '-' => Op::Minus,
        '*' => Op::Times,
        '/' => Op::Divide,
        _ => panic!("How to error?"),
    };
    Ok((i, op))
}

// Parse a binary expression
// "abcd / xyzz" -> Expr::Some(Op::Divide, Expr::Variable("abcd"), Expr::Variable("xyzz"))
fn parse_expr_binary(input: &'static str) -> IResult<&'static str, Expr> {
    let (i, arg1) = parse_expr_variable(input)?;
    let (i, op) = parse_expr_operator(i)?;
    let (i, arg2) = parse_expr_variable(i)?;
    Ok((i, Expr::Some(op, Rc::new(RefCell::new(arg1)), Rc::new(RefCell::new(arg2)))))
}

// Parse any kind of expression
fn parse_expr(input: &'static str) -> IResult<&'static str, Expr> {
    alt((parse_expr_binary, parse_expr_literal))(input)
}


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
    Some(Op, Rc<RefCell<Expr>>, Rc<RefCell<Expr>>),
    Variable(&'static str),
    Literal(i64),
    Unknown(&'static str),
}

fn parse(input: &'static str) -> HashMap<&'static str, Expr> {
    input.lines()
        .map(|i| {
            let (i, name) = parse_monkey_name(i).unwrap();
            let (_, expr) = parse_expr(i).unwrap();
            (name, expr)
        }).collect()
}

//------------------------------ SOLVE
fn apply (a: &Expr, b: &Expr, op: &Op) -> Expr {
    let f = match op {
        Op::Plus => |x,y| x + y,
        Op::Minus => |x,y| x - y,
        Op::Times => |x,y| x * y,
        Op::Divide => |x,y| x / y,
        Op::Equals => unimplemented!(),
    };

    match (a, b) {
        (Expr::Literal(x), Expr::Literal(y)) => Expr::Literal(f(*x,*y)),
        (x,y) => Expr::Some(*op, Rc::new(RefCell::new(x.clone())), Rc::new(RefCell::new(y.clone()))),
    }
}

fn try_eval( eq: &Expr, vars: &HashMap<&'static str, Expr>) -> Expr {
    let result = match eq {
        Expr::Some(op, a, b) => apply(&try_eval(&*a.borrow(), vars), &try_eval(&*b.borrow(), vars), &op),

        Expr::Variable(x) => try_eval(&vars[x], &vars),
        Expr::Literal(x) => Expr::Literal(*x),
        Expr::Unknown(x) => Expr::Unknown(x),
    };
    result
}

fn try_invert( eq: &Expr, accum: i64, vars: &HashMap<&'static str, Expr>) -> i64 {

    match eq {
        // Some variable
        Expr::Variable(x) => try_invert(&vars[*x], accum, vars),

        // Only the unknown remains. accum must be its value.
        Expr::Unknown(_) => accum,

        // Some binary operator
        Expr::Some(op, left, right) => {
            let left = &*left.borrow();
            let right = &*right.borrow();

            // Either left or right expr has the unknown. Try to eval each to find out which does not.
            let left_eval = try_eval(&left, vars);
            let right_eval = try_eval(&right, vars);

            // Solve for the remaining unknown
            if let Expr::Literal(lval) = left_eval {
                // Left is known; Right is unknown
                match op {
                    Op::Plus => try_invert(right, accum - lval, vars),    // a + Y = X  -->  Y = X - a
                    Op::Minus => try_invert(right, lval - accum , vars),  // a - Y = X  -->  Y = -X + a
                    Op::Divide => try_invert(right, lval / accum , vars), // a / Y = X  -->  Y = a / X
                    Op::Times => try_invert(right, accum / lval, vars),   // a * Y = X  -->  Y = X / a
                    Op::Equals => try_invert(right, lval, vars),          // X == f(Y)
                }
            } else if let Expr::Literal(rval) = right_eval {
                // Right is known; Left is unknown
                match op {
                    Op::Plus => try_invert(left, accum - rval, vars),    // Y + a = X  -->  Y = X - a
                    Op::Minus => try_invert(left, accum + rval, vars),   // Y - a = X  -->  Y = X + a
                    Op::Divide => try_invert(left, accum * rval, vars),  // Y / a = X  -->  Y = a * X
                    Op::Times => try_invert(left, accum / rval, vars),   // Y * a = X  -->  Y = X / a
                    Op::Equals => try_invert(left, rval, vars),          // f(Y) == X
                }
            } else {
                panic!();
            }
        },
        _ => {
            dbg!(eq);
            panic!();
        },
    }
}

fn eval( eq: &Expr, vars: &HashMap<&'static str, Expr>) -> i64 {
    let e = try_eval(eq, vars);
    match e {
        Expr::Literal(x) => x,
        _ => panic!(),
    }
}

#[allow(unused)]
fn prt( eq: &Expr, vars: &HashMap<&'static str, Expr>) -> String {
    let result = match eq {
        Expr::Some(Op::Plus, a, b) => "(".to_string() + &prt(&*a.borrow(), vars) + "+" + &prt(&*b.borrow(), vars) + ")",
        Expr::Some(Op::Minus, a, b) => "(".to_string() + &prt(&*a.borrow(), vars) + "-" + &prt(&*b.borrow(), vars) + ")",
        Expr::Some(Op::Times, a, b) => "(".to_string() + &prt(&*a.borrow(), vars) + "*" + &prt(&*b.borrow(), vars) + ")",
        Expr::Some(Op::Divide, a, b) => "(".to_string() + &prt(&*a.borrow(), vars) + "/" +  &prt(&*b.borrow(), vars) + ")",
        Expr::Some(Op::Equals, a, b) => "(".to_string() + &prt(&*a.borrow(), vars) + "=" +  &prt(&*b.borrow(), vars) + ")",

        Expr::Variable(x) => prt(&vars[*x], vars),
        Expr::Literal(x) => format!("{}", x),
        Expr::Unknown(x) => x.to_string(),
    };
    result
}

#[test] fn test_day21_part1() { assert_eq!(solve1(_SAMPLE), 152); }
#[test] fn test_day21_part1a() { assert_eq!(prt(&Expr::Variable("root"), &parse(_SAMPLE)), "(((4+(2*(5-3)))/4)+((32-2)*5))"); }
#[test] fn test_day21_part2() { assert_eq!(solve2(_SAMPLE), 301); }

fn solve1(input: &'static str) -> i64 {
    eval(&Expr::Variable("root"), &parse(input))
}

fn solve2(input: &'static str) -> i64 {
    let mut sim = parse(input);

    // Replace humn variable with "unknown" to solve for
    sim.insert("humn", Expr::Unknown("humn"));

    // Find root equation and change it to Op::Equals
    match &sim["root"] {
        Expr::Some(_, left, right) => {
            let root = Expr::Some(Op::Equals, left.clone(), right.clone());
            try_invert(&root, 0, &sim)
        },
        _ => panic!(),
    }

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
    assert_eq!(ans, 3006709232464);
    ans
}

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