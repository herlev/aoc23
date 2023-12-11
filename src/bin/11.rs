use aoc23::{Grid2, Point2};
use itertools::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
enum Field {
  #[default]
  Space,
  Galaxy(u32),
}

fn manhattan_dist(p1: Point2<i32>, p2: Point2<i32>) -> u32 {
  let d = (p1 - p2).abs();
  (d.x + d.y) as u32
}

fn parse(input: &str, scale: u32) -> HashMap<u32, Point2<i32>> {
  let mut gmap = HashMap::new();
  let mut y_offset = 0;
  let mut gid = 0;
  let g = Grid2::from_char_grid(input, &mut |_, c| match c {
    '.' => Field::Space,
    '#' => {
      gid += 1;
      Field::Galaxy(gid)
    }
    _ => panic!(),
  });
  for y in g.y_range.clone() {
    let mut x_offset = 0;
    if g.x_range.clone().all(|x| matches!(g.at(x, y).unwrap(), Field::Space)) {
      y_offset += scale - 1;
      continue;
    }
    for x in g.x_range.clone() {
      if g.y_range.clone().all(|y| matches!(g.at(x, y).unwrap(), Field::Space)) {
        x_offset += scale - 1;
        continue;
      }
      if let Some(Field::Galaxy(gid)) = g.at(x, y) {
        gmap.insert(
          *gid,
          Point2 {
            x: x + x_offset as i32,
            y: y + y_offset as i32,
          },
        );
      }
    }
  }
  gmap
}

fn part1(input: &str) -> u32 {
  let gmap = parse(input, 2);

  gmap
    .values()
    .combinations(2)
    .map(|v| (*v[0], *v[1]))
    .map(|(p1, p2)| manhattan_dist(p1, p2))
    .sum()
}

fn part2(input: &str) -> u64 {
  let gmap = parse(input, 1_000_000);

  gmap
    .values()
    .combinations(2)
    .map(|v| (*v[0], *v[1]))
    .map(|(p1, p2)| manhattan_dist(p1, p2) as u64)
    .sum::<u64>()
}
fn main() {
  let input = include_str!("../../inputs/11.txt");
  let tinput = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  assert_eq!(part1(tinput), 374);
  assert_eq!(part1(input), 10173804);
  assert_eq!(part2(tinput), 82000210);
  assert_eq!(part2(input), 634324905172);
}
