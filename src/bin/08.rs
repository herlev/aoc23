use itertools::*;
use std::collections::HashMap;

fn parse_line(input: &str) -> (String, (String, String)) {
  let (from, rest) = input.split_once(" = ").unwrap();
  let (left, right) = rest.split_once(", ").unwrap();
  (from.into(), (left[1..].into(), right[..right.len() - 1].into()))
}

fn part1(input: &str) -> u32 {
  let (dirs, map) = input.split_once("\n\n").unwrap();
  let mut dirs = dirs.chars().into_iter().cycle();
  let map = map.lines().map(parse_line).collect::<HashMap<_, _>>();
  let mut steps = 0;
  let mut current = "AAA".to_owned();
  while current != "ZZZ" {
    current = match dirs.next().unwrap() {
      'L' => map.get(&current).unwrap().0.clone(),
      'R' => map.get(&current).unwrap().1.clone(),
      _ => panic!(),
    };
    steps += 1;
  }

  steps
}

fn part2(input: &str) -> u64 {
  let (dirs, map) = input.split_once("\n\n").unwrap();
  let mut dirs = dirs.chars().into_iter().cycle();
  let map = map.lines().map(parse_line).collect::<HashMap<_, _>>();
  let mut current_nodes = map.keys().sorted().cloned().filter(|k| k.ends_with("A")).collect_vec();
  let mut steps = 0;
  let mut dist_to_zs: Vec<Option<u64>> = vec![None; current_nodes.len()];
  while !dist_to_zs.iter().all(|d| d.is_some()) {
    let dir = dirs.next().unwrap();
    steps += 1;
    for (i, current) in current_nodes.iter_mut().enumerate() {
      if dist_to_zs[i].is_some() {
        continue;
      }
      *current = match dir {
        'L' => map.get(current).unwrap().0.clone(),
        'R' => map.get(current).unwrap().1.clone(),
        _ => panic!(),
      };
      if current.ends_with("Z") {
        dist_to_zs[i] = Some(steps);
      }
    }
  }
  dist_to_zs
    .into_iter()
    .filter_map(|d| d)
    .reduce(|a, b| num::integer::lcm(a, b))
    .unwrap()
}

fn main() {
  let input = include_str!("../../inputs/08.txt");
  let tinput = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  let tinput2 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)";
  println!("part2: {}", part2(tinput2));
  println!("part2: {}", part2(input));
}
