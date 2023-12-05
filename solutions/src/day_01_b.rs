extern crate test;
use std::collections::HashMap;

pub fn main(contents: String) -> u32 {
  get_calibration(contents)
}

fn get_calibration(contents: String) -> u32 {
  let char_to_word: HashMap<char, Vec<(&str, u32)>> = HashMap::from([
    ('o', vec![("ne", 1)]),
    ('t', vec![("wo", 2), ("hree", 3)]),
    ('f', vec![("our", 4), ("ive", 5)]),
    ('s', vec![("ix", 6), ("even", 7)]),
    ('e', vec![("ight", 8)]),
    ('n', vec![("ine", 9)]),
  ]);

  let char_to_word_backwards: HashMap<char, Vec<(&str, u32)>> = HashMap::from([
    ('e', vec![("no", 1), ("erht", 3), ("vif", 5), ("nin", 9)]),
    ('o', vec![("wt", 2)]),
    ('r', vec![("uof", 4)]),
    ('x', vec![("is", 6)]),
    ('n', vec![("eves", 7)]),
    ('t', vec![("hgie", 8)]),
  ]);

  contents
    .lines()
    .map(|line| get_first_and_last(line, &char_to_word, &char_to_word_backwards))
    .reduce(|acc, n| acc + n).unwrap()
}

fn get_first_and_last(line: &str, char_to_word: &HashMap<char, Vec<(&str, u32)>>, char_to_word_backwards: &HashMap<char, Vec<(&str, u32)>>) -> u32 {
  let mut chars = line.chars();
  let mut chars_rev = chars.clone().rev();

  let first = search_line(&mut chars, vec![], char_to_word);
  let last = search_line(&mut chars_rev, vec![], char_to_word_backwards);
  match (first, last) {
    (Some(f_n), Some(l_n)) => f_n * 10 + l_n,
    _ => 0,
  }
}

fn search_line(chars: &mut impl Iterator<Item = char>, remaining_words: Vec<(&str, u32)>, char_to_word: &HashMap<char, Vec<(&str, u32)>>) -> Option<u32> {
  if let Some(c) = chars.next() {
    if let Some(d) = c.to_digit(10) {
      return Some(d);
    }
    let mut possible_words: Vec<(&str, u32)> = vec![];
    for (p_s, p_v) in remaining_words.into_iter() {
      if p_s.starts_with(c) {
        if p_s.len() == 1 {
          return Some(p_v);
        }
        possible_words.push((&p_s[1 ..], p_v));
      }
    }
    if let Some(new_patterns) = char_to_word.get(&c) {
      possible_words.append(&mut new_patterns.clone());
    }
    return search_line(chars, possible_words, char_to_word);
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 1;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_01_b() {
    const EXAMPLE_ANSWER: u32 = 281;
    const ANSWER: Option<u32> = Some(54770);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_01_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
