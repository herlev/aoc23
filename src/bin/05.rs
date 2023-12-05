use itertools::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Mapping {
  start: i64,
  end: i64,
  offset: i64,
}

fn parse_mapping(input: &str) -> Mapping {
  let mut i = input.split_whitespace();
  let dest_range_start: i64 = i.next().unwrap().parse().unwrap();
  let src_range_start = i.next().unwrap().parse().unwrap();
  let len: i64 = i.next().unwrap().parse().unwrap();
  let offset = dest_range_start - src_range_start;
  let end = src_range_start + len;
  Mapping {
    start: src_range_start,
    end,
    offset,
  }
}

fn parse_map(input: &str) -> (String, Vec<Mapping>) {
  let mut i = input.lines();
  let name = i.next().unwrap().split_once(" ").unwrap().0.into();
  let mappings = i.map(parse_mapping).collect_vec();
  (name, mappings)
}

fn get_mapped_num(map: &Vec<Mapping>, num: i64) -> i64 {
  for mapping in map {
    if mapping.start <= num && num <= mapping.end {
      return num + mapping.offset;
    }
  }
  num
}

fn part1(input: &str) -> u32 {
  let (seeds, input) = input.split_once("\n\n").unwrap();
  let seeds: Vec<i64> = seeds
    .split_once(": ")
    .unwrap()
    .1
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect_vec();

  let maps: HashMap<_, _> = input.split("\n\n").map(parse_map).collect();

  let mut locations = Vec::new();
  for seed in seeds {
    let soil = get_mapped_num(maps.get("seed-to-soil").unwrap(), seed);
    let fertilizer = get_mapped_num(maps.get("soil-to-fertilizer").unwrap(), soil);
    let water = get_mapped_num(maps.get("fertilizer-to-water").unwrap(), fertilizer);
    let light = get_mapped_num(maps.get("water-to-light").unwrap(), water);
    let temperature = get_mapped_num(maps.get("light-to-temperature").unwrap(), light);
    let humidity = get_mapped_num(maps.get("temperature-to-humidity").unwrap(), temperature);
    let location = get_mapped_num(maps.get("humidity-to-location").unwrap(), humidity);
    // dbg!(soil, fertilizer, water, light, temperature, humidity, location);
    // println!("")
    locations.push(location);
  }

  // dbg!(seeds);
  // dbg!(maps);
  *locations.iter().min().unwrap() as u32
}

fn part2(input: &str) -> u32 {
  let (seeds, input) = input.split_once("\n\n").unwrap();
  let seeds: Vec<i64> = seeds
    .split_once(": ")
    .unwrap()
    .1
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect_vec();

  let maps: HashMap<_, _> = input.split("\n\n").map(parse_map).collect();

  let mut min_location = None;
  for (i, v) in seeds.chunks(2).enumerate() {
    dbg!(i);
    let (start, len) = (v[0], v[1]);
    for seed in start..(start + len) {
      let soil = get_mapped_num(maps.get("seed-to-soil").unwrap(), seed);
      let fertilizer = get_mapped_num(maps.get("soil-to-fertilizer").unwrap(), soil);
      let water = get_mapped_num(maps.get("fertilizer-to-water").unwrap(), fertilizer);
      let light = get_mapped_num(maps.get("water-to-light").unwrap(), water);
      let temperature = get_mapped_num(maps.get("light-to-temperature").unwrap(), light);
      let humidity = get_mapped_num(maps.get("temperature-to-humidity").unwrap(), temperature);
      let location = get_mapped_num(maps.get("humidity-to-location").unwrap(), humidity);
      // dbg!(soil, fertilizer, water, light, temperature, humidity, location);
      // println!("")
      min_location = match min_location {
        None => Some(location),
        Some(n) => Some(location.min(n)),
      };
    }
  }

  // dbg!(seeds);
  // dbg!(maps);
  dbg!(min_location.unwrap());
  min_location.unwrap() as u32
}

fn main() {
  let input = include_str!("../../inputs/05.txt");
  let tinput = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  // assert_eq!(part1(tinput), 0);
  // assert_eq!(part1(input), 0);
  // assert_eq!(part2(tinput), 0);
  // assert_eq!(part2(input), 0);
}

#[test]
fn test() {
  let input = "";
  // assert_eq!(part1(input), 0);
  // assert_eq!(part2(input), 0);
}
