use std::collections::HashSet;

extern crate test;

pub fn main(contents: String) -> u64 {
  get_shortest_total_distance(contents)
}

fn get_shortest_total_distance(contents: String) -> u64 {
  const GALAXY: char = '#';
  let grid: Vec<Vec<char>> = contents
    .lines()
    .map(|line| line.chars().collect())
    .collect();

  let width = grid.len();
  let height = grid[0].len();
  let mut empty_cols = (0..(height)).into_iter().collect::<HashSet<_>>();
  let mut empty_rows = HashSet::<usize>::new();
  let mut galaxies = HashSet::<(usize, usize)>::new();
  for (j, line) in grid.iter().enumerate() {
    let mut empty_row = true;
    for (i, c) in line.iter().enumerate() {
      if c == &GALAXY {
        empty_row = false;
        empty_cols.remove(&(i));
        galaxies.insert((i, j));
      }
    }
    if empty_row {
      empty_rows.insert(j);
    }
  }

  let mut x_adj: i64 = 0;
  let mut all: Vec<(i64, i64)> = vec![];
  let mut total_sum: u64 = 0;
  for i in 0..width {
    if empty_cols.contains(&i) { x_adj += 1; }
    else {
      let mut y_adj: i64 = 0;
      for j in 0..height {
        if empty_rows.contains(&j) { y_adj += 1; }
        else {
          if let Some(_) = galaxies.take(&(i, j)) {
            let new_galaxy = ((i as i64) + x_adj * 999999, (j as i64) + y_adj * 999999);
            for g in all.iter() {
              total_sum += distance_between(&new_galaxy, g);
            }
            all.push(new_galaxy);
          }
        }
      }
    }
  }

  total_sum
}

fn distance_between(a: &(i64, i64), b: &(i64, i64)) -> u64 {
  (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u64
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 11;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_11_b() {
    const EXAMPLE_ANSWER: u64 = 82000210;
    const ANSWER: Option<u64> = None;
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_11_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
