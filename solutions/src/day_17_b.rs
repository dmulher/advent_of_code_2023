use std::{collections::{HashMap, hash_map::DefaultHasher, VecDeque}, hash::{Hash, Hasher}};

extern crate test;

pub fn main(contents: String) -> u32 {
  do_thing(contents)
}

fn do_thing(contents: String) -> u32 {
  let heat: Vec<Vec<u32>> = contents.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
  find_all_paths(&heat)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn travel(self, pos: (usize, usize), width: usize, height: usize) -> Option<(usize, usize)> {
    match self {
      Direction::Up => if pos.1 > 0 {Some((pos.0, pos.1 - 1))} else {None},
      Direction::Down => if pos.1 < height - 1 {Some((pos.0, pos.1 + 1))} else {None},
      Direction::Left => if pos.0 > 0 {Some((pos.0 - 1, pos.1))} else {None},
      Direction::Right => if pos.0 < width - 1 {Some((pos.0 + 1, pos.1))} else {None},
    }
  }

  fn opposite(self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

fn hash(pos: &(usize, usize), dir: &Direction, count: &u8) -> u64 {
  let mut h = DefaultHasher::new();
  pos.hash(&mut h);
  dir.hash(&mut h);
  count.hash(&mut h);
  h.finish()
}

fn find_all_paths(heat_map: &Vec<Vec<u32>>) -> u32 {
  let mut visited: HashMap<u64, u32> = HashMap::new();
  let mut queue: VecDeque<(usize, usize, Direction, u8, u32)> = VecDeque::new();
  let height = heat_map.len();
  let width = heat_map[0].len();
  let mut lowest_val: Option<u32> = None;
  queue.push_back((0, 0, Direction::Right, 0, 0));
  while let Some((x, y, direction, count, heat_val)) = queue.pop_front() {
    let this_hash = hash(&(x, y), &direction, &count);
    let this_heat = if x == 0 && y == 0 {0} else {heat_val + heat_map[x][y]};
    let can_turn = count >= 4;
    let must_turn = count == 10;


    if let Some(lv) = lowest_val {
      if this_heat > lv {
        continue;
      }
      if can_turn && x == width - 1 && y == height - 1 && this_heat < lv {
        lowest_val = Some(this_heat);
        continue;
      }
    } else if can_turn && x == width - 1 && y == height - 1 {
      lowest_val = Some(this_heat);
      continue;
    }

    if let Some(val) = visited.get(&this_hash) {
      if val <= &this_heat {
        continue;
      }
    }
    visited.insert(this_hash, this_heat);

    [Direction::Up, Direction::Left, Direction::Down, Direction::Right]
      .into_iter()
      .filter_map(|dir| {
        if dir.opposite() == direction {
          return None;
        }
        if let Some(new_pos) = dir.travel((x, y), width, height) {
          if (direction != dir && can_turn) || (direction == dir && !must_turn) {
            let new_dir_count = if direction == dir {count + 1} else {1};
            return Some((new_pos.0, new_pos.1, dir, new_dir_count, this_heat));
          }
        }
        None
      })
      .for_each(|new_check| queue.push_back(new_check));
  }
  lowest_val.unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 17;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_17_b() {
    const EXAMPLE_ANSWER: u32 = 94;
    const ANSWER: Option<u32> = None;
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_17_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
