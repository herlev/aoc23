use itertools::*;
// use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct Set {
  red: u32,
  green: u32,
  blue: u32,
}

#[derive(Debug)]
struct Game {
  id: u32,
  sets: Vec<Set>,
}

fn parse_set(set_str: &str) -> Set {
  let mut set = Set::default();
  for s in set_str.trim().split(", ") {
    let (amount, color) = s.split_once(" ").unwrap();
    let amount = amount.parse().unwrap();
    match color {
      "red" => set.red = amount,
      "green" => set.green = amount,
      "blue" => set.blue = amount,
      _ => panic!(),
    }
  }
  set
}

fn parse_game(game_str: &str) -> Game {
  let (left, right) = game_str.split_once(":").unwrap();
  let id: u32 = left.split_once(" ").unwrap().1.parse().unwrap();
  let sets = right.split(";").map(parse_set).collect_vec();
  Game { id, sets }
}

fn part1(input: &str) -> u32 {
  input
    .lines()
    .map(parse_game)
    .filter(|game| {
      game
        .sets
        .iter()
        .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
    })
    .map(|game| game.id)
    .sum()
}

fn minimum_set(g: Game) -> Set {
  let max_reds = g.sets.iter().map(|s| s.red).max().unwrap();
  let max_greens = g.sets.iter().map(|s| s.green).max().unwrap();
  let max_blues = g.sets.iter().map(|s| s.blue).max().unwrap();
  Set {
    red: max_reds,
    green: max_greens,
    blue: max_blues,
  }
}

fn part2(input: &str) -> u32 {
  input
    .lines()
    .map(parse_game)
    .map(minimum_set)
    .map(|s| s.red * s.green * s.blue)
    .sum()
}

fn main() {
  let input = include_str!("../../inputs/2.txt");
  let tinput = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
}

#[test]
fn test() {
  let input = "";
}
