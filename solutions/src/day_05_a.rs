extern crate test;

pub fn main(contents: String) -> u32 {
  get_total_score(contents)
}

fn get_total_score(contents: String) -> u32 {
  let mut lines = contents.lines();

  let seed_line = lines.next().unwrap();
  let mut seeds = seed_line.strip_prefix("seeds: ").unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

  let mut next_seeds: Vec<u32> = vec![];
  while let Some(line) = lines.next() {
    if line == "" {
      continue;
    }
    let mut vals = line.split_whitespace();
    let val_1 = vals.next().unwrap();
    let val_2 = vals.next().unwrap();
    let val_3 = vals.next();
    match val_3 {
      Some(range) => {
        let dest = val_1.parse::<u32>().unwrap();
        let source = val_2.parse::<u32>().unwrap();
        let range_num = range.parse::<u32>().unwrap();
        for i in (0..seeds.len()).rev() {
          let seed = seeds[i];
          if seed >= source && seed < (source + range_num) {
            next_seeds.push(dest + (seeds.remove(i) - source));
          }
        }
      },
      None => {
        // new map
        seeds = seeds.iter().chain(next_seeds.iter()).map(|seed| *seed).collect();
        next_seeds.clear();
      }
    }
  }

  return *(seeds.iter().min().unwrap());
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 5;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_05_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(35);
    const ANSWER: Option<u32> = Some(313045984);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_05_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
