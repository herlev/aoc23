use itertools::*;

fn extrapolate(mut v: Vec<i64>) -> i64 {
  let mut last_vals = vec![*v.last().unwrap()];
  while v.iter().any(|&n| n != 0) {
    v = v.iter().tuple_windows().map(|(l, r)| r - l).collect_vec();
    last_vals.push(*v.last().unwrap());
  }
  last_vals.into_iter().sum()
}

fn extrapolate_backwards(mut v: Vec<i64>) -> i64 {
  let mut first_vals = vec![v[0]];
  while v.iter().any(|&n| n != 0) {
    v = v.iter().tuple_windows().map(|(l, r)| r - l).collect_vec();
    first_vals.push(v[0]);
  }
  first_vals.into_iter().rev().reduce(|acc, n| n - acc).unwrap()
}

fn part1(input: &str) -> i64 {
  let historyies: Vec<Vec<i64>> = input
    .lines()
    .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect_vec())
    .collect_vec();
  historyies.into_iter().map(extrapolate).sum()
}

fn part2(input: &str) -> i64 {
  let historyies: Vec<Vec<i64>> = input
    .lines()
    .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect_vec())
    .collect_vec();
  historyies.into_iter().map(extrapolate_backwards).sum()
}

fn main() {
  let input = include_str!("../../inputs/09.txt");
  let tinput = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
}
