use std::iter::zip;

extern crate test;

pub fn main(contents: String) -> u32 {
  get_total_score(contents)
}

fn get_total_score(contents: String) -> u32 {
  let mut lines = contents.lines();
  let time: Vec<u32> = lines.next().unwrap().split_whitespace().skip(1).map(|t| t.parse::<u32>().unwrap()).collect();
  let distance: Vec<u32> = lines.next().unwrap().split_whitespace().skip(1).map(|d| d.parse::<u32>().unwrap()).collect();

  zip(time, distance).map(get_possible_times).reduce(|acc, r| acc * r).unwrap()
}

fn get_possible_times(race: (u32, u32)) -> u32 {
  let (time, distance) = race;
  (0..time).map(|time_held_down| (time - time_held_down) * time_held_down).filter(|attempt| attempt > &distance).count() as u32
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 6;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_06_a() {
    const EXAMPLE_ANSWER: u32 = 288;
    const ANSWER: Option<u32> = Some(114400);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_06_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
