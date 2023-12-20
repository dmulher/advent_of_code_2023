extern crate test;

pub fn main(contents: String) -> u32 {
  rocks(contents)
}

fn rocks(contents: String) -> u32 {
  let mut load: u32 = 0;
  let transpose = transpose(contents.lines().map(|line| line.bytes().collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>());
  let map = transpose.into_iter().map(|line| line.into_iter().rev().enumerate());
  for line in map {
    let mut rocks_in_waiting = 0;
    let mut len = 0;
    for (i, r) in line {
      if r == b'#' && rocks_in_waiting > 0 {
        load += triangle_number(i) - triangle_number(i-rocks_in_waiting);
        rocks_in_waiting = 0;
      } else if r == b'O' {
        rocks_in_waiting += 1;
      }
      len += 1;
    }
    if rocks_in_waiting > 0 {
      load += triangle_number(len) - triangle_number(len-rocks_in_waiting);
    }
  }
  load
}

fn triangle_number(n: usize) -> u32 {
  let n = n as u32;
  (n * (n + 1))/2
}

fn transpose(v: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let len = v[0].len();
  let mut iters: Vec<_> = v.iter().map(|n| n.into_iter()).collect();
  (0..len)
    .map(|_| {
      iters
        .iter_mut()
        .map(|n| *n.next().unwrap())
        .collect::<Vec<u8>>()
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 14;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_14_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(136);
    const ANSWER: Option<u32> = Some(109665);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_14_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
