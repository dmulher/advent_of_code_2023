extern crate test;

pub fn get_game_powers(contents: String) -> u32 {
  contents
    .lines()
    .map(read_line)
    .sum()
}

fn read_line(line: &str) -> u32 {
  let mut game = line.split(':').skip(1);

  get_val(game.next().unwrap())
}

// fn with_regex(line: &str) -> u32 {
//   let mut red: u32 = 0;
//   let mut blue: u32 = 0;
//   let mut green: u32 = 0;
//   let set_val_reg = Regex::new(r"(\d+) ([a-z]+)").unwrap();
//   set_val_reg
//     .captures_iter(line)
//     .for_each(|cap| match &cap[2] {
//       "red" => red = red.max(cap[1].parse::<u32>().unwrap()),
//       "blue" => blue = blue.max(cap[1].parse::<u32>().unwrap()),
//       "green" => green = green.max(cap[1].parse::<u32>().unwrap()),
//       other => panic!("colour was not right {other}")
//     });
//   red * blue * green
// }

fn get_val(line: &str) -> u32 {
  let mut red: u32 = 0;
  let mut blue: u32 = 0;
  let mut green: u32 = 0;
  line.split(':').last().unwrap().split(|c| c == ';' || c == ',')
    .map(|set| set.trim().split(' '))
    .for_each(|mut spl| {
      let val = spl.next().unwrap().parse::<u32>().unwrap();
      match spl.next() {
        Some("red") => red = red.max(val),
        Some("blue") => blue = blue.max(val),
        Some("green") => green = green.max(val),
        Some(other) => panic!("colour was not right {other}"),
        None => panic!("no whitespace?")
      }
    });
  red * blue * green
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  #[test]
  fn it_works() {
    const ITERATIONS: u128 = 20;
    const FILE_NAME: &str = "inputs/day_02_a.txt";
    const ANSWER: Option<u32> = Some(63542);
    utils::run_method::<u32>(&get_game_powers, FILE_NAME, ITERATIONS, ANSWER, "day 02 b");
  }

  #[bench]
  fn bench_day_02_b(b: &mut Bencher) {
    const FILE_NAME: &str = "inputs/day_02_a.txt";
    let input = read_file_to_string(FILE_NAME);
    b.iter(|| get_game_powers(input.clone()));
  }
}