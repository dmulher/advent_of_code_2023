extern crate test;

pub fn main(contents: String) -> u32 {
  looping(contents)
}

fn looping(contents: String) -> u32 {
  contents.lines().map(get_val_for_line_l).sum()
}

fn get_val_for_line_l(line: &str) -> u32 {
  let mut line_split = line.split_whitespace();
  let springs = [line_split.next().unwrap(); 5].join("?");
  let nums = line_split.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>().repeat(5);

  get_val_l(&springs, &nums) as u32
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
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_12_a() {
    const EXAMPLE_ANSWER: u32 = 21;
    const ANSWER: Option<u32> = Some(8270);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_12_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
