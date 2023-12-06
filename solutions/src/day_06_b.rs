extern crate test;

pub fn main(contents: String) -> u64 {
  get_total_score(contents)
}

fn get_total_score(contents: String) -> u64 {
  let mut lines = contents.lines().map(|line| line.split_whitespace().skip(1).collect::<String>().parse::<u64>().unwrap());
  let time: u64 = lines.next().unwrap();
  let distance: u64 = lines.next().unwrap();

  get_quadratic(time, distance)
}

// fn get_possible_times_naive(time: u64, distance: u64) -> u64 {
//   (0..time).map(|time_held_down| (time - time_held_down) * time_held_down).filter(|attempt| attempt > &distance).count() as u64
// }

// fn get_possible_times_slightly_better(time: u64, distance: u64) -> u64 {
//   let lowest = get_lowest_possible_time(time, distance);
//   let highest = get_highest_possible_time(time, distance);
//   highest - lowest + 1
// }

// fn get_lowest_possible_time(time: u64, distance: u64) -> u64 {
//   let mut min_time_held_down: u64 = 0;
//   let mut travelled: u64 = 0;
//   while distance > travelled {
//     min_time_held_down += 1;
//     travelled = (time - min_time_held_down) * min_time_held_down;
//   }
//   min_time_held_down
// }

// fn get_highest_possible_time(time: u64, distance: u64) -> u64 {
//   let mut travelled: u64 = 0;
//   let mut max_time_held_down: u64 = time;
//   while distance > travelled {
//     max_time_held_down -= 1;
//     travelled = (time - max_time_held_down) * max_time_held_down;
//   }
//   max_time_held_down
// }

fn get_quadratic(time: u64, distance: u64) -> u64 {
  let a = -1.0;
  let b = time as f64;
  let c = (distance as f64) * -1.0;

  let sqrt = ((b.powf(2.0) - 4.0*a*c) as f64).sqrt();
  let factor_1 = (-1.0*b + sqrt)/(2.0*a);
  let factor_2 = (-1.0*b - sqrt)/(2.0*a);
  let min = factor_1.min(factor_2);
  let max = factor_1.max(factor_2);
  let lowest = if min.fract() == 0.0 { min + 1.0 } else { min }.ceil() as u64;
  let highest = if max.fract() == 0.0 { max - 1.0 } else { max }.floor() as u64;
  highest - lowest + 1
}

// fn funny_single_line(contents: String) -> u64 {
//   contents.lines().map(|line| line.split_whitespace().skip(1).collect::<String>().parse::<f64>().unwrap()).array_chunks::<2>().map(|[time, distance]| ((-1.0*time + (time.powf(2.0) - 4.0*distance).sqrt())/-2.0, (-1.0*time - (time.powf(2.0) - 4.0*distance).sqrt())/-2.0)).map(|(fac_1, fac_2)| (fac_1.max(fac_2).floor() as u64) - (fac_1.min(fac_2).ceil() as u64) + 1).next().unwrap()
// }

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 6;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_06_b() {
    const EXAMPLE_ANSWER: u64 = 71503;
    const ANSWER: Option<u64> = Some(21039729);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_06_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
