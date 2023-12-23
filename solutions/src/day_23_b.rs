use std::collections::{HashMap, HashSet};

extern crate test;

pub fn main(contents: String) -> u32 {
  brute_force(contents)
}

fn brute_force(contents: String) -> u32 {
  let map = contents
    .lines()
    .map(|line| line.bytes().map(|b| PathSegment::from(b)).collect::<Vec<PathSegment>>())
    .collect::<Vec<Vec<PathSegment>>>();

  let height = map.len();
  let width = map[0].len();

  // First, make each intersection into a node
  let start_node = (1, 0);
  let mut disgusting_intersection_map: HashMap<(usize, usize), HashMap<Direction, ((usize, usize), u32)>> = HashMap::new();
  disgusting_intersection_map.insert(start_node, HashMap::new());

  let end_pos = travel_path(start_node, Direction::South, &map, width, height, start_node, Direction::South, 0, &mut disgusting_intersection_map);

  let mut visited = HashSet::new();
  visited.insert(start_node);
  longest_path_between(start_node, end_pos.unwrap(), &disgusting_intersection_map, visited, 0)
}

fn longest_path_between(start_pos: (usize, usize), end_pos: (usize, usize), intersection_map: &HashMap<(usize, usize), HashMap<Direction, ((usize, usize), u32)>>, visited: HashSet<(usize, usize)>, curr_depth: u32) -> u32 {
  let curr_intersection = intersection_map.get(&start_pos).unwrap();
  curr_intersection
    .iter()
    .map(|(_, path)| {
      if path.0 == start_pos {
        0
      } else if path.0 == end_pos {
        curr_depth + path.1
      } else {
        let mut new_visited = visited.clone();
        if new_visited.insert(path.0) {
          longest_path_between(path.0, end_pos, intersection_map, new_visited, curr_depth + path.1)
        } else {
          0
        }
      }
    })
    .max()
    .unwrap()
}

fn get_all_paths(pos: (usize, usize), travel_dir: Direction, map: &Vec<Vec<PathSegment>>) -> Vec<Direction> {
  let exists_north = travel_dir != Direction::South && map[pos.1-1][pos.0] != PathSegment::Wall;
  let exists_south = travel_dir != Direction::North && map[pos.1+1][pos.0] != PathSegment::Wall;
  let exists_west = travel_dir != Direction::East && map[pos.1][pos.0-1] != PathSegment::Wall;
  let exists_east = travel_dir != Direction::West && map[pos.1][pos.0+1] != PathSegment::Wall;
  let mut possible_paths = vec![];
  if exists_north {
    possible_paths.push(Direction::North);
  }
  if exists_south {
    possible_paths.push(Direction::South);
  }
  if exists_west {
    possible_paths.push(Direction::West);
  }
  if exists_east {
    possible_paths.push(Direction::East);
  }
  possible_paths
}

fn travel_path(curr_pos: (usize, usize), travel_dir: Direction, map: &Vec<Vec<PathSegment>>, width: usize, height: usize, last_intersection: (usize, usize), last_intersection_direction: Direction, curr_distance: u32, intersection_map: &mut HashMap<(usize, usize), HashMap<Direction, ((usize, usize), u32)>>) -> Option<(usize, usize)> {
  if curr_pos.1 == height - 1 {
    intersection_map.get_mut(&last_intersection).unwrap().insert(last_intersection_direction, (curr_pos, curr_distance));
    return Some(curr_pos);
  }
  let possible_paths = get_all_paths(curr_pos, travel_dir, map);

  match possible_paths[..] {
    [] => None,
    [next_dir] => {
      let next_pos = next_dir.travel(curr_pos);
      travel_path(next_pos, next_dir, map, width, height, last_intersection, last_intersection_direction, curr_distance + 1, intersection_map)
    },
    _ => {
      // Create a new intersection
      let intersection_dir_map = intersection_map.entry(curr_pos).or_insert(HashMap::new());
      if !intersection_dir_map.contains_key(&travel_dir.opposite()) {
        intersection_dir_map.insert(travel_dir.opposite(), (last_intersection, curr_distance));
      }
      intersection_map.get_mut(&last_intersection).unwrap().insert(last_intersection_direction, (curr_pos, curr_distance));
      possible_paths.into_iter().filter_map(|next_dir: Direction| {
        let next_pos = next_dir.travel(curr_pos);
        if intersection_map.get(&curr_pos).unwrap().contains_key(&next_dir) {
          None
        } else {
          travel_path(next_pos, next_dir, map, width, height, curr_pos, next_dir, 1, intersection_map)
        }
      }).fold(None, |acc, pos| match (acc, pos) {
        (_, p) => Some(p),
      })
    },
  }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Direction {
  North,
  South,
  West,
  East,
}

impl Direction {
  fn opposite(&self) -> Self {
    match self {
      &Self::North => Self::South,
      &Self::South => Self::North,
      &Self::West => Self::East,
      &Self::East => Self::West
    }
  }

  fn travel(&self, pos: (usize, usize)) -> (usize, usize) {
    match self {
      &Self::North => (pos.0, pos.1-1),
      &Self::South => (pos.0, pos.1+1),
      &Self::West => (pos.0-1, pos.1),
      &Self::East => (pos.0+1, pos.1),
    }
  }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum PathSegment {
  Free,
  Wall
}

impl From<u8> for PathSegment {
  fn from(value: u8) -> Self {
    match value {
      b'.' | b'^' | b'v' | b'<' | b'>' => PathSegment::Free,
      b'#' => PathSegment::Wall,
      _ => panic!("Map piece incorrect")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 23;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_23_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(154);
    const ANSWER: Option<u32> = Some(6298);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_23_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
