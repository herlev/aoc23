use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> u32 {
  let mut sum = 0;
  for line in input.lines() {
    let input = line.split_once(": ").unwrap().1;
    let (winning, you) = input.split_once(" | ").unwrap();
    let winning: HashSet<u32> = winning.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let you: HashSet<u32> = you.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let winning_numbers = winning.intersection(&you);
    let mut points = winning_numbers.count() as u32;
    if points > 0 {
      points = 2u32.pow(points - 1)
    }
    sum += points;
  }
  sum
}

fn part2(input: &str) -> u32 {
  let mut scratch_cards = HashMap::<u32, u32>::new();
  for (i, line) in input.lines().enumerate() {
    let game = i + 1;
    *scratch_cards.entry(game as u32).or_default() += 1;
    let input = line.split_once(": ").unwrap().1;
    let (winning, you) = input.split_once(" | ").unwrap();
    let winning: HashSet<u32> = winning.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let you: HashSet<u32> = you.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let winning_numbers = winning.intersection(&you);

    let num_cards = *scratch_cards.entry(game as u32).or_default();
    let points = winning_numbers.count();
    for i in game + 1..game + 1 + points {
      *scratch_cards.entry(i as u32).or_default() += num_cards;
    }
  }
  scratch_cards.values().sum()
}

fn main() {
  let input = include_str!("../../inputs/04.txt");
  let tinput = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
}
