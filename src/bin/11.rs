use aoc23::{Grid2, Point2, PriorityQueue};
use itertools::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
enum Field {
  Space(u32),
  Galaxy(u32),
}

impl Default for Field {
  fn default() -> Self {
    Self::Space(1)
  }
}

fn parse(input: &str) -> Grid2<Field> {
  let mut gid = 0;
  let mut g = Grid2::from_char_grid(input, &mut |p, c| match c {
    '.' => Field::Space(1),
    '#' => {
      gid += 1;
      Field::Galaxy(gid)
    }
    _ => panic!(),
  });

  for y in g.y_range.clone() {
    if g
      .x_range
      .clone()
      .all(|x| matches!(g.at(x, y).unwrap(), Field::Space(_)))
    {
      g.x_range.clone().for_each(|x| match g.at_mut(x, y) {
        Some(Field::Space(n)) => *n = 2,
        _ => (),
      })
    }
  }
  for x in g.x_range.clone() {
    if g
      .y_range
      .clone()
      .all(|y| matches!(g.at(x, y).unwrap(), Field::Space(_)))
    {
      g.y_range.clone().for_each(|y| match g.at_mut(x, y) {
        Some(Field::Space(n)) => *n = 2,
        _ => (),
      })
    }
  }
  g
}

fn manhattan_dist(p1: Point2<i32>, p2: Point2<i32>) -> u32 {
  let d = (p1 - p2).abs();
  (d.x + d.y) as u32
}

fn shortest_path(g1: u32, g2: u32, g: &Grid2<Field>, gmap: &HashMap<u32, Point2<i32>>) -> u32 {
  let p1 = *gmap.get(&g1).unwrap();
  let p2 = *gmap.get(&g2).unwrap();
  let mut pq = PriorityQueue::new();
  let mut visited = HashSet::new();
  #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
  struct State {
    point: Point2<i32>,
    dist: u32,
    p: u32,
  }
  let pqe = |s: State| (s.p as usize, s);
  let initial = State {
    point: p1,
    dist: 0,
    p: 0,
  };
  pq.push(pqe(initial));

  while let Some(State { point: p, dist, p: _ }) = pq.pop() {
    if visited.contains(&p) {
      continue;
    }
    visited.insert(p);

    for n in p.neighbors_grid(
      g.x_range.clone().last().unwrap() as usize + 1,
      g.y_range.clone().last().unwrap() as usize + 1,
    ) {
      if n == p2 {
        return dist + 1;
      }
      let dist = dist
        + match g.at_point(&n).unwrap() {
          Field::Space(n) => *n,
          Field::Galaxy(_) => 1,
        };
      let expected_dist = dist + manhattan_dist(n, p2);
      let s = State {
        point: n,
        dist,
        p: expected_dist,
      };
      pq.push(pqe(s));
    }
  }
  todo!()
}

fn part1(input: &str) -> u32 {
  let mut g = parse(input);
  let mut gmap = HashMap::new();
  for y in g.y_range.clone() {
    for x in g.x_range.clone() {
      if let Some(Field::Galaxy(gid)) = g.at(x, y) {
        gmap.insert(*gid, Point2 { x, y });
      }
    }
  }

  // g.print(&|p, f| match f {
  //   Field::Space(n) => n.to_string().chars().next().unwrap(),
  //   Field::Galaxy(_) => '#',
  // });

  // dbg!(shortest_path(5, 2, &g));

  // dbg!(g);
  dbg!(gmap.keys().combinations(2).count());
  let mut i = 0;
  gmap
    .keys()
    .combinations(2)
    // .enumerate()
    .collect_vec()
    // .iter()
    .par_iter()
    .map(|v| (*v[0], *v[1]))
    .map(|(gid1, gid2)| shortest_path(gid1, gid2, &g, &gmap))
    .sum()
}

fn part2(input: &str) -> u32 {
  todo!()
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
  // println!("part2: {}", part2(tinput));
  // println!("part2: {}", part2(input));
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
