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

  let end_pos = travel_path(start_node, Direction::South, &map, width, height, start_node, Direction::South, 0, true, &mut disgusting_intersection_map);
  println!("intersections = {:?}", disgusting_intersection_map);
  println!("end_pos = {:?}", end_pos);

  let mut visited = HashSet::new();
  visited.insert(start_node);
  longest_path_between(start_node, end_pos.unwrap(), &disgusting_intersection_map, visited)
}

fn longest_path_between(start_pos: (usize, usize), end_pos: (usize, usize), intersection_map: &HashMap<(usize, usize), HashMap<Direction, ((usize, usize), u32)>>, visited: HashSet<(usize, usize)>) -> u32 {
  let curr_intersection = intersection_map.get(&start_pos).unwrap();
  curr_intersection
    .iter()
    .map(|(_, path)| {
      if path.0 == start_pos {
        0
      } else if path.0 == end_pos {
        path.1
      } else {
        let mut new_visited = visited.clone();
        if new_visited.insert(path.0) {
          path.1 + longest_path_between(path.0, end_pos, intersection_map, new_visited)
        } else {
          0
        }
      }
    })
    .max()
    .unwrap()
}

fn get_all_paths(pos: (usize, usize), travel_dir: Direction, height: usize, width: usize, map: &Vec<Vec<PathSegment>>) -> Vec<Direction> {
  let exists_north = travel_dir != Direction::South && pos.1 > 0 && map[pos.1-1][pos.0] != PathSegment::Wall;
  let exists_south = travel_dir != Direction::North && pos.1 < height - 1 && map[pos.1+1][pos.0] != PathSegment::Wall;
  let exists_west = travel_dir != Direction::East && pos.0 > 0 && map[pos.1][pos.0-1] != PathSegment::Wall;
  let exists_east = travel_dir != Direction::West && pos.0 < width - 1 && map[pos.1][pos.0+1] != PathSegment::Wall;
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

fn travel_path(curr_pos: (usize, usize), travel_dir: Direction, map: &Vec<Vec<PathSegment>>, width: usize, height: usize, last_intersection: (usize, usize), last_intersection_direction: Direction, curr_distance: u32, reversable: bool, intersection_map: &mut HashMap<(usize, usize), HashMap<Direction, ((usize, usize), u32)>>) -> Option<(usize, usize)> {
  if curr_pos.1 == height - 1 {
    intersection_map.get_mut(&last_intersection).unwrap().insert(last_intersection_direction, (curr_pos, curr_distance));
    return Some(curr_pos);
  }
  let curr_node = map[curr_pos.1][curr_pos.0];
  let still_reversable = reversable && curr_node == PathSegment::Free;
  let possible_paths = get_all_paths(curr_pos, travel_dir, height, width, map);

  match possible_paths[..] {
    [] => None,
    [next_dir] => {
      if curr_node.can_travel_direction(&next_dir) {
        let next_pos = next_dir.travel(curr_pos);
        travel_path(next_pos, next_dir, map, width, height, last_intersection, last_intersection_direction, curr_distance + 1, still_reversable, intersection_map)
      } else {
        None
      }
    },
    _ => {
      // Create a new intersection
      let intersection_dir_map = intersection_map.entry(curr_pos).or_insert(HashMap::new());
      if !intersection_dir_map.contains_key(&travel_dir.opposite()) && reversable {
        intersection_dir_map.insert(travel_dir.opposite(), (last_intersection, curr_distance));
      }
      intersection_map.get_mut(&last_intersection).unwrap().insert(last_intersection_direction, (curr_pos, curr_distance));
      possible_paths.into_iter().filter_map(|next_dir: Direction| {
        if curr_node.can_travel_direction(&next_dir) {
          let next_pos = next_dir.travel(curr_pos);
          travel_path(next_pos, next_dir, map, width, height, curr_pos, next_dir, 1, curr_node == PathSegment::Free, intersection_map)
        } else {
          None
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
  Jump(Direction),
  Wall
}

impl From<u8> for PathSegment {
  fn from(value: u8) -> Self {
    match value {
      b'.' => PathSegment::Free,
      b'^' => PathSegment::Jump(Direction::North),
      b'v' => PathSegment::Jump(Direction::South),
      b'<' => PathSegment::Jump(Direction::West),
      b'>' => PathSegment::Jump(Direction::East),
      b'#' => PathSegment::Wall,
      _ => panic!("Map piece incorrect")
    }
  }
}

impl PathSegment {
  fn can_travel_direction(&self, direction: &Direction) -> bool {
    match (self, direction) {
      (PathSegment::Free, _) => true,
      (PathSegment::Jump(Direction::North), Direction::North) => true,
      (PathSegment::Jump(Direction::South), Direction::South) => true,
      (PathSegment::Jump(Direction::East), Direction::East) => true,
      (PathSegment::Jump(Direction::West), Direction::West) => true,
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 23;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_23_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(94);
    const ANSWER: Option<u32> = Some(2134);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_23_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
