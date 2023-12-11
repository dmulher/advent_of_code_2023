use std::collections::HashSet;

extern crate test;

pub fn main(contents: String) -> u32 {
  get_shortest_total_distance(contents)
}

fn get_shortest_total_distance(contents: String) -> u32 {
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

  let mut x_adj: i32 = 0;
  let mut all: Vec<(i32, i32)> = vec![];
  let mut total_sum: u32 = 0;
  for i in 0..width {
    if empty_cols.contains(&i) { x_adj += 1; }
    else {
      let mut y_adj: i32 = 0;
      for j in 0..height {
        if empty_rows.contains(&j) { y_adj += 1; }
        else {
          if let Some(_) = galaxies.take(&(i, j)) {
            let new_galaxy = ((i as i32) + x_adj, (j as i32) + y_adj);
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

fn distance_between(a: &(i32, i32), b: &(i32, i32)) -> u32 {
  (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u32
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 11;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_11_a() {
    const EXAMPLE_ANSWER: u32 = 374;
    const ANSWER: Option<u32> = Some(9742154);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_11_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
