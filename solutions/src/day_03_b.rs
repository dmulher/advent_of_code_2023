use std::collections::HashMap;

extern crate test;

pub fn sum_active_symbols(contents: String) -> u32 {
  get_symbols(&contents)
}

fn check_line(line: &HashMap<usize, char>, y: usize, x: usize) -> Vec<(usize, usize)> {
  let mut possible_nums: Vec<(usize, usize)> = vec![];
  let mut middle = false;
  if line[&y].is_digit(10) {
    possible_nums.push((x, y));
    middle = true;
  }
  if !middle && y > 0 {
    if line[&(y-1)].is_digit(10) {
      possible_nums.push((x, y-1));
    }
  }
  if !middle && y < line.len() - 1 {
    if line[&(y+1)].is_digit(10) {
      possible_nums.push((x, y+1));
    }
  }
  possible_nums
}

fn get_symbols(contents: &String) -> u32 {
  let map = contents.lines().map(|line| {
    line.char_indices().collect::<HashMap<usize, char>>()
  }).enumerate().collect::<HashMap<usize, HashMap<usize, char>>>();

  let mut total_val: u32 = 0;
  let row_count = map.len();
  for x in 0..row_count {
    let line: &HashMap<usize, char> = &map[&x];
    for y in 0..line.len() {
      let c: char = line[&y];
      if c != '*' {
        continue;
      }

      // Get all possible indexes of numbers
      let mut possible_nums: Vec<(usize, usize)> = vec![];
      if x > 0 {
        possible_nums = possible_nums.into_iter().chain(check_line(&map[&(x-1)], y, x-1).into_iter()).collect();
      }
      if y > 0 {
        if line[&(y-1)].is_digit(10) {
          possible_nums.push((x, y-1));
        }
      }
      if y < line.len() - 1 {
        if line[&(y+1)].is_digit(10) {
          possible_nums.push((x, y+1));
        }
      }
      if x < row_count - 1 {
        possible_nums = possible_nums.into_iter().chain(check_line(&map[&(x+1)], y, x+1).into_iter()).collect();
      }

      if possible_nums.len() == 2 {
        // Figure out the numbers
        total_val += possible_nums.into_iter().map(|(nx, ny)| {
          let n_line: &HashMap<usize, char> = &map[&nx];
          let mut num: String = "".to_string();

          // Get leftmost index
          let mut next = ny;
          while let Some(left) = n_line.get(&(next - 1)) {
            if left.is_digit(10) {
              next -= 1;
            } else {
              break;
            }
          }

          // Read to the right
          while let Some(right) = n_line.get(&next) {
            if right.is_digit(10) {
              num.push(*right);
              next += 1;
            } else {
              break;
            }
          }
          num.parse::<u32>().unwrap()
        }).reduce(|acc, n| {
          acc * n
        }).unwrap();
      }
    }
  }
  total_val
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const FILE_NAME: &str = "inputs/day_03_a.txt";
  const TASK_NAME: &str = "day_03_b";

  #[test]
  fn test_day_03_b() {
    const ITERATIONS: u128 = 1;
    const ANSWER: Option<u32> = Some(81463996);
    utils::run_method::<u32>(&sum_active_symbols, FILE_NAME, ITERATIONS, ANSWER, TASK_NAME);
  }

  #[bench]
  fn bench_day_03_b(b: &mut Bencher) {
    let input = read_file_to_string(FILE_NAME);
    b.iter(|| sum_active_symbols(input.clone()));
  }
}