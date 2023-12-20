use std::collections::HashMap;

extern crate test;

pub fn main(contents: String) -> u64 {
  get_steps(contents)
}

fn get_steps(contents: String) -> u64 {
  let mut lines = contents.lines();
  let instruction_str = lines.next().unwrap();
  let instructions = instruction_str.chars().cycle();

  lines.next();

  let mut all_starts: Vec<&str> = Vec::new();

  let map = lines.clone().map(|line| {
    let mut split = line.split(" = ");
    let key = split.next().unwrap();
    if key.ends_with('A') {
      all_starts.push(key);
    }
    let mut vals = split.next().unwrap()
      .strip_prefix('(').unwrap()
      .strip_suffix(')').unwrap()
      .split(", ");
    (key, (vals.next().unwrap(), vals.next().unwrap()))
  }).collect::<HashMap<&str, (&str, &str)>>();

  all_starts
    .into_iter()
    .map(|mut val| {
      let mut from_here = instructions.clone();
      let mut steps = 0u64;
      while !val.ends_with('Z') {
        steps += 1;
        let choices = map[val];
        val = match from_here.next().unwrap() {
          'R' => choices.1,
          'L' => choices.0,
          _ => panic!("Wrong instructions"),
        };
      }
      steps
    })
    .reduce(|acc, val| lcm(acc, val))
    .unwrap()
}

fn lcm(a: u64, b: u64) -> u64 {
  a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
  match b {
    0 => a,
    _ => gcd(b, a % b)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 8;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_08_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(6);
    const ANSWER: Option<u64> = Some(9064949303801);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_08_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
