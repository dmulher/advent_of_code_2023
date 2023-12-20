use std::collections::HashSet;

extern crate test;

pub fn main(contents: String) -> u64 {
  get_shortest_total_distance(contents)
}

fn get_shortest_total_distance(contents: String) -> u64 {
  const GALAXY: char = '#';

  let height = contents.lines().count(); // O(n)
  let mut empty_cols = (0..(height)).into_iter().collect::<HashSet<_>>(); // O(n)
  let mut empty_rows = HashSet::<usize>::new(); // O(m)
  let mut galaxies = HashSet::<(usize, usize)>::new(); // O(m*n)
  contents
    .lines()
    .map(|line| line.char_indices())
    .enumerate()
    .for_each(|(j, chars)| {
      let mut empty_row = true;
      chars.filter(|(_, c)| c == &GALAXY).for_each(|(i, _)| {
        empty_row = false;
        empty_cols.remove(&(i));
        galaxies.insert((i, j));
      });
      if empty_row {
        empty_rows.insert(j);
      }
    }); // O(n*m)
  let width = height; // Assumption, the grid is always square

  let mut x_adj: i64 = 0;
  let mut all: Vec<(i64, i64)> = vec![]; // O(n*m)
  let mut total_sum: u64 = 0;
  let adj_step: i64 = 10i64.pow(6) - 1;
  for i in 0..width {
    if empty_cols.contains(&i) { x_adj += 1; }
    else {
      let mut y_adj: i64 = 0;
      for j in 0..height {
        if empty_rows.contains(&j) { y_adj += 1; }
        else {
          if let Some(_) = galaxies.take(&(i, j)) {
            let new_galaxy = ((i as i64) + x_adj * adj_step, (j as i64) + y_adj * adj_step);
            for g in all.iter() {
              total_sum += distance_between(&new_galaxy, g); // O((n*m)^2)
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
    const EXAMPLE_ANSWER: Option<u64> = Some(82000210);
    const ANSWER: Option<u64> = Some(411142919886);
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
