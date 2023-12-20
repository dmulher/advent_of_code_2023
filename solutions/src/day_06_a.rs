use std::iter::zip;

extern crate test;

pub fn main(contents: String) -> u32 {
  get_total_score(contents)
}

// fn get_total_score(contents: String) -> u32 {
//   let mut lines = contents.lines().map(|line| line.split_whitespace().skip(1).map(|x| x.parse::<u32>().unwrap()));
//   zip(lines.next().unwrap(), lines.next().unwrap())
//     .map(|(time, distance)| {println!("{} - {}", get_possible_times((time, distance)), get_quadratic(time, distance)); get_possible_times((time, distance))})
//     .reduce(|acc, r| acc * r)
//     .unwrap() as u32
// }

// fn get_possible_times(race: (u32, u32)) -> u32 {
//   let (time, distance) = race;
//   (0..time).map(|time_held_down| (time - time_held_down) * time_held_down).filter(|attempt| attempt > &distance).count() as u32
// }

fn get_total_score(contents: String) -> u32 {
  let mut lines = contents.lines().map(|line| line.split_whitespace().skip(1).map(|x| x.parse::<u32>().unwrap()));
  zip(lines.next().unwrap(), lines.next().unwrap())
    .map(|(time, distance)| get_quadratic(time, distance))
    .reduce(|acc, r| acc * r)
    .unwrap() as u32
}

fn get_quadratic(time: u32, distance: u32) -> u32 {
  let a = -1.0;
  let b = time as f32;
  let c = (distance as f32) * -1.0;

  let sqrt = ((b.powf(2.0) - 4.0*a*c) as f32).sqrt();
  let factor_1 = (-1.0*b + sqrt)/(2.0*a);
  let factor_2 = (-1.0*b - sqrt)/(2.0*a);
  let min = factor_1.min(factor_2);
  let max = factor_1.max(factor_2);
  let lowest = if min.fract() == 0.0 { min + 1.0 } else { min }.ceil() as u32;
  let highest = if max.fract() == 0.0 { max - 1.0 } else { max }.floor() as u32;
  highest - lowest + 1
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
    const EXAMPLE_ANSWER: Option<u32> = Some(288);
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
