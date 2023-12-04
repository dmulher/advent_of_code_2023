extern crate test;

pub fn main(contents: String) -> u32 {
  get_calibration(contents)
}

fn get_calibration(contents: String) -> u32 {
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

  const DAY: u8 = 1;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_01_b() {
    const EXAMPLE_ANSWER: u32 = 281;
    const ANSWER: Option<u32> = Some(54770);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_01_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
