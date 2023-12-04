extern crate test;

pub fn sum_active_symbols(contents: String) -> u32 {
  contents
    .lines()
    .map(|line| get_total_score(line))
    .sum()
}

fn get_total_score(line: &str) -> u32 {
  let mut map = line.split(':').skip(1).next().unwrap().split('|')
    .map(|half| half.trim().split_whitespace().map(|num| num.parse::<u8>().unwrap()).collect::<Vec<u8>>());
  let left: Vec<u8> = map.next().unwrap();
  let right: Vec<u8> = map.next().unwrap();
  let mut matches: u8 = 0;
  for right_num in right {
    if left.contains(&right_num) {
      matches += 1;
    }
  }
  match matches {
    0 => 0,
    _ => 2_u32.pow((matches as u32)-1)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const FILE_NAME: &str = "inputs/day_04_a.txt";
  const TASK_NAME: &str = "day_04_a";

  #[test]
  fn test_day_04_a() {
    const ITERATIONS: u128 = 1;
    const ANSWER: Option<u32> = Some(26443);
    utils::run_method::<u32>(&sum_active_symbols, FILE_NAME, ITERATIONS, ANSWER, TASK_NAME);
  }

  #[bench]
  fn bench_day_04_a(b: &mut Bencher) {
    let input = read_file_to_string(FILE_NAME);
    b.iter(|| sum_active_symbols(input.clone()));
  }
}