#[allow(unused)]
use yaah::aoc;

fn get_dir_sizes(_input: &'static str) -> Vec<usize> {
    let mut size = Vec::new();
    let mut stack = Vec::new();
    for line in _input.lines() {
        if line.starts_with("$ cd ..") {
            let subdir_size = stack.pop().unwrap();
            *stack.last_mut().unwrap() += subdir_size;
            size.push(subdir_size);
        } else if line.starts_with("$ cd ") {
            stack.push(0);
        } else if let Ok(file_size) = line.split(" ").next().unwrap().parse::<usize>() {
            *stack.last_mut().unwrap() += file_size;
        }
    }

    stack.iter().rev().fold(0, |sum, &val| { size.push(sum + val); sum + val });
    size
}

fn solve(_input: &'static str, _part: usize) -> usize {
    let sizes = get_dir_sizes(_input);
    if _part == 1 {
        sizes.iter().filter(|x| **x <= 100000usize).sum()
    } else {
        let space = 70_000_000;
        let need = 30_000_000;
        let used = *sizes.iter().max().unwrap();
        let need = need + used - space;
        *sizes.iter().filter(|x| **x >= need).min().unwrap()
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