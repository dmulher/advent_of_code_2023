use std::collections::HashMap;

extern crate test;

pub fn main(contents: String) -> u32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 21;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_21_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(0);
    const ANSWER: Option<u32> = None;
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_21_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
