extern crate test;

pub fn main(contents: String) -> u32 {
  get_total_score(contents)
}

#[derive(Debug, Clone, Copy)]
struct SeedRange {
  start: u32,
  end: u32,
}

impl SeedRange {
  fn new(start: u32, end: u32) -> SeedRange {
    SeedRange { start, end }
  }
}


fn get_total_score(contents: String) -> u32 {
  let mut lines = contents.lines();

  let seed_line = lines.next().unwrap();
  let mut seed_ranges: Vec<SeedRange> = vec![];
  let mut seeds = seed_line.strip_prefix("seeds: ").unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap());
  while let Some(seed) = seeds.next() {
    let range = seeds.next().unwrap();
    seed_ranges.push(SeedRange::new(seed, seed + range - 1));
  }
  let mut next_seeds: Vec<SeedRange> = vec![];
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
        let source_start = val_2.parse::<u32>().unwrap();
        let range_num = range.parse::<u32>().unwrap();
        let source_end = source_start + range_num - 1;
        for i in (0..seed_ranges.len()).rev() {
          let seed_range = seed_ranges[i];

          if seed_range.end < source_start || seed_range.start > source_end {
            // It's not in there
            continue;
          }
          if seed_range.start < source_start {
            let left_overspill = SeedRange::new(seed_range.start, source_start - 1);
            // Seperate out the front
            seed_ranges.push(left_overspill);
          }
          if seed_range.end > source_end {
            let right_overspill = SeedRange::new(source_end + 1, seed_range.end);
            // Seperate out the back
            seed_ranges.push(right_overspill);
          }

          // Put the overlap in the next batch
          let max_start = source_start.max(seed_range.start);
          let min_end = source_end.min(seed_range.end);
          let new_range_start = dest + (max_start - source_start);
          let new_range_end = new_range_start + (min_end - max_start);
          let new_range = SeedRange::new(new_range_start, new_range_end);
          next_seeds.push(new_range);

          // Remove the overlap from the current batch
          seed_ranges.remove(i);
        }
      },
      None => {
        // new map
        seed_ranges = seed_ranges.iter().chain(next_seeds.iter()).map(|seed| *seed).collect();
        next_seeds.clear();
      }
    }
  }

  seed_ranges = seed_ranges.iter().chain(next_seeds.iter()).map(|seed| *seed).collect();
  return seed_ranges.into_iter().map(|range| range.start).min().unwrap(); // 250,145,417
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 5;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_05_b() {
    const EXAMPLE_ANSWER: u32 = 46;
    const ANSWER: Option<u32> = Some(20283860);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_05_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
