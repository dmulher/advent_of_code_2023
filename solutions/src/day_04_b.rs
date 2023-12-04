use std::collections::VecDeque;

extern crate test;

pub fn sum_active_symbols(contents: String) -> u32 {
  let mut card_vals = contents
    .lines()
    .map(|line| get_line_score(line));
  let mut count = 0;
  let mut remaining_copies: VecDeque<u32> = VecDeque::new();
  while let Some(matches) = card_vals.next() {
    let copies = remaining_copies.pop_front().unwrap_or(1);
    count += copies;
    let matches_count = remaining_copies.len();
    for i in 0..(matches as usize) {
      if i >= matches_count {
        remaining_copies.push_back(copies + 1)
      } else {
        remaining_copies[i] += copies;
      }
    }
  }
  count
}

fn get_line_score(line: &str) -> u32 {
  let mut map = line.split(':').skip(1).next().unwrap().split('|')
    .map(|half| half.trim().split_whitespace().map(|num| num.parse::<u8>().unwrap()).collect::<Vec<u8>>());
  let left: Vec<u8> = map.next().unwrap();
  let right: Vec<u8> = map.next().unwrap();
  let mut matches: u32 = 0;
  for right_num in right {
    if left.contains(&right_num) {
      matches += 1;
    }
  }
  matches
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const FILE_NAME: &str = "inputs/day_04_a.txt";
  const TASK_NAME: &str = "day_04_b";

  #[test]
  fn test_day_04_b() {
    const ITERATIONS: u128 = 1;
    const ANSWER: Option<u32> = Some(6284877);
    utils::run_method::<u32>(&sum_active_symbols, FILE_NAME, ITERATIONS, ANSWER, TASK_NAME);
  }

  #[bench]
  fn bench_day_04_b(b: &mut Bencher) {
    let input = read_file_to_string(FILE_NAME);
    b.iter(|| sum_active_symbols(input.clone()));
  }
}