// use std::collections::HashSet;

use std::collections::HashSet;

extern crate test;

pub fn main(contents: String) -> u32 {
  brute_force(contents)
}

fn brute_force(contents: String) -> u32 {
  let map: Vec<Vec<char>> = contents.lines().map(|lines| lines.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
  let width = map[0].len();
  let height = map.len();
  let mut max = 0;
  for i in 0..width {
    max = max.max(heat_map(&map, width, height, (i, 0), Direction::South));
    max = max.max(heat_map(&map, width, height, (i, height-1), Direction::North));
  }
  for j in 0..height {
    max = max.max(heat_map(&map, width, height, (0, j), Direction::East));
    max = max.max(heat_map(&map, width, height, (width-1, j), Direction::West));
  }
  max
}

fn heat_map(map: &Vec<Vec<char>>, width: usize, height: usize, start: (usize, usize), start_dir: Direction) -> u32 {
  let mut passed: HashSet<((usize, usize), Direction)> = HashSet::new();
  let mut beams: Vec<((usize, usize), Direction)> = Vec::new();
  beams.push((start, start_dir));
  while let Some((pos, mut direction)) = beams.pop() {
    if !passed.insert((pos, direction)) {
      continue;
    }
    let tile = map[pos.1][pos.0];
    match (direction, tile) {
      (Direction::East | Direction::West, '|') => {
        if let Some(new_pos) = Direction::North.apply(pos, height, width) {beams.push((new_pos, Direction::North))};
        if let Some(new_pos) = Direction::South.apply(pos, height, width) {beams.push((new_pos, Direction::South))};
      },
      (Direction::North | Direction::South, '-') => {
        if let Some(new_pos) = Direction::East.apply(pos, height, width) {beams.push((new_pos, Direction::East))};
        if let Some(new_pos) = Direction::West.apply(pos, height, width) {beams.push((new_pos, Direction::West))};
      },
      (_, '\\' | '/') => {direction = direction.reflect(tile); if let Some(new_pos) = direction.apply(pos, height, width) {beams.push((new_pos, direction))}},
      _ => {if let Some(new_pos) = direction.apply(pos, height, width) {beams.push((new_pos, direction))}},
    }
  }
  passed.into_iter().map(|(pos, _)| pos).collect::<HashSet<(usize, usize)>>().len() as u32
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  North,
  South,
  East,
  West,
}

impl Direction {
  fn apply(&self, pos: (usize, usize), height: usize, width: usize) -> Option<(usize, usize)> {
    match self {
      Direction::North => if pos.1 == 0 {None} else {Some((pos.0, pos.1-1))},
      Direction::South => if pos.1 + 1 == width {None} else {Some((pos.0, pos.1+1))},
      Direction::West => if pos.0 == 0 {None} else {Some((pos.0-1, pos.1))},
      Direction::East => if pos.0 + 1 == height {None} else {Some((pos.0+1, pos.1))},
    }
  }

  fn reflect(&self, reflection: char) -> Direction {
    match (self, reflection) {
      (Direction::North, '/') => Direction::East,
      (Direction::South, '/') => Direction::West,
      (Direction::West, '/') => Direction::South,
      (Direction::East, '/') => Direction::North,
      (Direction::North, '\\') => Direction::West,
      (Direction::South, '\\') => Direction::East,
      (Direction::West, '\\') => Direction::North,
      (Direction::East, '\\') => Direction::South,
      _ => panic!("Bad times")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 16;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_16_b() {
    const EXAMPLE_ANSWER: u32 = 51;
    const ANSWER: Option<u32> = Some(8163);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_16_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
