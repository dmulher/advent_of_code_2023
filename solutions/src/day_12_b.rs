use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

extern crate test;

pub fn main(contents: String) -> u64 {
  recursion(contents)
}

fn recursion(contents: String) -> u64 {
  contents.lines().map(get_val_for_line_r).sum()
}

fn get_val_for_line_r(line: &str) -> u64 {
  let mut line_split = line.split_whitespace();
  let springs = [line_split.next().unwrap(); 5].join("?");
  let nums = line_split.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>().repeat(5);

  let springs = springs.as_bytes();
  let mut memory: HashMap<u64, u64> = HashMap::new();
  let n = springs.len(); // Line length
  let m = nums.len(); // Num clues
  let result = get_val_r(springs, &nums, 0, 0, n, m, &mut memory);

  result as u64
}

fn get_val_r(remaining_line: &[u8], clues: &[usize], clue_idx: usize, current_broken: usize, line_len: usize, clue_len: usize, memory: &mut HashMap<u64, u64>) -> u64 {
  let mem = hashem(remaining_line, clues, clue_idx, current_broken, clue_len);
  if let Some(mem_val) = memory.get(&mem) {
    return *mem_val;
  }

  if remaining_line.is_empty() {
    if (clue_idx == clue_len && current_broken == 0) || (clue_idx == clue_len - 1 && clues[clue_idx] == current_broken) {
      return 1;
    }
    return 0;
  }
  if current_broken > 0 && clue_idx >= clue_len {
    return 0;
  }

  let next_c = remaining_line[0];
  let operational = next_c == b'.' || next_c == b'?';
  let broken = next_c == b'#' || next_c == b'?';
  let mut val = 0;
  
  if operational {
    if current_broken == 0 {
      val += get_val_r(&remaining_line[1..], clues, clue_idx, 0, line_len, clue_len, memory);
    } else if clue_idx < clue_len && clues[clue_idx] == current_broken {
      val += get_val_r(&remaining_line[1..], clues, clue_idx+1, 0, line_len, clue_len, memory);
    }
  }
  if broken {
    val += get_val_r(&remaining_line[1..], clues, clue_idx, current_broken + 1, line_len, clue_len, memory);
  }
  memory.insert(mem, val);
  val
}

fn hashem(remaining_line: &[u8], clues: &[usize], clue_idx: usize, current_broken: usize, clue_len: usize) -> u64 {
  let mut h = DefaultHasher::new();
  remaining_line.hash(&mut h);
  clues.hash(&mut h);
  clue_idx.hash(&mut h);
  current_broken.hash(&mut h);
  clue_len.hash(&mut h);
  h.finish()
}

fn looping(contents: String) -> u64 {
  contents.lines().map(get_val_for_line_l).sum()
}

fn get_val_for_line_l(line: &str) -> u64 {
  let mut line_split = line.split_whitespace();
  let springs = [line_split.next().unwrap(); 5].join("?");
  let nums = line_split.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>().repeat(5);

  get_val_l(&springs, &nums) as u64
}

fn get_val_l(line: &str, clues: &[usize]) -> usize {
  let line = line.as_bytes();
  let n = line.len();
  let m = clues.len();
  let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];

  dp[n][m][0] = 1;
  dp[n][m - 1][clues[m - 1]] = 1;

  for pos in (0..n).rev() {
    let operational = line[pos] == b'.' || line[pos] == b'?';
    let broken = line[pos] == b'#' || line[pos] == b'?';
    for (clue_idx, &clue_val) in clues.iter().enumerate() {
      for current_broken in 0..=clue_val {
        if operational {
          if current_broken == 0 {
            dp[pos][clue_idx][0] += dp[pos + 1][clue_idx][0];
          } else if clue_idx < m && clues[clue_idx] == current_broken {
            dp[pos][clue_idx][current_broken] += dp[pos + 1][clue_idx + 1][0];
          }
        }
        if broken {
          dp[pos][clue_idx][current_broken] += dp[pos + 1][clue_idx][current_broken + 1];
        }
      }
    }
    if operational {
      dp[pos][m][0] += dp[pos + 1][m][0];
    }
  }

  dp[0][0][0]
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 12;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_12_b_recursion() {
    const EXAMPLE_ANSWER: u64 = 525152;
    const ANSWER: Option<u64> = Some(204640299929836);
    match utils::run_method::<u64>(&recursion, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[test]
  fn test_day_12_b_looping() {
    const EXAMPLE_ANSWER: u64 = 525152;
    const ANSWER: Option<u64> = Some(204640299929836);
    match utils::run_method::<u64>(&looping, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_12_b_recursion(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| recursion(input.clone()));
  }

  #[bench]
  fn bench_day_12_b_looping(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| looping(input.clone()));
  }
}
