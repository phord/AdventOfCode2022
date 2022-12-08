#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

//------------------------------ PARSE INPUT
#[allow(unused)]
pub fn split_to_words(_input: &'static str) -> Vec<Vec<&str>> {
    _input
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect::<Vec<Vec<&str>>>()
}

fn parse(_input: &'static str) -> Vec<Vec<&str>> {
    split_to_words(_input)
}

//------------------------------ SOLVE

fn get_state(_input: &'static str) -> Vec<usize> {
    let _inp = parse(_input);

    // Ingest the input
    let mut size = Vec::new();
    let mut stack = Vec::new();
    for line in _inp {
        match line[0] {
            "$" => {
                match line[1] {
                    "cd" => {
                        if line[2] == ".." {
                            let subdir_size = stack.pop().unwrap();
                            *stack.last_mut().unwrap() += subdir_size;
                            size.push(subdir_size);
                        } else {
                            // Descend into subdir
                            stack.push(0);
                        }
                    },
                    "ls" => { }
                    _ => panic!("Unknown command: {:?}", line)
                };

            },
            "dir" => { /* IGNORED */},
            x     => { // File size; accumulate in current dir
                *stack.last_mut().unwrap() += x.parse::<usize>().unwrap();
            }
            _ => panic!("Not handled: {:?}", line),
        }
    }

    let mut subdir_size = 0;
    for s in stack.iter().rev() {
        subdir_size += s;
        size.push(subdir_size);
    }
    size
}

fn solve(_input: &'static str, _part: usize) -> usize {
    let state = get_state(_input);
    if _part==1 {
        let ans = state.iter().filter(|x| **x <= 100000usize).sum();
        return ans;
    } else {
        let space = 70000000;
        let need = 30000000;
        let sizes = state;
        let used = sizes.iter().max().unwrap();
        let unused = space - *used;
        let need = need - unused;

        let ans:usize = *sizes.iter().filter(|x| **x >= need).min().unwrap();
        ans
    }
}

//------------------------------ PART 1

#[allow(unused)]
#[aoc(day7, part1)]
fn day7_part1(_input: &'static str) -> usize {
    let ans = solve(_input, 1);
    assert_eq!(ans, 919137);
    ans
}

#[test]
fn test_day7_part1() {
    assert_eq!(solve(_SAMPLE, 1), _ANS1);
}

//------------------------------ PART 2

#[allow(unused)]
#[aoc(day7, part2)]
fn day7_part2(_input: &'static str) -> usize {
    let ans = solve(_input, 2);
    assert_eq!(ans, 2877389);
    ans
}

#[test]
fn test_day7_part2() {
    assert_eq!(solve(_SAMPLE, 2), _ANS2);
}

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

const _ANS1: usize = 95437;
const _ANS2: usize = 24933642;