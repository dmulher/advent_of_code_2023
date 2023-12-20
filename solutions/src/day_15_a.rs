extern crate test;

pub fn main(contents: String) -> u32 {
  hash(contents)
}

fn hash(contents: String) -> u32 {
  contents.split(',').map(|chunk| chunk.bytes().fold(0, |acc, c| (acc + c)*17) as u32).sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 15;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_15_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(1320);
    const ANSWER: Option<u32> = Some(509167);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_15_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
