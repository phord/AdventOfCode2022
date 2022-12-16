#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use fnv::FnvHashMap;
use fnv::FnvHashSet;

//------------------------------ PARSE INPUT

#[derive(Debug)]
struct Valve {
    rate: usize,
    exits: Vec<&'static str>,
}

type Path = Vec<&'static str>;

fn find_all_paths(map: &Map, from: &str, accum: Vec<Path>) -> Vec<Path> {
    let more:Vec<Path> = accum.iter().map(|path| {
        let tail = path.last().unwrap();
        let valve = &map[tail];
        valve.exits.iter()
            .filter(|x| !path.contains(x))
            .map(|x| {
                let mut foo = path.clone();
                foo.push(x);
                foo
            })
            .collect::<Vec<Path>>()
    }).flatten()
      .collect();
    if more.len() == 0 {
        accum
    } else {
        let mut more = find_all_paths(map, from, more);
        more.extend(accum.into_iter());
        more
    }
}

fn find_path<'a>(all: &'a Vec<Path>, from: &'static str, to: &'static str) -> &'a Path {
    all.iter()
        .filter(|x| **x.last().unwrap() == *to)
        .map(|x| (x.len(), x))
        .min()
        .map(|(_,v)| v)
        .unwrap()
}

fn find_paths<'a>(map: &'a Map, from: &'static str, to: &Vec<&'static str>) -> Vec<Path> {
    let all: Vec<Path> = find_all_paths(map, from, vec![vec![from]]);
    to.iter().filter(|x| **x != from).map(move |dest| find_path(&all, from, dest).clone()).collect()
}
type PathMap = FnvHashMap<(&'static str, &'static str), Path>;
fn find_interesting_paths(map: &Map) -> PathMap {
    let interesting_valves:Vec<&str> = map.iter()
            .filter(|(name, valve)| **name == "AA" || valve.rate > 0)
            .map(|(key, _)| *key)
            .collect();

    interesting_valves.iter()
            .map(|from| find_paths(map, from, &interesting_valves))
            .flatten()
            .map(|x| ((*x.first().unwrap(), *x.last().unwrap()), x))
            .collect()
}

type Map = FnvHashMap<&'static str, Valve>;

fn parse(input: &'static str) -> Map {
    let s: Vec<Vec<&str>> = input.lines().map(|l| l.split(" ").collect()).collect();
    let mut map = FnvHashMap::default();
    for row in s {
        let name = row[1];
        let rate: Vec<&str> = row[4].split("=").collect();
        let rate: Vec<&str> = rate[1].split(";").collect();
        let rate = rate[0].parse::<usize>().unwrap();
        let exits = row[9..].iter().map(|t| &t[0..2]).collect();
        map.insert(name, Valve { rate, exits });
    }
    map
}

//------------------------------ SOLVE
#[test] fn test_day16_part1() { assert_eq!(solve1(_SAMPLE), 1651); } // 1651
fn new_pressure<'a>(map: &'a Map, open: &mut FnvHashSet<&'static str>) -> usize {
    if open.len() == 0 {
        // println!("No valves are open.");
        0
    } else {
        let press = open.iter().map(|x| map[x].rate).sum::<usize>();
        // let mut valves = open.iter().filter(|x| **x != "AA").collect::<Vec<_>>();
        // valves.sort();
        // println!("Valves {:?} are open releasing {} pressure.", valves, press);
        press
    }
}

fn nav<'a>(map: &'a Map, path_map: &'a PathMap, open: &mut FnvHashSet<&'static str>, path: &Path,
            room: &'static str,
            pressure: usize, clock: usize) -> Vec<usize> {
    // println!("== Minute {} ==", clock);
    let mut pressure = pressure + new_pressure(map, open);
    // println!("You move to valve {}.", room);
    if clock < 30 {
        if path.len() == 0 {
            // We arrived at a room at the end of a path.  Open the valve if it's closed.
            let mut clock = clock;
            let valve = &map[room];
            let to_open = !open.contains(room) && valve.rate > 0;
            if to_open {
                clock = clock + 1;
                // println!("== Minute {} ==", clock);
                pressure = pressure + new_pressure(map, open);
                // println!("You open valve {}.", room);
                open.insert(room);
            }

            // Find all remaining interesting paths to take.  Each path we take should lead to a closed valve.
            let paths:Vec<&Path> = path_map.iter()
                            .filter(|((from,to), _)| *from == room && !open.contains(to))
                            .map(|((_,_), path)| path)
                            .collect();

            let result = if paths.is_empty() {
                // println!("No more paths to closed valves");
                nav(&map, path_map, open, path, room, pressure, clock+1)
            } else {
                paths.iter().flat_map(|path| {
                        assert_eq!(room, *path.first().unwrap());
                        let room = path[1];
                        let path: Path = path[2..].iter().map(|x| *x).collect();
                        nav(&map, path_map, open, &path, room, pressure, clock+1)}).collect()
            };
            if to_open {
                // Close the valve before trying other paths
                open.remove(room);
            }
            result
        } else {
            // Continue on the pre-ordained path.
            let room = path.first().unwrap();
            let path = path[1..].iter().map(|x| *x).collect();
            nav(&map, path_map, open, &path, room, pressure, clock+1)
        }
    } else {
        vec![pressure]
    }
}

fn solve(input: &'static str, part: usize) -> usize {
    let map = parse(input);
    let mut open = FnvHashSet::default();
    open.insert("AA");
    let path: Path = vec![];
    let name = "AA";

    let paths = find_interesting_paths(&map);
    // dbg!(&paths);

    let mut result = nav(&map, &paths, &mut open, &path, name, 0, 0);
    result.sort();
    // dbg!(&result);
    let result = result.iter().max().unwrap();
    *result

}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day16, part1)]
fn day16_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // 1840 is too high.  1845 is too high.
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
// #[aoc(day16, part2)]
fn day16_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ TESTS

#[test] fn test_day16_part2() { assert_eq!(solve2(_SAMPLE), _ANS2); }

//------------------------------ SAMPLE DATA

const _SAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

const _ANS1: usize = 1;
const _ANS2: usize = 2;