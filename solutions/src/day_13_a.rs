extern crate test;

pub fn main(contents: String) -> usize {
  get_reflections(contents)
}

fn get_reflections(contents: String) -> usize {
  let mut patterns: Vec<Vec<Vec<char>>> = vec![];
  let mut idx = 0;
  let mut size = 0;
  contents.lines().for_each(|l| {
    if l.is_empty() {
      idx += 1;
      size = 0;
    } else {
      if size == 0 {
        patterns.push(vec![]);
      }
      patterns[idx].push(l.chars().collect::<Vec<char>>());
      size += 1;
    }
  });

  patterns.into_iter().map(|p| {
    match find_horizontal_symmetry(&p) {
      0 => find_vertical_symmetry(&p),
      val => val
    }
  }).sum()
}

fn find_vertical_symmetry(pattern: &Vec<Vec<char>>) -> usize {
  let row_count = pattern.len();
  let col_count = pattern[0].len();
  let mut match_start: Option<usize> = None;
  let mut matching = 0;
  let mut x = 1;
  while x < col_count {
    match match_start {
      Some(idx) => {
        if idx < matching {
          return matching;
        }
        let mut failed = false;
        for y in 0..row_count {
          if pattern[y][x] != pattern[y][idx-matching] {
            failed = true;
            break;
          }
        }
        if failed {
          matching = 0;
          match_start = None;
          x = idx + 2;
        } else {
          matching += 1;
          x += 1;
        }
      },
      None => {
        let idx = x - 1;
        let mut failed = false;
        for y in 0..row_count {
          if pattern[y][x] != pattern[y][idx] {
            failed = true;
            break;
          }
        }
        if !failed {
          matching = 1;
          match_start = Some(idx);
        }
        x += 1;
      }
    }
  }

  if let Some(start_idx) = match_start {
    return start_idx + 1;
  }
  0
}

fn find_horizontal_symmetry(pattern: &Vec<Vec<char>>) -> usize {
  let mut match_from: Option<usize> = None;
  let mut matching: usize = 0;

  let row_count = pattern.len();
  let mut y = 1;
  while y < row_count {
    let line = &pattern[y];
    match match_from {
      Some(idx) => {
        if matching > idx {
          return 100 * matching;
        }
        if &pattern[idx - matching] == line {
          matching += 1;
          y += 1;
        } else {
          matching = 0;
          match_from = None;
          y = idx + 2;
        }
      },
      None => {
        let match_idx = y - 1;
        if &pattern[match_idx] == line {
          matching = 1;
          match_from = Some(match_idx);
        }
        y += 1;
      }
    }
  }

  if let Some(start_idx) = match_from {
    return 100 * (start_idx + 1);
  }
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 13;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_13_a() {
    const EXAMPLE_ANSWER: Option<usize> = Some(405);
    const ANSWER: Option<usize> = Some(27502);
    match utils::run_method::<usize>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_13_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
