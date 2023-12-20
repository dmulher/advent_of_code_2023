use std::collections::HashMap;

extern crate test;

pub fn main(contents: String) -> u32 {
  get_total_winnings(contents)
}

fn get_total_winnings(contents: String) -> u32 {
  let mut bets = contents
    .lines()
    .map(|line| {
      let mut split = line.split_whitespace();
      let hand = read_hand_enum(split.next().unwrap());
      let bet = split.next().unwrap().parse::<u32>().unwrap();
      (hand, bet)
    })
    .collect::<Vec<(u32, u32)>>();
  bets.sort_unstable_by(|(hand, _), (hand_2, _)| hand_2.cmp(hand));
  bets
    .into_iter()
    .enumerate()
    .map(|(idx, (_, val))| ((idx as u32)+1) * val )
    .sum()
}

fn read_hand_enum(hand: &str) -> u32 {
  let mut hand_map: HashMap<u8, u8> = HashMap::new();
  let mut hand_val: u32 = 0;
  let mut card_count: u8 = 0;
  let mut jokers: u8 = 0;
  let mut max: u8 = 0;

  for (idx, card) in hand
    .chars()
    .map(char_to_u8)
    .enumerate() {
      hand_val += (card as u32) << 4*(4-idx);
      if card == 12 {
        jokers += 1;
        continue;
      }
      match hand_map.get(&card) {
        Some(v) => {
          max = max.max(v+1);
          hand_map.insert(card, v+1);
        },
        None => {
          max = max.max(1);
          card_count += 1;
          hand_map.insert(card, 1);
        },
      };
    }

  (hand_to_u32(max + jokers, card_count) << 20) + hand_val
}

fn char_to_u8(c: char) -> u8 {
  match c {
    'A' => 00,
    'K' => 01,
    'Q' => 02,
    'T' => 03,
    '9' => 04,
    '8' => 05,
    '7' => 06,
    '6' => 07,
    '5' => 08,
    '4' => 09,
    '3' => 10,
    '2' => 11,
    'J' => 12,
    _ => panic!("Oh no"),
  }
}

fn hand_to_u32(max: u8, count: u8) -> u32 {
  (match (max, count) {
    (5, _) => 0,
    (4, _) => 1,
    (3, 2) => 2,
    (3, _) => 3,
    (2, 3) => 4,
    (2, _) => 5,
    (1, _) => 6,
    _ => panic!("No idea")
  }) as u32
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 7;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_07_b() {
    const EXAMPLE_ANSWER: Option<u32> = Some(5905);
    const ANSWER: Option<u32> = Some(251135960);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_07_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
