use aoc23::{Grid2, Point2};
use core::panic;
use itertools::*;
use std::{
  collections::{HashMap, HashSet},
  default,
};

#[derive(Default, Clone, Debug)]
enum Field {
  #[default]
  None,
  Symbol,
  Digit(u8),
}

fn get_num(grid: &Grid2<Field>, part_number_coords: &mut HashSet<Point2<i32>>, point: Point2<i32>) -> u32 {
  let y = point.y;
  let mut left_x = point.x;
  while matches!(grid.at(left_x - 1, y), Some(Field::Digit(_))) {
    left_x -= 1;
  }
  let mut right_x = point.x;
  while matches!(grid.at(right_x + 1, y), Some(Field::Digit(_))) {
    right_x += 1;
  }

  let mut sum = 0;
  for x in left_x..=right_x {
    let p = Point2::new(x, y);
    part_number_coords.remove(&p);
    sum *= 10;
    sum += match grid.at_point(&p).unwrap() {
      Field::Digit(n) => *n as u32,
      _ => panic!(""),
    }
  }
  sum
}

fn part1(input: &str) -> u32 {
  // get symbol coordinates
  // get number coordinates
  // parse numbers, mark visited number coordinates with hashset
  // sum numbers
  let lines = input.lines().collect_vec();
  let width = lines[0].len();
  let height = lines.len();
  let mut grid: Grid2<Field> = Grid2::new(0..=(width - 1) as i32, 0..=(height - 1) as i32);
  for (y, line) in lines.iter().enumerate() {
    for (x, c) in line.chars().enumerate() {
      *grid.at_mut(x as i32, y as i32).unwrap() = match c {
        a if a.is_digit(10) => Field::Digit(a.to_digit(10).unwrap() as u8),
        '.' => Field::None,
        _ => Field::Symbol,
      }
    }
  }
  let symbol_coords = grid
    .iter()
    .filter(|p| matches!(grid.at_point(p), Some(Field::Symbol)))
    .collect_vec();

  let mut part_number_coords = symbol_coords
    .iter()
    .flat_map(|p| p.neighbors8())
    .filter(|p| matches!(grid.at_point(p), Some(Field::Digit(_))))
    .collect::<HashSet<_>>();

  let mut nums = Vec::new();
  while !part_number_coords.is_empty() {
    let p = *part_number_coords.iter().next().unwrap();
    part_number_coords.remove(&p);
    let n = get_num(&grid, &mut part_number_coords, p);
    nums.push(n);
  }
  nums.iter().sum()
}

fn get_num2(grid: &Grid2<Field>, visited: &mut HashSet<Point2<i32>>, point: Point2<i32>) -> u32 {
  let y = point.y;
  let mut left_x = point.x;
  while matches!(grid.at(left_x - 1, y), Some(Field::Digit(_))) {
    left_x -= 1;
  }
  let mut right_x = point.x;
  while matches!(grid.at(right_x + 1, y), Some(Field::Digit(_))) {
    right_x += 1;
  }

  let mut sum = 0;
  for x in left_x..=right_x {
    let p = Point2::new(x, y);
    visited.insert(p);
    sum *= 10;
    sum += match grid.at_point(&p).unwrap() {
      Field::Digit(n) => *n as u32,
      _ => panic!(""),
    }
  }
  sum
}

fn part2(input: &str) -> u32 {
  let lines = input.lines().collect_vec();
  let width = lines[0].len();
  let height = lines.len();
  let mut grid: Grid2<Field> = Grid2::new(0..=(width - 1) as i32, 0..=(height - 1) as i32);
  for (y, line) in lines.iter().enumerate() {
    for (x, c) in line.chars().enumerate() {
      *grid.at_mut(x as i32, y as i32).unwrap() = match c {
        a if a.is_digit(10) => Field::Digit(a.to_digit(10).unwrap() as u8),
        '*' => Field::Symbol,
        _ => Field::None,
      }
    }
  }
  let star_coords = grid
    .iter()
    .filter(|p| matches!(grid.at_point(p), Some(Field::Symbol)))
    .collect_vec();

  let mut gear_ratio_sum = 0;
  for sp in star_coords {
    let mut visited = HashSet::<Point2<i32>>::new();
    let mut part_numbers = Vec::new();
    for p in sp
      .neighbors8()
      .filter(|p| matches!(grid.at_point(p), Some(Field::Digit(_))))
    {
      if visited.contains(&p) {
        continue;
      }
      part_numbers.push(get_num2(&grid, &mut visited, p))
    }
    if part_numbers.len() == 2 {
      gear_ratio_sum += part_numbers.iter().product::<u32>();
    }
  }
  gear_ratio_sum
}

fn main() {
  let input = include_str!("../../inputs/03.txt");
  let tinput = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
}
