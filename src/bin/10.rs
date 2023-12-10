use aoc23::{Direction, Grid2, Point2};
use std::collections::{HashSet, VecDeque};

#[derive(Default, Clone, Debug, Copy)]
enum Field {
  #[default]
  Ground,
  UpDown,
  LeftRight,
  UpRight,
  UpLeft,
  DownLeft,
  DownRight,
}

impl Field {
  fn char(&self) -> char {
    match self {
      Self::Ground => '.',
      Self::UpDown => '|',
      Self::LeftRight => '-',
      Self::UpRight => 'L',
      Self::UpLeft => 'J',
      Self::DownLeft => '7',
      Self::DownRight => 'F',
    }
  }
  pub fn dirs(&self) -> impl Iterator<Item = Direction> {
    match self {
      Self::UpDown => vec![Direction::Up, Direction::Down],
      Self::LeftRight => vec![Direction::Left, Direction::Right],
      Self::UpRight => vec![Direction::Up, Direction::Right],
      Self::UpLeft => vec![Direction::Up, Direction::Left],
      Self::DownLeft => vec![Direction::Down, Direction::Left],
      Self::DownRight => vec![Direction::Down, Direction::Right],
      Self::Ground => vec![],
    }
    .into_iter()
  }
}

fn parse(input: &str) -> (Grid2<Field>, Point2<i32>, Vec<Point2<i32>>) {
  let mut start: Option<Point2<i32>> = None;
  let grid = Grid2::from_char_grid(input, &mut |p, c| {
    if c == 'S' {
      start = Some(p);
    }
    match c {
      'S' => Field::Ground,
      '.' => Field::Ground,
      '|' => Field::UpDown,
      '-' => Field::LeftRight,
      'L' => Field::UpRight,
      'J' => Field::UpLeft,
      '7' => Field::DownLeft,
      'F' => Field::DownRight,
      _ => unreachable!(),
    }
  });
  let start = start.unwrap();

  let mut connections = Vec::new();

  for dir in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
    let p = start + dir.to_point();
    let f = grid.at_point(&p);
    let connected = match (dir, f) {
      (Direction::Up, Some(Field::UpDown) | Some(Field::DownLeft) | Some(Field::DownRight)) => true,
      (Direction::Right, Some(Field::LeftRight) | Some(Field::UpLeft) | Some(Field::DownLeft)) => true,
      (Direction::Down, Some(Field::UpDown) | Some(Field::UpRight) | Some(Field::UpLeft)) => true,
      (Direction::Left, Some(Field::LeftRight) | Some(Field::UpRight) | Some(Field::DownRight)) => true,
      _ => false,
    };
    if connected {
      connections.push(p);
    }
  }
  (grid, start, connections)
}

fn part1(input: &str) -> u32 {
  let (grid, start, connections) = parse(input);
  let mut q = VecDeque::new();
  let mut visited = HashSet::new();
  visited.insert(start);
  for p in connections {
    q.push_back((1, p));
  }
  let mut max = 0;
  while !q.is_empty() {
    let (d, p) = q.pop_front().unwrap();
    if visited.contains(&p) {
      continue;
    }
    visited.insert(p);
    max = d;
    let f = grid.at_point(&p);
    if f.is_none() {
      continue;
    }
    let f = f.unwrap();
    for dir in f.dirs() {
      q.push_back((d + 1, p + dir.to_point()));
    }
  }
  max
}

fn part2(input: &str) -> u32 {
  let (grid, start, connections) = parse(input);
  let mut q = VecDeque::new();
  let mut visited = HashSet::new();
  visited.insert(start);
  q.push_back((start, connections[0]));

  let mut on_right = HashSet::new();
  let mut on_left = HashSet::new();
  let mut path = HashSet::new();
  let mut turn_sum: i32 = 0;
  path.insert(start);
  while !q.is_empty() {
    let (prev_p, p) = q.pop_front().unwrap();
    if visited.contains(&p) {
      continue;
    }
    visited.insert(p);
    let f = grid.at_point(&p);
    if f.is_none() {
      continue;
    }
    let f = f.unwrap();
    for dir in f.dirs() {
      let next_point = p + dir.to_point();
      if visited.contains(&next_point) {
        continue;
      }
      {
        use Direction::*;
        use Field::*;
        let turn = match (f, dir) {
          (UpDown | LeftRight, _) => 0,
          (UpRight, Up) | (DownLeft, Down) | (UpLeft, Left) | (DownRight, Right) => 1,
          _ => -1,
        };
        turn_sum += turn;
      }
      q.push_back((p, next_point));
    }
    // ----
    // dbg!(p);
    path.insert(p);
    let diff = p - prev_p;
    let point_to_right = p + Direction::from_point(diff).cw().to_point();
    let point_to_left = p + Direction::from_point(diff).ccw().to_point();
    on_right.insert(point_to_right);
    on_right.insert(prev_p + Direction::from_point(prev_p - p).ccw().to_point());
    on_left.insert(point_to_left);
    on_left.insert(prev_p + Direction::from_point(prev_p - p).cw().to_point());
  }
  let on_right: HashSet<_> = on_right.difference(&path).cloned().collect();
  let on_left: HashSet<_> = on_left.difference(&path).cloned().collect();
  match turn_sum.signum() {
    1 => floodfill_count(on_right, &grid, &path),
    -1 => floodfill_count(on_left, &grid, &path),
    _ => panic!(),
  }
}

fn floodfill_count(mut inside: HashSet<Point2<i32>>, grid: &Grid2<Field>, path: &HashSet<Point2<i32>>) -> u32 {
  let mut visited = HashSet::new();
  let mut q = VecDeque::new();
  for p in &inside {
    q.push_back(*p);
  }

  while !q.is_empty() {
    let p = q.pop_back().unwrap();
    if visited.contains(&p) {
      continue;
    }
    visited.insert(p);
    inside.insert(p);

    for p in p.neighbors() {
      if !path.contains(&p) && grid.at_point(&p).is_some() {
        q.push_back(p);
      }
    }
  }
  // grid.print(&|p, c| {
  //   if inside.contains(&p) {
  //     ' '
  //   } else {
  //     c.char()
  //   }
  // });
  inside.len() as u32
}

fn main() {
  let input = include_str!("../../inputs/10.txt");
  let tinput = ".....
.S-7.
.|.|.
.L-J.
.....";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  // assert_eq!(part2(tinput), 1);

  assert_eq!(
    part2(
      "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
    ),
    4
  );

  assert_eq!(
    part2(
      ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
    ),
    8
  );

  assert_eq!(
    part2(
      "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
    ),
    10
  );
}
