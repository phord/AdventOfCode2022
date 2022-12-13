#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

use std::fmt;
use std::cmp::Ordering;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<&str> {
    input.lines().collect()
}


//------------------------------ SOLVE

#[derive(PartialEq)]
enum Token {
    Open,
    Close,
    Number(usize),
}

fn next_token(s: &str) -> (Token, &str) {
    if s.starts_with("[") {
        (Token::Open, &s[1..])
    } else if s.starts_with(",") {
        next_token(&s[1..])
    } else if s.starts_with("]") {
        (Token::Close, &s[1..])
    } else {
        let num: String = s.chars().take_while(|x| x.is_digit(10)).collect();
        (Token::Number(num.parse::<usize>().unwrap()), &s[num.len()..])
    }
}

#[derive(Clone)]
enum Node {
    List(Vec<Node>),
    Number(usize),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::List(x) => write!(f, "{:?}", x),
            Node::Number(x) => write!(f, "{}", x),
        }
    }
}

fn parse_subtree(s: &str) -> (Node, &str) {
    let mut branch = vec![];
    let mut s = s;
    loop {
        let (t, ss) = next_token(s);
        s = ss;
        let item = match t {
            Token::Open => { let (n,ss) = parse_subtree(s); s=ss; n},
            Token::Number(x) => Node::Number(x),
            Token::Close => { return (Node::List(branch), s); } ,
        };
        branch.push(item);
    }
}

fn parse_tree(s: &str) -> Node {
    let (t, ss) = next_token(s);
    assert!(t == Token::Open);
    let (ret, s) = parse_subtree(ss);
    assert!(s.len()==0);
    ret
}
fn cmp_list(left: &Vec<Node>, right: &Vec<Node>) -> Ordering {
    for (c, l) in left.iter().enumerate() {
        if c == right.len() {
            // println!("    - Right side ran out of items, so inputs are not in the right order");
            return Ordering::Greater; }
        else {
            match cmp_ex(l, &right[c]) {
                Ordering::Equal => {},
                Ordering::Less => {
                    // println!("    - Left side is smaller, so inputs are in the right order");
                    return Ordering::Less; }
                Ordering::Greater => {
                    // println!("    - Right side is smaller, so inputs are NOT in the right order");
                    return Ordering::Greater; }
                                // x => return x,
            }
        }
    }

    if right.len() > left.len() {
        // println!("- Left side ran out of items, so inputs are in the right order");
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn cmp_ex(left: &Node, right: &Node) -> Ordering {
    match (left, right) {
        (Node::List(l), Node::List(r)) => {
            // println!("   - Compare {:?} vs {:?}", &l, &r);
            cmp_list(l, r)},
        (Node::List(_l), Node::Number(_r)) => {
            // println!("   - Compare {:?} vs {:?}", &l, &r);
            // println!("   - Mixed types; convert right to [{}] and retry comparison", &r);
            cmp_ex(&left, &Node::List(vec![right.clone()]))},
        (Node::Number(_l), Node::List(_r)) => {
            // println!("   - Compare {:?} vs {:?}", &l, &r);
            // println!("   - Mixed types; convert left to [{}] and retry comparison", &l);
            cmp_ex(&Node::List(vec![left.clone()]), &right)},
        (Node::Number(l), Node::Number(r)) => {
            // println!("   - Compare {} vs {}", &l, &r);
            if l < r { Ordering::Less } else if l == r { Ordering::Equal } else { Ordering::Greater }
        }
    }
}

fn cmp(left: &Node, right: &Node) -> bool {
    match cmp_ex(left, right) {
        Ordering::Greater => false,
        _ => true,
    }
}

fn solve1(input: &'static str) -> usize {
    let pairs = parse(input);

    let mut it = pairs.into_iter();
    let mut count = 0;
    let mut sum = 0;
    loop {
        let left = it.next();
        if left.is_none() { break; }
        let left = left.unwrap();
        let right = it.next().unwrap();
        it.next();
        let l = parse_tree(left);
        let r = parse_tree(right);
        count += 1;
        let correct = cmp(&l, &r);
        if correct { sum += count; }
    }
    sum
}

fn solve2(input: &'static str) -> usize {
    let pairs = parse(input);

    let mut it = pairs.into_iter();
    let mut vec = Vec::new();
    loop {
        let left = it.next();
        if left.is_none() { break; }
        let left = left.unwrap();
        let right = it.next().unwrap();
        it.next();
        vec.push(parse_tree(left));
        vec.push(parse_tree(right));
    }
    let marker1 = parse_tree("[[2]]");
    let marker2 = parse_tree("[[6]]");
    vec.push(marker1.clone());
    vec.push(marker2.clone());

    vec.sort_by(|a,b| cmp_ex(a,b));

    let mut prod = 1;
    for (c,x) in vec.iter().enumerate() {
        if cmp_ex(x, &marker1) == Ordering::Equal
            || cmp_ex(x, &marker2) == Ordering::Equal {
                prod *= c+1;
            }
        // println!("{} {:?}", c, x);
    }
    prod
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day13, part1)]
fn day13_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    assert_eq!(ans, 6395);
    ans
}

#[allow(unused)]
#[aoc(day13, part2)]
fn day13_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    assert_eq!(ans, 117 * 213);
    ans
}

//------------------------------ TESTS

#[test] fn test_day13_part1() { assert_eq!(solve1(_SAMPLE), 13); }
#[test] fn test_day13_part2() { assert_eq!(solve2(_SAMPLE), 140); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

const _ANS1: usize = 1;
const _ANS2: usize = 2;