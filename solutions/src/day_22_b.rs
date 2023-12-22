use std::{hash::{Hash, Hasher}, collections::{hash_map::DefaultHasher, HashMap, HashSet}};

extern crate test;

pub fn main(contents: String) -> u64 {
  count_disintegrations(contents)
}

fn count_disintegrations(contents: String) -> u64 {
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

  let mut tops: Vec<Vec<u64>> = vec![vec![]; height];
  let mut bottoms: Vec<Vec<u64>> = vec![vec![]; height];
  for (hash, block) in blocks.iter() {
    bottoms[block.z_1 - 1].push(*hash);
    tops[block.z_2 - 1].push(*hash);
  }
  bottoms = bottoms.into_iter().map(|mut row| {row.sort(); row}).collect();
  tops = tops.into_iter().map(|mut row| {row.sort(); row}).collect();

  let mut supported_by: HashMap<u64, Vec<u64>> = HashMap::new();
  let mut irremoveables: HashSet<u64> = HashSet::new();
  for y in (1..(height)).rev() {
    let slice_tops = &tops[y-1];
    let slice_bots = &bottoms[y];
    for bot_hash in slice_bots {
      let mut is_supported_by: Vec<u64> = vec![];
      let bot = blocks.get(&bot_hash).unwrap();
      for top_hash in slice_tops {
        let top = blocks.get(&top_hash).unwrap();
        if bot.z_aligned(top) {
          is_supported_by.push(*top_hash);
        }
      }

      if is_supported_by.len() == 1 {
        irremoveables.insert(*is_supported_by.first().unwrap());
      }
      supported_by.insert(*bot_hash, is_supported_by);
    }
  }

  let mut memory: HashMap<u64, u64> = HashMap::new();
  irremoveables
    .into_iter()
    .map(|hash| remove_single(hash, &bottoms, &blocks, &supported_by, height, &mut memory))
    .sum()
}

fn remove_single(hash: u64, block_bottoms: &Vec<Vec<u64>>, blocks: &HashMap<u64, Block>, supported_by: &HashMap<u64, Vec<u64>>, max_height: usize, memory: &mut HashMap<u64, u64>) -> u64 {
  let block = blocks.get(&hash).unwrap();
  let top = block.z_2;
  let mut falling_blocks = vec![vec![]; max_height - (top + 1)];
  falling_blocks.push(vec![hash]);
  remove(falling_blocks, block_bottoms, blocks, supported_by, top, memory)
}

fn remove(mut falling_blocks: Vec<Vec<u64>>, block_bottoms: &Vec<Vec<u64>>, blocks: &HashMap<u64, Block>, supported_by: &HashMap<u64, Vec<u64>>, height: usize, memory: &mut HashMap<u64, u64>) -> u64 {
  let mut debug = "\n".to_string();
  debug.push_str(format!("Tower = {:?}\n", falling_blocks).as_str());
  debug.push_str(format!("Height = {height}\n").as_str());
  let falling_block_hash = hash_tower(&falling_blocks);
  if let Some(ans) = memory.get(&falling_block_hash) {
    debug.push_str(format!("We had previously found it. ans = {ans}\n").as_str());
    *ans
  } else if let Some(falling_row) = falling_blocks.pop() {
    debug.push_str(format!("Our row is {:?}\n", falling_row).as_str());
    let mut total_falling_blocks = 0;
    if falling_blocks.len() > 0 && falling_row.len() > 0 {
      let row_above = &block_bottoms[height];
      debug.push_str(format!("Row above is {:?}\n", row_above).as_str());
      row_above
        .iter()
        .filter(|above_hash| supported_by
          .get(&above_hash)
          .unwrap()
          .iter()
          .all(|supporting_hash| falling_row.contains(supporting_hash))
        )
        .for_each(|new_falling_hash| {
          let new_block = blocks.get(new_falling_hash).unwrap();
          let height = new_block.z_2 - new_block.z_1;
          let idx = falling_blocks.len() - 1 - height;
          debug.push_str(format!("Found new falling block {new_falling_hash}\n").as_str());
          falling_blocks[idx].push(*new_falling_hash);
          total_falling_blocks += 1;
        });
    }
    total_falling_blocks += remove(falling_blocks, block_bottoms, blocks, supported_by, height + 1, memory);
    memory.insert(falling_block_hash, total_falling_blocks);
    debug.push_str(format!("Answer is {total_falling_blocks}\n").as_str());
    total_falling_blocks
  } else {
    0
  }
}

fn hash_tower(falling_blocks: &Vec<Vec<u64>>) -> u64 {
  let mut h = DefaultHasher::new();
  falling_blocks.hash(&mut h);
  h.finish()
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
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_22_b() {
    const EXAMPLE_ANSWER: Option<u64> = Some(7);
    const ANSWER: Option<u64> = None;
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_22_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
