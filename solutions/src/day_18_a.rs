extern crate test;

pub fn main(contents: String) -> u32 {
  dig_trench(contents)
}

fn dig_trench(contents: String) -> u32 {
  let (_, total) = contents
    .lines()
    .map(|line| {
      let mut split = line.split_whitespace();
      let dir = Direction::from(split.next().unwrap());
      let distance = split.next().unwrap().parse::<i32>().unwrap();
      (dir, distance)
    })
    .fold(((0, 0), 0), |((x, y), total), (dir, distance)| {
      let (change_x, change_y) = dir.to_change();
      let (next_x, next_y) = (x + change_x * distance, y + change_y * distance);
      ((next_x, next_y), total + (next_x + x) * (next_y - y) + distance)
    });

  (total as u32) / 2 + 1
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
      "R" => Direction::Right,
      "L" => Direction::Left,
      "U" => Direction::Up,
      "D" => Direction::Down,
      _ => panic!("Direction was wrong")
    }
  }
}

impl Direction {
  fn to_change(&self) -> (i32, i32) {
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
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_18_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(62);
    const ANSWER: Option<u32> = Some(48795);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_18_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
