use std::collections::{HashMap, VecDeque, HashSet};

extern crate test;

pub fn main(contents: String) -> u32 {
  find_all_captured_pieces(contents)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
  North,
  East,
  South,
  West,
}

impl Direction {
  fn reverse(self) -> Direction {
    match self {
      Direction::East => Direction::West,
      Direction::West => Direction::East,
      Direction::North => Direction::South,
      Direction::South => Direction::North,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pipe {
  Ground,
  Starting,
  Horizontal,
  Vertical,
  Corner(Direction, Direction)
}

fn find_all_captured_pieces(contents: String) -> u32 {
  let (start_idx, mut map) = build_map(&contents);
  let (width, height) = map.keys().map(|(i, j)| (i+1, j+1)).max().unwrap();

  // Figure out what the starting pipe actually is
  let real_starting_pipe = get_starting_pipe(start_idx, &map, height, width);
  let entry_move_direction = match real_starting_pipe {
    Pipe::Vertical => Direction::North,
    Pipe::Horizontal => Direction::West,
    Pipe::Corner(ns, _) => ns,
    _ => panic!("starting pipe is whack"),
  };

  // Get the original path
  map.insert(start_idx, real_starting_pipe);
  let path = follow_path(start_idx, entry_move_direction, &map, height, width);

  let mut visited: HashSet<(usize, usize)> = HashSet::new();

  // Get all non-path internal nodes
  let mut captures: VecDeque<((usize, usize), Direction)> = get_inside_boundary(&map, &mut visited, &path, height, width);

  let mut total: u32 = 0;
  // Iterate through the internal nodes and spread out
  while let Some((inside, dir)) = captures.pop_front() {
    let inside_pipe = map.get(&inside);
    if let Some(_) = inside_pipe {
      total += 1;
    }

    for (pos, dir) in get_all_neighbouring_dir(inside, dir, &map, height, width, &mut visited) {
      captures.push_back((pos, dir));
    }
  }

  total
}

fn build_map(contents: &String) -> ((usize, usize), HashMap<(usize, usize), Pipe>) {
  let mut start_idx = (0, 0);
  let map = contents
    .lines()
    .enumerate()
    .flat_map(|(j, line)| {
      line.char_indices().map(|(i, c)| {
        if c == 'S' {
          start_idx = (i, j);
        }
        ((i, j), char_to_pipe(c))
      }).collect::<Vec<((usize, usize), Pipe)>>()
    })
    .collect::<HashMap<(usize, usize), Pipe>>();
  (start_idx, map)
}

fn get_starting_pipe(start_idx: (usize, usize), map: &HashMap<(usize, usize), Pipe>, height: usize, width: usize) -> Pipe {
  let starting_dirs: Vec<Direction> = [Direction::North, Direction::South, Direction::East, Direction::West].into_iter().filter(|dir| {
    if let Some(pos) = apply_dir_to_pos(start_idx, *dir, height, width) {
      let next_pipe = map.get(&pos);
      match (dir, next_pipe) {
        (Direction::North | Direction::South, Some(&Pipe::Vertical)) => return true,
        (Direction::North, Some(&Pipe::Corner(Direction::South, _))) => return true,
        (Direction::South, Some(&Pipe::Corner(Direction::North, _))) => return true,
        (Direction::East | Direction::West, Some(&Pipe::Horizontal)) => return true,
        (Direction::East, Some(&Pipe::Corner(_, Direction::West))) => return true,
        (Direction::West, Some(&Pipe::Corner(_, Direction::East))) => return true,
        _ => return false,
      }
    }
    false
  }).collect();
  let starting_north = starting_dirs.contains(&Direction::North);
  let starting_south = starting_dirs.contains(&Direction::South);
  let starting_east = starting_dirs.contains(&Direction::East);
  let starting_west = starting_dirs.contains(&Direction::West);

  if starting_north && starting_south { Pipe::Vertical } else if starting_east && starting_west { Pipe::Horizontal } else {Pipe::Corner(starting_dirs[0], starting_dirs[1])}
}

fn get_inside_boundary(map: &HashMap<(usize, usize), Pipe>, visited: &mut HashSet<(usize, usize)>, path: &HashSet<(usize, usize)>, height: usize, width: usize) -> VecDeque<((usize, usize), Direction)> {
  let mut captures: VecDeque<((usize, usize), Direction)> = VecDeque::new();
  let entry_point = get_inside(&map, height, width, visited, &path);
  let entry_move_direction = Direction::West;
  let entry_look_direction = Direction::South;

  // Get the outer-most part
  let mut next_cell = Some((entry_point, entry_move_direction, entry_look_direction));
  // Iterate through the path again, now knowing what the inside is
  while let Some((pos, move_dir, look_dir)) = next_cell {
    // Check in look direction, move along path
    if !visited.insert(pos) {
      break;
    }
    for looked_capture in look_around(pos, look_dir, &map, height, width, visited, &path) {
      captures.push_back(looked_capture);
    }
    let curr_pipe = map.get(&pos);
    let next_dir = match curr_pipe {
      Some(Pipe::Horizontal | Pipe::Vertical) => move_dir,
      Some(Pipe::Corner(ns, es)) => follow_corner(ns, es, &move_dir),
      _ => panic!("Something went wrong"),
    };
    let next_look = match curr_pipe {
      Some(Pipe::Horizontal | Pipe::Vertical) => look_dir,
      Some(Pipe::Corner(ns, es)) => keep_look_orientation_around_corner(ns, es, &look_dir),
      _ => panic!("Something went wrong"),
    };
    if let Some(next_pos) = apply_dir_to_pos(pos, next_dir, height, width) {
      next_cell = Some((next_pos, next_dir, next_look));
    } else {
      next_cell = None;
    }
  }
  captures
}

fn follow_path(start_idx: (usize, usize), entry_move_direction: Direction, map: &HashMap<(usize, usize), Pipe>, height: usize, width: usize) -> HashSet<(usize, usize)> {
  let mut path: HashSet<(usize, usize)> = HashSet::new();
  let mut next_cell = Some((start_idx, entry_move_direction));
  while let Some((pos, move_dir)) = next_cell {
    // Check in look direction, move along path
    if path.contains(&pos) {
      break;
    } else {
      path.insert(pos);
    }
    let curr_pipe = map.get(&pos);
    let next_dir = match curr_pipe {
      Some(Pipe::Horizontal | Pipe::Vertical) => move_dir,
      Some(Pipe::Corner(ns, es)) => follow_corner(ns, es, &move_dir),
      _ => panic!("Something went wrong"),
    };
    if let Some(next_pos) = apply_dir_to_pos(pos, next_dir, height, width) {
      next_cell = Some((next_pos, next_dir));
    } else {
      next_cell = None;
    }
  }
  path
}

fn apply_dir_to_pos(pos: (usize, usize), dir: Direction, height: usize, width: usize) -> Option<(usize, usize)> {
  if (dir == Direction::North && pos.1 == 0) || (dir == Direction::South && pos.1 == height - 1) || (dir == Direction::West && pos.0 == 0) || (dir == Direction::East && pos.0 == width - 1) {
    return None;
  }
  match dir {
    Direction::North => Some((pos.0, pos.1-1)),
    Direction::East => Some((pos.0+1, pos.1)),
    Direction::South => Some((pos.0, pos.1+1)),
    Direction::West => Some((pos.0-1, pos.1)),
  }
}

fn get_inside(map: &HashMap<(usize, usize), Pipe>, height: usize, width: usize, visited: &mut HashSet<(usize, usize)>, path: &HashSet<(usize, usize)>) -> (usize, usize) {
  for j in 0..height {
    for i in 0..width {
      let map_val = map.get(&(i, j));
      if let Some(Pipe::Ground) = map.get(&(i, j)) {
        visited.insert((i, j));
        continue;
      } else if !path.contains(&(i, j)) {
        visited.insert((i, j));
        continue;
      } else if let Some(&Pipe::Corner(Direction::South, Direction::East)) = map_val {
        return (i, j);
      }
    }
  }
  panic!("Something went wrong");
}

fn look_around(pos: (usize, usize), looking: Direction, map: &HashMap<(usize, usize), Pipe>, height: usize, width: usize, visited: &mut HashSet<(usize, usize)>, path: &HashSet<(usize, usize)>) -> Vec<((usize, usize), Direction)> {
  let pipe_type = map.get(&pos);
  let mut looked_spaces = vec![];
  if let Some(Pipe::Corner(ns, ew)) = pipe_type {
    if is_looking_outside_corner(ns, ew, &looking) {

      let corner_look = keep_look_orientation_around_corner(ns, ew, &looking);
      if let Some(corner_look_pos) = apply_dir_to_pos(pos, corner_look, height, width) {
        if !visited.contains(&corner_look_pos) && !path.contains(&corner_look_pos) {
          visited.insert(corner_look_pos);
          looked_spaces.push((corner_look_pos, corner_look));
        }
      }
    } else {
      return looked_spaces;
    }
  }
  if let Some(look_pos) = apply_dir_to_pos(pos, looking, height, width) {
    if !visited.contains(&look_pos) && !path.contains(&look_pos) {
      visited.insert(look_pos);
      looked_spaces.push((look_pos, looking));
    }
  }
  looked_spaces
}

fn is_looking_outside_corner(north_south: &Direction, east_west: &Direction, look_dir: &Direction) -> bool {
  look_dir == &north_south.reverse() || look_dir == &east_west.reverse()
}

fn keep_look_orientation_around_corner(north_south: &Direction, east_west: &Direction, look_dir: &Direction) -> Direction {
  // Ugly, but verbose
  match (north_south, east_west, look_dir) {
    (Direction::North, Direction::West, Direction::West) => Direction::North,
    (Direction::North, Direction::West, Direction::East) => Direction::South,
    (Direction::North, Direction::West, Direction::North) => Direction::West,
    (Direction::North, Direction::West, Direction::South) => Direction::East,
    (Direction::South, Direction::West, Direction::West) => Direction::South,
    (Direction::South, Direction::West, Direction::East) => Direction::North,
    (Direction::South, Direction::West, Direction::North) => Direction::East,
    (Direction::South, Direction::West, Direction::South) => Direction::West,
    (Direction::North, Direction::East, Direction::West) => Direction::South,
    (Direction::North, Direction::East, Direction::East) => Direction::North,
    (Direction::North, Direction::East, Direction::North) => Direction::East,
    (Direction::North, Direction::East, Direction::South) => Direction::West,
    (Direction::South, Direction::East, Direction::West) => Direction::North,
    (Direction::South, Direction::East, Direction::East) => Direction::South,
    (Direction::South, Direction::East, Direction::North) => Direction::West,
    (Direction::South, Direction::East, Direction::South) => Direction::East,
    _ => panic!("Something has gone terribly wrong"),
  }
}

fn follow_corner(north_south: &Direction, east_west: &Direction, travel_dir: &Direction) -> Direction {
  match (north_south, east_west, travel_dir) {
    (ns, _, Direction::East | Direction::West) => *ns,
    (_, ew, Direction::North | Direction::South) => *ew,
  }
}

fn get_all_neighbouring_dir(pos: (usize, usize), dir: Direction, map: &HashMap<(usize, usize), Pipe>, height: usize, width: usize, visited: &mut HashSet<(usize, usize)>) -> Vec<((usize, usize), Direction)> {
  let mut dirs: Vec<((usize, usize), Direction)> = Vec::new();
  if dir != Direction::East {
    if let Some(neighbouring_dir) = get_next_pos(pos, Direction::West, map, height, width, visited) {
      dirs.push(neighbouring_dir);
    }
  }
  if dir != Direction::West {
    if let Some(neighbouring_dir) = get_next_pos(pos, Direction::East, map, height, width, visited) {
      dirs.push(neighbouring_dir);
    }
  }
  if dir != Direction::North {
    if let Some(neighbouring_dir) = get_next_pos(pos, Direction::South, map, height, width, visited) {
      dirs.push(neighbouring_dir);
    }
  }
  if dir != Direction::South {
    if let Some(neighbouring_dir) = get_next_pos(pos, Direction::North, map, height, width, visited) {
      dirs.push(neighbouring_dir);
    }
  }
  dirs
}

fn get_next_pos(pos: (usize, usize), moving: Direction, map: &HashMap<(usize, usize), Pipe>, height: usize, width: usize, visited: &mut HashSet<(usize, usize)>) -> Option<((usize, usize), Direction)> {
  let next_pos = apply_dir_to_pos(pos, moving, height, width)?;
  if !visited.insert(next_pos) {
    return None;
  }
  if let Some(_) = map.get(&next_pos) {
    Some((next_pos, moving))
  } else {
    None
  }
}

fn char_to_pipe(c: char) -> Pipe {
  match c {
    '|' => Pipe::Vertical,
    '-' => Pipe::Horizontal,
    'L' => Pipe::Corner(Direction::North, Direction::East),
    'J' => Pipe::Corner(Direction::North, Direction::West),
    '7' => Pipe::Corner(Direction::South, Direction::West),
    'F' => Pipe::Corner(Direction::South, Direction::East),
    '.' => Pipe::Ground,
    'S' => Pipe::Starting,
    _ => panic!("Error"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 10;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_10_b() {
    const EXAMPLE_ANSWER: u32 = 8;
    const ANSWER: Option<u32> = Some(495);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_10_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
