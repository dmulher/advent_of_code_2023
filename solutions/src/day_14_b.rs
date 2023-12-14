use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

extern crate test;

pub fn main(contents: String) -> u64 {
  rocks(contents)
}

fn hashit(map_set: &Vec<Vec<u8>>) -> u64 {
  let mut h = DefaultHasher::new();
  map_set.hash(&mut h);
  h.finish()
}

fn rocks(contents: String) -> u64 {
  // let map = transpose(contents.lines().map(|line| line.bytes().rev().enumerate().collect::<Vec<(usize, u8)>>()).collect::<Vec<Vec<(usize, u8)>>>());
  let mut map_set: HashMap<u64, Vec<Vec<u8>>> = HashMap::new();
  let mut maps: Vec<u64> = Vec::new();
  let mut map = contents.lines().map(|line| line.bytes().collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
  let height = map.len();

  while !maps.contains(&hashit(&map)) {
    maps.push(hashit(&map));
    map_set.insert(hashit(&map), map.clone());
    rock_up(&mut map);
    rock_left(&mut map);
    rock_down(&mut map);
    rock_right(&mut map);
  }
  let loop_start = maps.iter().position(|m| m == &hashit(&map)).unwrap();
  let loop_end = maps.len();
  let loop_length = loop_end - loop_start;
  let remainder = (1000000000 - loop_start - 1) % loop_length;
  println!("Loop is from {loop_start} to {loop_end} (length {loop_length}), our final loop will leave {remainder} remaining");

  let final_map = map_set.get(&maps[loop_start + remainder + 1]).unwrap();

  final_map.into_iter().enumerate().rev().map(|(weight, line)| line.into_iter().filter(|r| **r == b'O').count() * (height - weight)).sum::<usize>() as u64
}

fn rock_up(rock_map: &mut Vec<Vec<u8>>) -> () {
  for j in 0..rock_map[0].len() {
    let mut rocks_in_waiting = 0;
    for i in (0..rock_map.len()).rev() {
      if rock_map[i][j] == b'#' && rocks_in_waiting > 0 {
        for x in 1..=rocks_in_waiting {
          rock_map[i+x][j] = b'O';
        }
        rocks_in_waiting = 0;
      } else if rock_map[i][j] == b'O' {
        rock_map[i][j] = b'.';
        rocks_in_waiting += 1;
      }
    }
    if rocks_in_waiting > 0 {
      for x in 0..rocks_in_waiting {
        rock_map[x][j] = b'O';
      }
    }
  }
}

fn rock_down(rock_map: &mut Vec<Vec<u8>>) -> () {
  for j in 0..rock_map[0].len() {
    let mut rocks_in_waiting = 0;
    for i in 0..rock_map.len() {
      if rock_map[i][j] == b'#' && rocks_in_waiting > 0 {
        for x in 1..=rocks_in_waiting {
          rock_map[i-x][j] = b'O';
        }
        rocks_in_waiting = 0;
      } else if rock_map[i][j] == b'O' {
        rock_map[i][j] = b'.';
        rocks_in_waiting += 1;
      }
    }
    if rocks_in_waiting > 0 {
      for x in (rock_map.len() - rocks_in_waiting)..rock_map.len() {
        rock_map[x][j] = b'O';
      }
    }
  }
}

fn rock_left(rock_map: &mut Vec<Vec<u8>>) -> () {
  for i in 0..rock_map.len() {
    let mut rocks_in_waiting = 0;
    for j in (0..rock_map[0].len()).rev() {
      if rock_map[i][j] == b'#' && rocks_in_waiting > 0 {
        for x in 1..=rocks_in_waiting {
          rock_map[i][j+x] = b'O';
        }
        rocks_in_waiting = 0;
      } else if rock_map[i][j] == b'O' {
        rock_map[i][j] = b'.';
        rocks_in_waiting += 1;
      }
    }
    if rocks_in_waiting > 0 {
      for x in 0..rocks_in_waiting {
        rock_map[i][x] = b'O';
      }
    }
  }
}

fn rock_right(rock_map: &mut Vec<Vec<u8>>) -> () {
  for i in 0..rock_map.len() {
    let mut rocks_in_waiting = 0;
    for j in 0..rock_map[0].len() {
      if rock_map[i][j] == b'#' && rocks_in_waiting > 0 {
        for x in 1..=rocks_in_waiting {
          rock_map[i][j-x] = b'O';
        }
        rocks_in_waiting = 0;
      } else if rock_map[i][j] == b'O' {
        rock_map[i][j] = b'.';
        rocks_in_waiting += 1;
      }
    }
    if rocks_in_waiting > 0 {
      for x in (rock_map[0].len()-rocks_in_waiting)..rock_map[0].len() {
        rock_map[i][x] = b'O';
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 14;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_14_b() {
    const EXAMPLE_ANSWER: u64 = 64;
    const ANSWER: Option<u64> = Some(96061);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_14_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
