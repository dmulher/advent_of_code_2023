use std::collections::{HashMap, VecDeque};

extern crate test;

pub fn main(contents: String) -> u32 {
  find_reachable_plots(contents)
}

fn find_reachable_plots(content: String) -> u32 {
  // Basically, does he reach a spot on an odd or even? He can end on evens, not on odds
  let map = content
    .lines()
    .map(|line| line.bytes().collect::<Vec<u8>>())
    .collect::<Vec<Vec<u8>>>();
  let start = map.iter().enumerate().filter_map(|(y, line)| {
    match line.iter().position(|c| *c == b'S') {
      Some(x) => Some((x, y)),
      None => None
    }
  }).next().unwrap();

  let height = map.len();
  let width = map[0].len();
  let directions = [Direction::North, Direction::South, Direction::West, Direction::East];

  let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
  let mut pathing: VecDeque<((usize, usize), u8)> = VecDeque::new();
  pathing.push_back((start, 0));

  let max_steps = 64;
  // We can worry about efficiency later
  while let Some((pos, steps)) = pathing.pop_front() {
    if visited.contains_key(&pos) {
      continue;
    }
    visited.insert(pos, steps % 2 == 0);
    if steps < max_steps {
      for dir in &directions {
        if let Some(new_pos) = dir.apply(pos, height, width) {
          if map[new_pos.1][new_pos.0] != b'#' {
            pathing.push_back((new_pos, steps + 1));
          }
        }
      }
    }
  }
  visited.into_iter().filter(|(_, v)| *v).count() as u32
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
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 21;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_21_a() {
    // const EXAMPLE_ANSWER: Option<u32> = Some(16);
    const EXAMPLE_ANSWER: Option<u32> = None;
    const ANSWER: Option<u32> = Some(3585);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_21_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
