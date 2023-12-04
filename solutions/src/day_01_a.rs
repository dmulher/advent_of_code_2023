extern crate test;

pub fn main(contents: String) -> u32 {
  get_calibration(contents)
}

fn get_calibration(contents: String) -> u32 {
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

  const DAY: u8 = 1;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_01_a() {
    const EXAMPLE_ANSWER: u32 = 142;
    const ANSWER: Option<u32> = Some(54630);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_01_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
