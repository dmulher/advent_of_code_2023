use std::collections::HashMap;

extern crate test;

pub fn sum_active_symbols(contents: String) -> u32 {
  let symbols = get_symbols(&contents);
  contents
    .lines()
    .enumerate()
    .map(|(idx, line)| get_parts(line, symbols.get(&(idx-1)), symbols.get(&idx).unwrap(), symbols.get(&(idx+1))))
    .sum()
}

fn get_parts(line: &str, previous_line: Option<&Vec<usize>>, current_line: &Vec<usize>, next_line: Option<&Vec<usize>>) -> u32 {
  let mut total_num = "".to_string();
  let mut total = 0;
  let mut blessed = false;
  let max_idx = line.len();
  for (idx, val) in line.chars().enumerate() {
    if val.is_digit(10) {
      total_num.push(val);
      if !blessed {
        blessed = current_line.contains(&(idx-1)) || previous_line.unwrap_or(&Vec::<usize>::new()).contains(&(idx-1)) || next_line.unwrap_or(&Vec::<usize>::new()).contains(&(idx-1));
      }
    } else {
      if !total_num.is_empty() {
        if !blessed {
          blessed = current_line.contains(&(idx)) || previous_line.unwrap_or(&Vec::<usize>::new()).contains(&(idx-1)) || next_line.unwrap_or(&Vec::<usize>::new()).contains(&(idx-1)) || previous_line.unwrap_or(&Vec::<usize>::new()).contains(&idx) || next_line.unwrap_or(&Vec::<usize>::new()).contains(&idx);
        }
        if blessed {
          total += total_num.parse::<u32>().unwrap();
        }
      }
      blessed = false;
      total_num.clear();
    }
  }
  if !total_num.is_empty() {
    if !blessed {
      blessed = previous_line.unwrap_or(&Vec::<usize>::new()).contains(&(max_idx-2)) || previous_line.unwrap_or(&Vec::<usize>::new()).contains(&(max_idx-1)) || next_line.unwrap_or(&Vec::<usize>::new()).contains(&(max_idx-2)) || next_line.unwrap_or(&Vec::<usize>::new()).contains(&(max_idx-1));
    }
    if blessed {
      total += total_num.parse::<u32>().unwrap();
    }
  }
  total
}

fn get_symbols(contents: &String) -> HashMap<usize, Vec<usize>> {
  contents
    .lines()
    .enumerate()
    .map(|(line_num, line)| {
      (line_num, line
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_digit(10) && c != &'.')
        .map(|(col_num, _)| col_num)
        .collect::<Vec<usize>>())
    })
    .collect::<HashMap<usize, Vec<usize>>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const FILE_NAME: &str = "inputs/day_03_a.txt";
  const TASK_NAME: &str = "day_03_a";

  #[test]
  fn test_day_03_a() {
    const ITERATIONS: u128 = 1;
    const ANSWER: Option<u32> = Some(527144);
    utils::run_method::<u32>(&sum_active_symbols, FILE_NAME, ITERATIONS, ANSWER, TASK_NAME);
  }

  #[bench]
  fn bench_day_03_a(b: &mut Bencher) {
    let input = read_file_to_string(FILE_NAME);
    b.iter(|| sum_active_symbols(input.clone()));
  }
}