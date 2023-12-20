extern crate test;

pub fn main(contents: String) -> u64 {
  dig_trench(contents)
}

fn dig_trench(contents: String) -> u64 {
  let (_, total) = contents
    .lines()
    .map(|line| {
      let hex = line.split('#').skip(1).next().unwrap().replace(')', "");
      let (distance_hex, dir_num) = hex.split_at(5);
      let dir = Direction::from(dir_num);
      let distance = i64::from_str_radix(distance_hex, 16).unwrap();
      (dir, distance)
    })
    .fold(((0, 0), 0), |((x, y), total), (dir, distance)| {
      let (change_x, change_y) = dir.to_change();
      let (next_x, next_y) = (x + change_x * distance, y + change_y * distance);
      ((next_x, next_y), total + (next_x + x) * (next_y - y) + distance)
    });

  (total as u64) / 2 + 1
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl From<&str> for Direction {
  fn from(value: &str) -> Self {
    match value {
      "0" => Direction::Right,
      "1" => Direction::Down,
      "2" => Direction::Left,
      "3" => Direction::Up,
      _ => panic!("Direction was wrong")
    }
  }
}

impl Direction {
  fn to_change(&self) -> (i64, i64) {
    match self {
      Direction::Right => (1, 0),
      Direction::Left => (-1, 0),
      Direction::Up => (0, -1),
      Direction::Down => (0, 1),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 18;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_18_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(952408144115);
    const ANSWER: Option<u64> = Some(40654918441248);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_18_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
