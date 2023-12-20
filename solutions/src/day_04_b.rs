use std::collections::VecDeque;

extern crate test;

pub fn main(contents: String) -> u32 {
  get_total_score(contents)
}

fn get_total_score(contents: String) -> u32 {
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

  const DAY: u8 = 4;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_04_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(30);
    const ANSWER: Option<u32> = Some(6284877);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_04_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
