use std::collections::HashMap;

extern crate test;

pub fn main(contents: String) -> u32 {
  get_steps(contents)
}

fn get_steps(contents: String) -> u32 {
  let mut lines = contents.lines();
  let mut instructions = lines.next().unwrap().chars().cycle();

  lines.next();

  let mut current_val = "AAA";
  let mut steps: u32 = 0;

  let map = lines.map(|line| {
    let mut split = line.split(" = ");
    let key = split.next().unwrap();
    let mut vals = split.next().unwrap()
      .strip_prefix('(').unwrap()
      .strip_suffix(')').unwrap()
      .split(", ");
    (key, (vals.next().unwrap(), vals.next().unwrap()))
  }).collect::<HashMap<&str, (&str, &str)>>();

  while current_val != "ZZZ" {
    steps += 1;
    let choices = map[current_val];
    current_val = match instructions.next().unwrap() {
      'R' => choices.1,
      'L' => choices.0,
      _ => panic!("Wrong instructions"),
    };
  }

  steps
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 8;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_08_a() {
    const EXAMPLE_ANSWER: u32 = 6;
    const ANSWER: Option<u32> = Some(12737);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_08_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
