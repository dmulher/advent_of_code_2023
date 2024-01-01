use std::collections::{HashMap, HashSet};

extern crate test;

pub fn main(contents: String) -> u32 {
  cut_wires(contents, 0)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Node(u8, u8, u8);

impl FromIterator<u8> for Node {
  fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
    let mut into_iter = iter.into_iter();
    let a = into_iter.next().unwrap();
    let b = into_iter.next().unwrap();
    let c = into_iter.next().unwrap();
    Self(a, b, c)
  }
}

fn cut_wires(contents: String, attempts: u8) -> u32 {
  if attempts > 100 {
    panic!("Not working");
  }
  let mut wires: HashMap<Node, HashSet<Node>> = HashMap::new();

  contents
    .lines()
    .for_each(|line| {
      let (left_node, connecting_nodes) = line.split_once(": ").unwrap();
      let left = left_node.bytes().collect::<Node>();
      connecting_nodes
        .split_whitespace()
        .map(|node| node.bytes().collect::<Node>())
        .for_each(|right| {
          wires.entry(left).or_insert(HashSet::new()).insert(right);
          wires.entry(right).or_insert(HashSet::new()).insert(left);
        })
    });

  let total_set = wires.keys().map(|node| *node).collect::<HashSet<Node>>();
  let mut left_set = total_set.clone();

  let mut missing_links = left_set.iter().map(|node| count_set(wires.get(node).unwrap(), &left_set)).sum::<usize>();
  let mut broken = false;
  while missing_links != 3 {
    if left_set.len() <= 1 {
      broken = true;
      break;
    }
    let max = left_set.iter().max_by(|a, b| count_set(wires.get(*a).unwrap(), &left_set).cmp(&count_set(wires.get(*b).unwrap(), &left_set))).unwrap().clone();
    left_set.remove(&max);
    missing_links = left_set.iter().map(|node| count_set(wires.get(node).unwrap(), &left_set)).sum::<usize>();
  }
  if broken {
    cut_wires(contents, attempts + 1)
  } else {
    (left_set.len() * (total_set.len() - left_set.len())) as u32
  }
}

fn count_set(connections: &HashSet<Node>, left_set: &HashSet<Node>) -> usize {
  connections.difference(left_set).into_iter().count()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 25;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_25_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(54);
    const ANSWER: Option<u32> = Some(533628);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_25_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
