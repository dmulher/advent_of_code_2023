extern crate test;

pub fn main(contents: String) -> u32 {
  get_total_score(contents)
}

fn get_total_score(contents: String) -> u32 {
  contents
    .lines()
    .map(|line| get_line_score(line))
    .sum()
}

fn get_line_score(line: &str) -> u32 {
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

  const DAY: u8 = 4;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_04_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(13);
    const ANSWER: Option<u32> = Some(26443);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_04_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
