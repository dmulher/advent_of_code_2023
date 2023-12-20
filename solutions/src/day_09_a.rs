extern crate test;

pub fn main(contents: String) -> i32 {
  get_next_history(contents)
}

fn get_next_history(contents: String) -> i32 {
  contents
    .lines()
    .map(|line| line.split_whitespace().map(|val| val.parse::<i32>().unwrap()).collect::<Vec<i32>>())
    .map(|line| {
      let mut vals = line;
      let mut last_vals: Vec<i32> = vec![];
      while vals.len() > 0 {
        let mut last_val = vals.remove(0);
        let mut zeroes = true;
        for i in 0..vals.len() {
          let new_val = vals[i] - last_val;
          last_val = vals[i];
          vals[i] = new_val;
          zeroes = zeroes && new_val == 0;
        }
        last_vals.push(last_val);
        if zeroes {
          return last_vals.into_iter().sum();
        }
      }
      0
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 9;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_09_a() {
    const EXAMPLE_ANSWER: Option<i32> = Some(114);
    const ANSWER: Option<i32> = Some(1853145119);
    match utils::run_method::<i32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_09_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
