use std::collections::{HashMap, VecDeque};

extern crate test;

pub fn main(contents: String) -> u32 {
  find_max_distance(contents)
}

#[derive(Clone, Copy)]
enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(Clone, Copy)]
enum Pipe {
  Ground,
  Starting,
  Link(Direction, Direction),
}

struct PathSegment {
  pipe: Pipe,
  max_val: u32,
}

fn find_max_distance(contents: String) -> u32 {
  let mut start_idx = (0, 0);
  let mut map = contents
    .lines()
    .enumerate()
    .flat_map(|(i, line)| {
      line.char_indices().map(|(j, c)| {
        if c == 'S' {
          start_idx = (i, j);
        }
        ((i, j), PathSegment{pipe: char_to_pipe(c), max_val: u32::MAX})
      }).collect::<Vec<((usize, usize), PathSegment)>>()
    })
    .collect::<HashMap<(usize, usize), PathSegment>>();

  // bfs
  let mut paths: VecDeque<((usize, usize), Direction, u32)> = VecDeque::new();

  if start_idx.0 > 0 {
    if let Some(path) = get_next_dir(Direction::North, start_idx, 0, &map) {
      paths.push_back(path);
    }
  }
  if let Some(path) = get_next_dir(Direction::East, start_idx, 0, &map) {
    paths.push_back(path);
  }
  if let Some(path) = get_next_dir(Direction::South, start_idx, 0, &map) {
    paths.push_back(path);
  }
  if start_idx.1 > 0 {
    if let Some(path) = get_next_dir(Direction::West, start_idx, 0, &map) {
      paths.push_back(path);
    }
  }

  while let Some((pos, next_dir, depth)) = paths.pop_front() {
    let segment = map.get_mut(&pos).unwrap();
    if segment.max_val <= depth {
      return segment.max_val;
    }
    segment.max_val = depth;
    if let Some(path) = get_next_dir(next_dir, pos, depth, &map) {
      paths.push_back(path);
    }
  }
  0
}

fn get_next_dir(dir: Direction, pos: (usize, usize), depth: u32, map: &HashMap<(usize, usize), PathSegment>) -> Option<((usize, usize), Direction, u32)> {
  let next_pos = match dir {
    Direction::North => (pos.0-1, pos.1),
    Direction::East => (pos.0, pos.1+1),
    Direction::South => (pos.0+1, pos.1),
    Direction::West => (pos.0, pos.1-1),
  };
  match (map.get(&next_pos).unwrap().pipe, dir) {
    (Pipe::Link(Direction::South, dir) | Pipe::Link(dir, Direction::South), Direction::North) |
    (Pipe::Link(Direction::West, dir) | Pipe::Link(dir, Direction::West), Direction::East) |
    (Pipe::Link(Direction::North, dir) | Pipe::Link(dir, Direction::North), Direction::South) |
    (Pipe::Link(Direction::East, dir) | Pipe::Link(dir, Direction::East), Direction::West) => Some((next_pos, dir, depth+1)),
    _ => None,
  }
}

fn char_to_pipe(c: char) -> Pipe {
  match c {
    '|' => Pipe::Link(Direction::North, Direction::South),
    '-' => Pipe::Link(Direction::West, Direction::East),
    'L' => Pipe::Link(Direction::North, Direction::East),
    'J' => Pipe::Link(Direction::North, Direction::West),
    '7' => Pipe::Link(Direction::South, Direction::West),
    'F' => Pipe::Link(Direction::South, Direction::East),
    '.' => Pipe::Ground,
    'S' => Pipe::Starting,
    _ => panic!("Error"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 10;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_10_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(8);
    const ANSWER: Option<u32> = Some(6786);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_10_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
