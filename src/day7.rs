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
        .map(|s| s.split_whitespace()
                   .collect())
        .collect::<Vec<Vec<&str>>>()
}

fn parse(_input: &'static str) -> Vec<Vec<&str>> {
    split_to_words(_input)
}

//------------------------------ SOLVE

#[derive(Debug)]
enum DirItem<'a> {
    File(usize),
    Dir(Rc<RefCell<HashMap<&'a str, Rc<RefCell<DirItem<'a>>>>>>),
}

fn size(item: &DirItem) -> usize {
    match &*item {
        DirItem::File(x) => *x,
        DirItem::Dir(x) => {
            let tree = x.borrow();
            let mut total = 0usize;
            for (_, i) in &*tree {
                let item = i.borrow();
                total += size(&*item);
            }
            total
        }
    }
}

fn dir_sizes(item: &DirItem) -> Vec<usize> {
    let mut sizes = Vec::new();
    match &*item {
        DirItem::File(_) => {},
        DirItem::Dir(x) => {
            sizes.push(size(&*item));
            let tree = x.borrow();
            for (_, i) in &*tree {
                let item = i.borrow();
                let subdirs = dir_sizes(&*item);
                sizes.extend(subdirs.iter());
            }
        }
    }
    sizes
}


#[allow(unused)]
struct State<'a> {
    files: Rc<RefCell<DirItem<'a>>>,
    path: String,
    parent: Option<Box<State<'a>>>,
}

impl <'a> State<'a> {
    fn new(files: Rc<RefCell<DirItem<'a>>>, path: &'a str, parent: State<'a>) -> State<'a> {
        State {
            files,
            path: String::from(path),
            parent: Option::Some(Box::new(parent)),
        }
    }

    fn chdir(self, path: &'a str) -> State<'a> {
        match path {
            ".." => { *self.parent.unwrap() }
            "/" => { if self.parent.is_none() { self } else {self.parent.unwrap().chdir(path) } }
            _ => {
                // FIXME: See if directory already exists
                let mut map = Rc::new(RefCell::new(DirItem::Dir(Rc::new(RefCell::new(HashMap::new())))));
                let files = Rc::clone(&self.files);
                match &*files.borrow() {
                    DirItem::File(_) => panic!("File exists; expected dir {}", path),
                    DirItem::Dir(tree) => {
                        let mut tree = tree.borrow_mut();
                        let exists = tree.get_mut(path);
                        match exists {
                            None =>  {tree.insert(path, Rc::clone(&map)); },
                            Some(node) => { map = Rc::clone(node); },
                        };
                    },
                };
                return State::new(map, path, self);
            }
        }
    }

    fn insert_child(&self, path: &'a str, item: DirItem<'a>) {
        let files = Rc::clone(&self.files);
        match &*files.borrow() {
            DirItem::File(_) => panic!("expected dir; found file"),
            DirItem::Dir(tree) => {
                let mut tree = tree.borrow_mut();
                let exists = tree.get_mut(path);
                match exists {
                    None =>  {tree.insert(path, Rc::new(RefCell::new(item))); },
                    Some(node) => { panic!("Node exists: {:?}", *node) },
                };
            },
        };
    }

    fn traverse_all_dir_sizes(&self) -> Vec<usize> {
        let files = Rc::clone(&self.files);
        let item = &*files.borrow();
        match item {
            DirItem::File(_) => vec![],
            DirItem::Dir(_) => {
                dir_sizes(item)
            },
        }
    }
}

fn get_state(_input: &'static str) -> State {
    let _inp = parse(_input);

    let mut state = State {
            files: Rc::new(RefCell::new(DirItem::Dir(Rc::new(RefCell::new(HashMap::new()))))),
            path: String::from("/"),
            parent: Option::None,
    };

    // Ingest the input
    let mut ls = false;
    for line in _inp {
        match (ls, line[0]) {
            (_, "$") => {
                ls = false;
                match line[1] {
                    "cd" => {
                        state = state.chdir(line[2]);
                    },
                    "ls" => {
                        ls = true;
                    }
                    _ => panic!("Unknown command: {:?}", line)
                };

            },
            (true, "dir") => {
                state.insert_child(line[1], DirItem::Dir(Rc::new(RefCell::new(HashMap::new()))));
            }
            (true, x)     => {
                let item = DirItem::File(x.parse::<usize>().unwrap());
                state.insert_child(line[1], item);
            }
            _ => panic!("Not handled: {:?}", line),
        }
    }
    state.chdir("/")
}

fn solve(_input: &'static str, _part: usize) -> usize {
    let state = get_state(_input);
    if _part==1 {
        let ans = state.traverse_all_dir_sizes().iter().filter(|x| **x <= 100000usize).sum();
        return ans;
    } else {
        let space = 70000000;
        let need = 30000000;
        let sizes = state.traverse_all_dir_sizes();
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
    // assert_eq!(ans, _);
    ans
}

#[test]
fn test_day7_part1() {
    assert_eq!(day7_part1(_SAMPLE), _ANS1);
}

//------------------------------ PART 2

#[allow(unused)]
#[aoc(day7, part2)]
fn day7_part2(_input: &'static str) -> usize {
    let ans = solve(_input, 2);
    // assert_eq!(ans, ___);
    ans
}

#[test]
fn test_day7_part2() {
    assert_eq!(day7_part2(_SAMPLE), _ANS2);
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