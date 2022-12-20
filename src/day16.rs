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

fn find_path<'a>(all: &'a Vec<Path>, to: &'static str) -> &'a Path {
    all.iter()
        .filter(|x| **x.last().unwrap() == *to)
        .map(|x| (x.len(), x))
        .min()
        .map(|(_,v)| v)
        .unwrap()
}

fn find_paths<'a>(map: &'a Map, from: &'static str, to: &Vec<&'static str>) -> Vec<Path> {
    let all: Vec<Path> = find_all_paths(map, from, vec![vec![from]]);
    to.iter().filter(|x| **x != from).map(move |dest| find_path(&all, dest).clone()).collect()
}
type PathMap = FnvHashMap<(&'static str, &'static str), usize>;
fn find_interesting_paths(map: &Map) -> PathMap {
    let interesting_valves:Vec<&str> = map.iter()
            .filter(|(name, valve)| **name == "AA" || valve.rate > 0)
            .map(|(key, _)| *key)
            .collect();

    interesting_valves.iter()
            .map(|from| find_paths(map, from, &interesting_valves))
            .flatten()
            .map(|x| ((*x.first().unwrap(), *x.last().unwrap()), x.len()))
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
#[test] fn test_day16_part1() { assert_eq!(solve1(_SAMPLE), 1651); }
#[test] fn test_day16_part2() { assert_eq!(solve2(_SAMPLE), 1707); }

fn nav<'a>(map: &'a Map, path_map: &'a PathMap, open: &mut FnvHashSet<&'static str>,
            room: &'static str, pressure: usize, time_remaining: usize) -> Vec<(FnvHashSet<&'static str>, usize)> {

    // Find all remaining interesting paths to take.  Each path leads to a closed valve.
    let paths:Vec<(&str, &usize)> = path_map.iter()
                    .filter(|((from,to), dist)| *from == room && !open.contains(to) && **dist < time_remaining)
                    .map(|((_, to), distance)| (*to, distance))
                    .collect();

    let orig = vec![(open.clone(), pressure)];
    if paths.is_empty() {
        orig
    } else{
        paths.iter().flat_map(|(dest, dist)| {
                assert!(**dist < time_remaining);
                let time_remaining = time_remaining - **dist;
                open.insert(dest);
                let pressure = pressure + time_remaining * map[dest].rate;
                let mut result = nav(&map, path_map, open, dest, pressure, time_remaining);
                open.remove(dest);
                result.extend(orig.clone().into_iter());
                result
            }).collect()
    }
}

use itertools::Itertools;

fn solve(input: &'static str, part: usize) -> usize {
    let map = parse(input);
    let mut open = FnvHashSet::default();
    // open.insert("AA");
    let name = "AA";

    let paths = find_interesting_paths(&map);
    let max_time = if part == 1 {30} else {26};

    let result = nav(&map, &paths, &mut open, name, 0, max_time);

    if part == 1 {
        let result = result.iter().map(|(_,x)| x).max().unwrap();
        *result
    } else {
        // dbg!(result.len());
        let mut bit = 1u32;
        let mut node_value: FnvHashMap<&'static str, u32> = FnvHashMap::default();
        for ((_, node), _) in paths {
            if !node_value.contains_key(node) {
                node_value.insert(node, bit);
                bit *= 2;
            }
        }

        let nsvalue = |set:FnvHashSet<&'static str>| set.iter().map(|s| node_value[s]).sum::<u32>();
        let mut best_map: FnvHashMap<u32, usize> = FnvHashMap::default();
        for (set, press) in result {
            let set = nsvalue(set);
            if best_map.contains_key(&set) {
                if best_map[&set] < press {
                    best_map.insert(set, press);
                }
            } else {
                best_map.insert(set, press);
            }
        }

        best_map.iter()
            .permutations(2)
            .filter(|pairs| (pairs[0].0 & pairs[1].0) == 0)
            .map(|pairs| pairs[0].1 + pairs[1].1)
            .max().unwrap()
    }
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day16, part1)]
fn day16_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    assert_eq!(ans, 1728);
    ans
}

#[allow(unused)]
#[aoc(day16, part2)]
fn day16_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    assert_eq!(ans, 2304);
    ans
}

//------------------------------ TESTS


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