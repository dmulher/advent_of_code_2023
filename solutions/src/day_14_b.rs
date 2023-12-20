extern crate test;

pub fn main(contents: String) -> u32 {
  rocks(contents)
}

fn rocks(contents: String) -> u32 {
  let mut maps: Vec<Vec<Vec<u8>>> = Vec::new();
  let mut map = contents.lines().map(|line| line.bytes().collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
  let height = map.len();
  let width = map[0].len();

  while !maps.contains(&&map) {
    maps.push(map.clone());
    rock_up_left(&mut map, width, height);
    rock_down_right(&mut map, width, height);
  }

  let loop_start = maps.iter().position(|m| m == &map).unwrap();
  let loop_end = maps.len();
  let loop_length = loop_end - loop_start;
  let remainder = (1000000000 - loop_start - 1) % loop_length;

  let final_map = &maps[loop_start + remainder + 1];

  final_map.into_iter().rev().enumerate().map(|(weight, line)| line.into_iter().filter(|r| **r == b'O').count() * (weight+1)).sum::<usize>() as u32
}

fn rock_up_left(rock_map: &mut Vec<Vec<u8>>, width: usize, height: usize) -> () {
  let mut last_known_left = vec![0; height];
  for j in 0..width {
    let mut last_known_up: usize = 0;
    for i in 0..height {
      match rock_map[i][j] {
        b'#' => {
          last_known_up = i + 1;
          last_known_left[i] = j + 1;
        },
        b'O' => {
          rock_map[i][j] = b'.';
          rock_map[last_known_up][last_known_left[last_known_up]] = b'O';
          last_known_left[last_known_up] += 1;
          last_known_up += 1;
        },
        _ => ()
      }
    }
  }
}

fn rock_down_right(rock_map: &mut Vec<Vec<u8>>, width: usize, height: usize) -> () {
  let mut last_known_right = vec![width - 1; height];
  for j in (0..width).rev() {
    let mut last_known_down: usize = height - 1;
    for i in (0..height).rev() {
      match rock_map[i][j] {
        b'#' => {
          last_known_down = i - 1;
          last_known_right[i] = j - 1;
        },
        b'O' => {
          rock_map[i][j] = b'.';
          rock_map[last_known_down][last_known_right[last_known_down]] = b'O';
          last_known_right[last_known_down] -= 1;
          last_known_down -= 1;
        },
        _ => ()
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 14;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_14_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(64);
    const ANSWER: Option<u32> = Some(96061);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_14_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
