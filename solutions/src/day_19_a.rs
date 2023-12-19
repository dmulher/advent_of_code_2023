use std::{collections::{HashMap, hash_map::DefaultHasher}, cmp::Ordering, hash::{Hash, Hasher}};

extern crate test;

pub fn main(contents: String) -> u32 {
  get_part_values(contents)
}

fn get_part_values(contents: String) -> u32 {
  let mut rules: HashMap<u64, RuleSet> = HashMap::new();
  let mut total: u32 = 0;
  let first_hash = hash_name("in");
  contents
    .lines()
    .for_each(|line| {
      if line.starts_with('{') {
        // Part
        let mut components = line
          .trim_start_matches('{')
          .trim_end_matches('}')
          .split(',')
          .map(|comp| comp.split('=').skip(1).next().unwrap().parse::<u32>().unwrap());
        let x = components.next().unwrap();
        let m = components.next().unwrap();
        let a = components.next().unwrap();
        let s = components.next().unwrap();
        let part = Part{x, m, a, s};
        // Test the rules

        let mut outcome = Outcome::NextRule(first_hash);
        while let Outcome::NextRule(next_rule) = outcome {
          outcome = rules.get(&next_rule).unwrap().test(&part);
        }
        if let Outcome::Accepted = outcome {
          total += part.x + part.m + part.a + part.s;
        }
      } else if !line.is_empty() {
        // Rule
        let mut rule_str_split = line.split('{');

        let rule_name = rule_str_split.next().unwrap();
        let mut h = DefaultHasher::new();
        rule_name.hash(&mut h);
        let rule_hash = h.finish();

        let mut rule_split = rule_str_split.next().unwrap().trim_end_matches('}').split(',').peekable();
        let mut rule_parts: Vec<Rule> = Vec::new();
        let mut else_outcome = None;
        while let Some(rule_str) = rule_split.next() {
          if let None = rule_split.peek() {
            else_outcome = Some(Outcome::from(rule_str));
          } else {
            let (part_type_str, rule_str) = rule_str.split_at(1);
            let (delimeter_str, rule_str) = rule_str.split_at(1);
            let (val_str, outcome_str) = rule_str.split_once(':').unwrap();
            let part_type = PartType::from(part_type_str);
            let comparison = if delimeter_str == "<" {Ordering::Less} else {Ordering::Greater};
            let val = val_str.parse::<u32>().unwrap();
            let outcome = Outcome::from(outcome_str);
            rule_parts.push(Rule{part_type, comparison, value: val, outcome});
          }
        }
        rules.insert(rule_hash, RuleSet{rules: rule_parts, else_outcome: else_outcome.unwrap()});
      }
    });
  total
}

fn hash_name(name: &str) -> u64 {
  let mut h = DefaultHasher::new();
  name.hash(&mut h);
  h.finish()
}

#[derive(Clone, Copy)]
enum Outcome {
  NextRule(u64),
  Accepted,
  Rejected,
}

impl From<&str> for Outcome {
  fn from(value: &str) -> Self {
    match value {
      "A" => Outcome::Accepted,
      "R" => Outcome::Rejected,
      _ => Outcome::NextRule(hash_name(value)),
    }
  }
}

struct Rule {
  part_type: PartType,
  comparison: Ordering,
  value: u32,
  outcome: Outcome,
}

impl Rule {
  fn test(&self, part: &Part) -> bool {
    let test_rating = match self.part_type {
      PartType::X => part.x,
      PartType::M => part.m,
      PartType::A => part.a,
      PartType::S => part.s,
    };
    test_rating.cmp(&self.value) == self.comparison
  }
}

struct RuleSet {
  rules: Vec<Rule>,
  else_outcome: Outcome,
}

impl RuleSet {
  fn test(&self, part: &Part) -> Outcome {
    for rule in self.rules.iter() {
      if rule.test(part) {
        return rule.outcome;
      }
    }
    return self.else_outcome;
  }
}

enum PartType {
  X,
  M,
  A,
  S,
}

impl From<&str> for PartType {
  fn from(value: &str) -> Self {
    match value {
      "x" => Self::X,
      "m" => Self::M,
      "a" => Self::A,
      "s" => Self::S,
      _ => panic!("Part type was wrong"),
    }
  }
}

struct Part {
  x: u32,
  m: u32,
  a: u32,
  s: u32,
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 19;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_19_a() {
    const EXAMPLE_ANSWER: u32 = 19114;
    const ANSWER: Option<u32> = Some(362930);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_19_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
