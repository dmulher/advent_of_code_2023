use std::collections::{VecDeque, HashSet};

extern crate test;

pub fn main(contents: String) -> u64 {
  find_reachable_plots(contents)
}

fn find_reachable_plots(contents: String) -> u64 {
  let map = get_map(contents);
  let start = get_start_pos(&map);

  let height = map.len();
  let width = map[0].len();

  let max_steps: usize = 26501365;
  let full_maps_in_line = max_steps / height;

  let steps_used = full_maps_in_line * height;

  let centre_total = find_reachable_plots_for_map(&map, height, width, start.clone(), max_steps);
  let off_centre_total = find_reachable_plots_for_map(&map, height, width, start, max_steps + 1);

  if max_steps <= height/2 { return centre_total as u64; }

  let corners = [(0,0), (0, height - 1), (width - 1, 0), (width - 1, height - 1)];
  let centres = [(width / 2, 0), (width / 2, height - 1), (0, height / 2), (width - 1, height / 2)];

  let even_length = full_maps_in_line % 2 == 0;
  let odds_evens = match even_length {
    true => ((full_maps_in_line - 1).pow(2), full_maps_in_line.pow(2)),
    false => (full_maps_in_line.pow(2), (full_maps_in_line - 1).pow(2)),
  };

  let mut total = 0;

  let has_orthagonal = full_maps_in_line > 0;
  if has_orthagonal {
    let orthagonal_steps_remaining = max_steps - (steps_used - height / 2);
    let orthagonal_steps: usize = centres.into_iter().map(|pos| find_reachable_plots_for_map(&map, height, width, pos, orthagonal_steps_remaining)).sum::<usize>();
    total += orthagonal_steps;
  }

  let new_corner_steps_remaining = max_steps - (steps_used + 1);
  let old_corner_steps_remaining = max_steps - (steps_used + 1 - height);
  let num_new_corners = if max_steps - steps_used == 0 { 0 } else { full_maps_in_line };
  let num_old_corners = full_maps_in_line - 1;

  let has_centre_aligned_corners = max_steps > height && (max_steps != steps_used || (max_steps / height) % 2 == 0);
  if has_centre_aligned_corners {
    let is_centre_aligned_cycle_new = (max_steps / height) % 2 == 1;
    let steps_remaining = if is_centre_aligned_cycle_new { new_corner_steps_remaining } else { old_corner_steps_remaining };
    let corner_steps: usize = corners.into_iter().map(|pos| find_reachable_plots_for_map(&map, height, width, pos, steps_remaining)).sum::<usize>();
    let num_corners = if is_centre_aligned_cycle_new { num_new_corners } else { num_old_corners };
    total += num_corners * corner_steps;
  }

  let has_off_aligned_corners = max_steps > height * 2 && (max_steps != steps_used || (max_steps / height) % 2 == 1);
  if has_off_aligned_corners {
    let is_off_aligned_cycle_new = (max_steps / height) % 2 == 0;
    let steps_remaining = if is_off_aligned_cycle_new { new_corner_steps_remaining } else { old_corner_steps_remaining };
    let corner_steps: usize = corners.into_iter().map(|pos| find_reachable_plots_for_map(&map, height, width, pos, steps_remaining)).sum::<usize>();
    let num_corners = if is_off_aligned_cycle_new { num_new_corners } else { num_old_corners };
    total += num_corners * corner_steps;
  }

  let has_peeking = max_steps - steps_used > (height / 2);
  if has_peeking {
    let peeking_steps_remaining = max_steps - (steps_used + height / 2 + 1);
    let peeking_steps = centres.into_iter().map(|pos| find_reachable_plots_for_map(&map, height, width, pos, peeking_steps_remaining)).sum::<usize>();
    total += peeking_steps;
  }

  let ans = (total + odds_evens.0 * centre_total + odds_evens.1 * off_centre_total) as u64;
  ans
}

fn get_map(contents: String) -> Vec<Vec<u8>> {
  contents
    .lines()
    .map(|line| line.bytes().collect::<Vec<u8>>())
    .collect::<Vec<Vec<u8>>>()
}

fn get_start_pos(map: &Vec<Vec<u8>>) -> (usize, usize) {
  map.iter().enumerate().find_map(|(y, line)| {
    match line.iter().position(|c| *c == b'S') {
      Some(x) => Some((x, y)),
      None => None
    }
  }).unwrap()
}

fn find_reachable_plots_for_map(map: &Vec<Vec<u8>>, height: usize, width: usize, pos: (usize, usize), steps_remaining: usize) -> usize {
  let directions = [Direction::North, Direction::South, Direction::West, Direction::East];

  let mut visited: HashSet<(usize, usize)> = HashSet::new();
  let mut pathing: VecDeque<((usize, usize), usize)> = VecDeque::new();
  pathing.push_back((pos, steps_remaining));

  let mut count = 0;
  while let Some((pos, steps)) = pathing.pop_front() {
    if !visited.insert(pos) {
      continue;
    }
    if steps % 2 == 0 {
      count += 1;
    }
    if steps > 0 {
      for dir in &directions {
        if let Some(new_pos) = dir.move_in_map(pos, height, width) {
          if map[new_pos.1][new_pos.0] != b'#' {
            pathing.push_back((new_pos, steps - 1));
          }
        }
      }
    }
  }
  count
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
  North,
  South,
  East,
  West,
}

impl Direction {
  fn move_in_map(&self, pos: (usize, usize), height: usize, width: usize) -> Option<(usize, usize)> {
    match self {
      Direction::North => if pos.1 == 0 {None} else {Some((pos.0, pos.1-1))},
      Direction::South => if pos.1 + 1 == height {None} else {Some((pos.0, pos.1+1))},
      Direction::West => if pos.0 == 0 {None} else {Some((pos.0-1, pos.1))},
      Direction::East => if pos.0 + 1 == width {None} else {Some((pos.0+1, pos.1))},
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 21;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_21_b() {
    const EXAMPLE_ANSWER: Option<u64> = None;
    const ANSWER: Option<u64> = Some(597102953699891);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_21_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
