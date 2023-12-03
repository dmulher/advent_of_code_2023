extern crate test;
// use regex::Regex;

const RED_MAX: u32 = 12;
const BLUE_MAX: u32 = 14;
const GREEN_MAX: u32 = 13;

pub fn get_possible_games(contents: String) -> u32 {
  contents
    .lines()
    .filter(|line| is_set_valid(*line))
    .map(get_id)
    .sum()
}

fn get_id(line: &str) -> u32 {
  line.split(":").next().unwrap().trim_start_matches("Game ").parse().unwrap()
}

// fn is_set_valid_with_regex(line: &str) -> bool {
//   let set_val_reg = Regex::new(r"(\d+) ([a-z]+)").unwrap();
//   let ans = set_val_reg
//     .captures_iter(line)
//     .map(|cap| match &cap[2] {
//       "red" => cap[1].parse::<u32>().unwrap() <= RED_MAX,
//       "blue" => cap[1].parse::<u32>().unwrap() <= BLUE_MAX,
//       "green" => cap[1].parse::<u32>().unwrap() <= GREEN_MAX,
//       other => panic!("colour was not right {other}")
//     })
//     .all(|a| a);
//   ans
// }

fn is_set_valid(line: &str) -> bool {
  line.split(':').last().unwrap().split(|c| c == ';' || c == ',')
    .map(|set| set.trim().split(' '))
    .map(|mut spl| {
      let val = spl.next().unwrap().parse::<u32>().unwrap();
      match spl.next() {
        Some("red") => val <= RED_MAX,
        Some("blue") => val <= BLUE_MAX,
        Some("green") => val <= GREEN_MAX,
        Some(other) => panic!("colour was not right {other}"),
        None => panic!("no whitespace?")
      }
    })
    .all(|a| a)
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const FILE_NAME: &str = "inputs/day_02_a.txt";
  const TASK_NAME: &str = "day_02_a";

  #[test]
  fn test_day_02_a() {
    const ITERATIONS: u128 = 1;
    const ANSWER: Option<u32> = Some(2268);
    utils::run_method::<u32>(&get_possible_games, FILE_NAME, ITERATIONS, ANSWER, TASK_NAME);
  }

  #[bench]
  fn bench_day_02_a(b: &mut Bencher) {
    let input = read_file_to_string(FILE_NAME);
    b.iter(|| get_possible_games(input.clone()));
  }
}