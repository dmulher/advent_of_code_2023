use std::collections::HashMap;

extern crate test;

pub fn main(contents: String) -> u32 {
  sum_active_symbols(contents)
}

fn sum_active_symbols(contents: String) -> u32 {
  let symbols = get_symbols(&contents);
  contents
    .lines()
    .enumerate()
    .map(|(idx, line)| {
      let previous_line = if idx > 0 { Some(&symbols[&(idx-1)]) } else { None };
      get_parts(line, previous_line, symbols.get(&idx).unwrap(), symbols.get(&(idx+1)))
    })
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
      if !blessed && idx > 0 {
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

  const DAY: u8 = 3;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_03_a() {
    const EXAMPLE_ANSWER: u32 = 4361;
    const ANSWER: Option<u32> = Some(527144);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_03_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
