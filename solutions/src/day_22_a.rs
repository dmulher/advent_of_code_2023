use std::{hash::{Hash, Hasher}, collections::{hash_map::DefaultHasher, HashMap, HashSet}};

extern crate test;

pub fn main(contents: String) -> u32 {
  count_disintegrations(contents)
}

fn count_disintegrations(contents: String) -> u32 {
  let mut initial_positions = contents
    .lines()
    .map(|line| {
      let (start_str, end_str) = line.split_once('~').unwrap();
      let start = get_coords(start_str);
      let end = get_coords(end_str);
      Block::from((start, end))
    })
    .collect::<Vec<Block>>();
  initial_positions.sort_by_key(|block| block.z_1);

  let width = initial_positions.iter().map(|block| block.x_2).max().unwrap() + 1;
  let depth = initial_positions.iter().map(|block| block.y_2).max().unwrap() + 1;

  let mut blocks: HashMap<u64, Block> = HashMap::new();
  let mut lowest_pos: Vec<Vec<usize>> = vec![vec![1; width]; depth];
  initial_positions.into_iter().for_each(|block| {
    let fallen_block = block.fall(&mut lowest_pos);
    let hash = hash(&fallen_block);
    blocks.insert(hash, fallen_block);
  });
  let height = blocks.values().map(|block| block.z_2).max().unwrap() + 1;
  let len = blocks.len();

  let mut tops: Vec<Vec<u64>> = vec![vec![]; height];
  let mut bottoms: Vec<Vec<u64>> = vec![vec![]; height];
  for (hash, block) in blocks.iter() {
    bottoms[block.z_1 - 1].push(*hash);
    tops[block.z_2 - 1].push(*hash);
  }

  let mut irremoveables: HashSet<u64> = HashSet::new();
  for y in 0..(height-1) {
    let slice_tops = &tops[y];
    let slice_bots = &bottoms[y+1];
    for bot_hash in slice_bots {
      let mut supported_by: u16 = 0;
      let mut first_support: Option<u64> = None;
      let bot = blocks.get(&bot_hash).unwrap();
      for top_hash in slice_tops {
        let top = blocks.get(&top_hash).unwrap();
        if bot.z_aligned(top) {
          supported_by += 1;
          match supported_by {
            1 => first_support = Some(*top_hash),
            _ => break,
          }
        }
      }
      if supported_by == 1 {
        irremoveables.insert(first_support.unwrap());
      }
    }
  }

  (len - irremoveables.len()) as u32
}

fn get_coords(coord: &str) -> Coord {
  let mut split = coord.split(',');
  let x = split.next().unwrap().parse::<usize>().unwrap();
  let y = split.next().unwrap().parse::<usize>().unwrap();
  let z = split.next().unwrap().parse::<usize>().unwrap();
  Coord{x, y, z}
}

#[derive(Debug)]
struct Coord {
  x: usize,
  y: usize,
  z: usize,
}

fn hash(block: &Block) -> u64 {
  let mut h = DefaultHasher::new();
  block.hash(&mut h);
  h.finish()
}

#[derive(Debug)]
struct Block {
  x_1: usize,
  x_2: usize,
  y_1: usize,
  y_2: usize,
  z_1: usize,
  z_2: usize,
}

impl From<(Coord, Coord)> for Block {
  fn from((start, end): (Coord, Coord)) -> Self {
    let x_1 = start.x.min(end.x);
    let x_2 = start.x.max(end.x);
    let y_1 = start.y.min(end.y);
    let y_2 = start.y.max(end.y);
    let z_1 = start.z.min(end.z);
    let z_2 = start.z.max(end.z);
    Block{x_1, x_2, y_1, y_2, z_1, z_2}
  }
}

impl Hash for Block {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.x_1.hash(state);
    self.x_2.hash(state);
    self.y_1.hash(state);
    self.y_2.hash(state);
    self.z_1.hash(state);
    self.z_2.hash(state);
  }
}

impl Block {
  fn fall(&self, lowest_pos: &mut Vec<Vec<usize>>) -> Self {
    let lowest_z = lowest_pos[self.y_1..=self.y_2].iter().map(|line| line[self.x_1..=self.x_2].iter().max().unwrap()).max().unwrap();

    let difference = self.z_2 - self.z_1;
    let z_1 = *lowest_z;
    let z_2 = z_1 + difference;

    for y in self.y_1..=self.y_2 {
      for x in self.x_1..=self.x_2 {
        lowest_pos[y][x] = z_2 + 1;
      }
    }

    Block{x_1: self.x_1, x_2: self.x_2, y_1: self.y_1, y_2: self.y_2, z_1, z_2 }
  }

  fn z_aligned(&self, other: &Block) -> bool {
    !(self.x_2 < other.x_1 || self.x_1 > other.x_2 || self.y_2 < other.y_1 || self.y_1 > other.y_2)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 22;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_22_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(5);
    const ANSWER: Option<u32> = Some(393);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_22_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
