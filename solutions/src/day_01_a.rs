extern crate test;

pub fn get_calibration(contents: String) -> u32 {
  contents
    .lines()
    .map(|line| line.chars().filter(|num| num.is_digit(10)))
    .map(get_first_and_last)
    .reduce(|acc, n| acc + n).unwrap()
}

fn get_first_and_last(mut digits: impl Iterator<Item = char>) -> u32 {
  match digits.next() {
    Some(first) => {
      let last = digits.last().unwrap_or(first);
      let mut first_string = String::from(first);
      first_string.push(last);
      first_string.parse().unwrap()
    },
    None => 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  #[test]
  fn it_works() {
    const ITERATIONS: u128 = 20;
    const FILE_NAME: &str = "inputs/day_01_a.txt";
    const ANSWER: Option<u32> = Some(54630);
    utils::run_method::<u32>(&get_calibration, FILE_NAME, ITERATIONS, ANSWER, "day 01 a");
  }

  #[bench]
  fn bench_day_01_a(b: &mut Bencher) {
    const FILE_NAME: &str = "inputs/day_01_a.txt";
    let input = read_file_to_string(FILE_NAME);
    b.iter(|| get_calibration(input.clone()));
  }
}
