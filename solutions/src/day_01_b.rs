extern crate test;

pub fn get_calibration(contents: String) -> u32 {
  contents
    .lines()
    .map(get_first_and_last)
    .reduce(|acc, n| acc + n).unwrap()
}

fn get_first_and_last(line: &str) -> u32 {
  let mut blah: Vec<u32> = vec![];
  let mut patterns: Vec<String> = vec![];
  let mut chars = line.chars();

  while let Some(c) = chars.next() {
    if c.is_digit(10) {
      blah.push(c.to_digit(10).unwrap());
    } else {
      let mut new_patterns: Vec<String> = patterns.into_iter().map(|mut patt| {
        match (patt.as_str(), c) {
          ("o", 'n' ) | ("t", 'w' | 'h' ) | ("th", 'r' ) | ("thr", 'e' ) | ("f", 'o' | 'i' ) | ("fo", 'u') | ("fi", 'v') | ("s", 'i' | 'e' ) | ("se", 'v') | ("sev", 'e') | ("e", 'i') | ("ei", 'g') | ("eig", 'h') | ("n", 'i') | ("ni", 'n') => {patt.push(c); Some(patt)},
          ("on", 'e' ) => {blah.push(1); None},
          ("tw", 'o' ) => {blah.push(2); None},
          ("thre", 'e' ) => {blah.push(3); None},
          ("fou", 'r' ) => {blah.push(4); None},
          ("fiv", 'e' ) => {blah.push(5); None},
          ("si", 'x' ) => {blah.push(6); None},
          ("seve", 'n' ) => {blah.push(7); None},
          ("eigh", 't' ) => {blah.push(8); None},
          ("nin", 'e' ) => {blah.push(9); None},
          _ => None
        }
      }).filter(|patt| patt.is_some()).map(|patt| patt.unwrap()).collect();
      new_patterns.push(c.to_string());
      patterns = new_patterns;
    }
  }
  match blah.first() {
    None => 0,
    Some(first) => {
      let mut first_str = first.to_string();
      first_str.push_str(&blah.last().unwrap_or(first).to_string());
      first_str.parse().unwrap()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const FILE_NAME: &str = "inputs/day_01_a.txt";
  const TASK_NAME: &str = "day_01_b";

  #[test]
  fn test_day_01_b() {
    const ITERATIONS: u128 = 1;
    const ANSWER: Option<u32> = Some(54770);
    utils::run_method::<u32>(&get_calibration, FILE_NAME, ITERATIONS, ANSWER, TASK_NAME);
  }

  #[bench]
  fn bench_day_01_b(b: &mut Bencher) {
    let input = read_file_to_string(FILE_NAME);
    b.iter(|| get_calibration(input.clone()));
  }
}