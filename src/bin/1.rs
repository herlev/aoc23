use itertools::*;

const REPLACEMENTS: [(&str, &str); 9] = [
  ("one", "1"),
  ("two", "2"),
  ("three", "3"),
  ("four", "4"),
  ("five", "5"),
  ("six", "6"),
  ("seven", "7"),
  ("eight", "8"),
  ("nine", "9"),
];

fn first_and_last_to_num(nums: Vec<u32>) -> u32 {
  nums[0] * 10 + nums[nums.len() - 1]
}

fn get_num(s: &str) -> Option<u32> {
  if s.chars().nth(0).unwrap().is_digit(10) {
    return Some(s.chars().nth(0).unwrap().to_digit(10).unwrap());
  }
  for (pattern, replacement) in REPLACEMENTS.iter() {
    if s.starts_with(pattern) {
      return Some(replacement.parse().unwrap());
    }
  }
  None
}

fn replace(s: &str) -> Vec<u32> {
  (0..s.len()).filter_map(|i| get_num(&s[i..])).collect()
}

fn part1(input: &str) -> u32 {
  input
    .lines()
    .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect_vec())
    .map(first_and_last_to_num)
    .sum()
}

fn part2(input: &str) -> u32 {
  input.lines().map(replace).map(first_and_last_to_num).sum()
}

fn main() {
  let input = include_str!("../../inputs/1.txt");
  let tinput = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  let tinput2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
  println!("part2: {}", part2(tinput2));
  println!("part2: {}", part2(input));
}

#[test]
fn test() {
  let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
  assert_eq!(part1(input), 142);
}
