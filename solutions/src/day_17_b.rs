use std::{collections::{BinaryHeap, HashSet, hash_map::DefaultHasher}, hash::{Hash, Hasher}, cmp::{Ordering, Reverse}};

extern crate test;

pub fn main(contents: String) -> u32 {
  do_thing(contents)
}

fn do_thing(contents: String) -> u32 {
  let heat: Vec<Vec<u32>> = contents.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
  find_all_paths(&heat)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
  x: usize,
  y: usize,
  dir: Direction,
  count: u8,
  distance: u32,
}

impl PartialOrd for Node {
  fn ge(&self, other: &Self) -> bool {
    return self.distance >= other.distance;
  }
  fn gt(&self, other: &Self) -> bool {
    return self.distance > other.distance;
  }
  fn le(&self, other: &Self) -> bool {
    return self.distance <= other.distance;
  }
  fn lt(&self, other: &Self) -> bool {
    return self.distance < other.distance;
  }
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.distance > other.distance {
      return Some(Ordering::Greater);
    } else if self.distance == other.distance {
      return Some(Ordering::Equal);
    } else {
      return Some(Ordering::Less);
    }
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    if self.distance > other.distance {
      return Ordering::Greater;
    } else if self.distance == other.distance {
      return Ordering::Equal;
    } else {
      return Ordering::Less;
    }
  }
}

impl Hash for Node {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.x.hash(state);
    self.y.hash(state);
    self.dir.hash(state);
    self.count.hash(state);
    self.distance.hash(state);
  }
}

impl Node {
  fn can_go_up(&self) -> bool {
    (self.y > 0 && self.dir == Direction::Up && self.count < 10) || (self.y > 3 && (self.count == 0 || ((self.dir == Direction::Left || self.dir == Direction::Right) && self.count > 3)))
  }
  fn can_go_down(&self, height: usize) -> bool {
    (self.y < height - 1 && self.dir == Direction::Down && self.count < 10) || (self.y < height - 4 && (self.count == 0 || ((self.dir == Direction::Left || self.dir == Direction::Right) && self.count > 3)))
  }
  fn can_go_left(&self) -> bool {
    (self.x > 0 && self.dir == Direction::Left && self.count < 10) || (self.x > 3 && (self.count == 0 || ((self.dir == Direction::Up || self.dir == Direction::Down) && self.count > 3)))
  }
  fn can_go_right(&self, width: usize) -> bool {
    (self.x < width - 1 && self.dir == Direction::Right && self.count < 10) || (self.x < width - 4 && (self.count == 0 || ((self.dir == Direction::Up || self.dir == Direction::Down) && self.count > 3)))
  }

  fn get_travel_length(&self, dir: &Direction) -> usize {
    if &self.dir != dir || self.count == 0 { 4 } else { 1 }
  }

  fn travel(&self, dir: &Direction, height: usize, width: usize) -> Option<(usize, usize)> {
    match dir {
      Direction::Up => if self.can_go_up() {Some((self.x, self.y - self.get_travel_length(dir)))} else {None},
      Direction::Down => if self.can_go_down(height) {Some((self.x, self.y + self.get_travel_length(dir)))} else {None},
      Direction::Left => if self.can_go_left() {Some((self.x - self.get_travel_length(dir), self.y))} else {None},
      Direction::Right => if self.can_go_right(width) {Some((self.x + self.get_travel_length(dir), self.y))} else {None},
    }
  }
}

fn hash(x: usize, y: usize, dir: &Direction, count: u8) -> u64 {
  let mut h = DefaultHasher::new();
  x.hash(&mut h);
  y.hash(&mut h);
  dir.hash(&mut h);
  count.hash(&mut h);
  h.finish()
}

fn find_all_paths(heat_map: &Vec<Vec<u32>>) -> u32 {
  let mut visited: HashSet<u64> = HashSet::new();
  let mut distances: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
  let height = heat_map.len();
  let width = heat_map[0].len();

  distances.push(Reverse(Node{x: 0, y: 0, dir: Direction::Right, count: 0, distance: 0}));
  while let Some(Reverse(node)) = distances.pop() {
    if node.x == width - 1 && node.y == height - 1 {
      return node.distance;
    }

    [Direction::Up, Direction::Left, Direction::Down, Direction::Right]
      .into_iter()
      .filter_map(|dir| {
        if let Some((x, y)) = node.travel(&dir, width, height) {
          let turned = node.dir != dir || node.count == 0;
          let count = if turned {4} else {node.count + 1};
          if visited.insert(hash(x, y, &dir, count)) {
            let distance = node.distance + stack_heat(heat_map, &dir, node.x, node.y, x, y);
            let new_node = Node{x, y, dir, count, distance};
            return Some(new_node);
          }
        }
        None
      })
      .for_each(|new_node| distances.push(Reverse(new_node)));
  }
  0
}

fn stack_heat(heat_map: &Vec<Vec<u32>>, dir: &Direction, last_x: usize, last_y: usize, x: usize, y: usize) -> u32 {
  match dir {
    Direction::Up => heat_map[y..last_y].iter().map(|l| l[x]).sum(),
    Direction::Down => heat_map[last_y+1..=y].iter().map(|l| l[x]).sum(),
    Direction::Left => heat_map[y][x..last_x].iter().sum(),
    Direction::Right => heat_map[y][last_x+1..=x].iter().sum(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 17;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_17_b() {
    const EXAMPLE_ANSWER: u32 = 94;
    const ANSWER: Option<u32> = Some(788);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_17_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
