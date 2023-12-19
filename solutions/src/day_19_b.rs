use std::{collections::{HashMap, hash_map::DefaultHasher}, cmp::Ordering, hash::{Hash, Hasher}};

extern crate test;

pub fn main(contents: String) -> u64 {
  get_part_values(contents)
}

fn get_part_values(contents: String) -> u64 {
  let mut rules: HashMap<u64, RuleSet> = HashMap::new();
  let first_hash = hash_name("in");
  contents
    .lines()
    .for_each(|line| {
      if !line.starts_with('{') && !line.is_empty() {
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
            let val = val_str.parse::<u64>().unwrap();
            let outcome = Outcome::from(outcome_str);
            rule_parts.push(Rule{part_type, comparison, value: val, outcome});
          }
        }
        rules.insert(rule_hash, RuleSet{rules: rule_parts, else_outcome: else_outcome.unwrap()});
      }
    });
  let initial_lim = PartLimitation{x_min: 1, x_max: 4000, m_min: 1, m_max: 4000, a_min: 1, a_max: 4000, s_min: 1, s_max: 4000};
  find_limitations(&rules, first_hash, initial_lim)
}

fn find_limitations(rules: &HashMap<u64, RuleSet>, rule_hash: u64, limitation: PartLimitation) -> u64 {
  let mut current_limitation = Some(limitation);
  let rule_set = rules.get(&rule_hash).unwrap();
  let mut total: u64 = 0;
  for rule in rule_set.rules.iter() {
    if let Some(curr_lim) = current_limitation {
      let (successes, failures) = rule.find_limitations(curr_lim);
      if let Some(success_range) = successes {
        match rule.outcome {
          Outcome::Accepted => total += success_range.total_configs(),
          Outcome::NextRule(next_rule) => total += find_limitations(rules, next_rule, success_range),
          Outcome::Rejected => (),
        }
      }
      current_limitation = failures
    }
  }
  if let Some(else_range) = current_limitation {
    match rule_set.else_outcome {
      Outcome::Accepted => total += else_range.total_configs(),
      Outcome::Rejected => (),
      Outcome::NextRule(next_rule) => total += find_limitations(rules, next_rule, else_range),
    }
  }
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
  value: u64,
  outcome: Outcome,
}

impl Rule {
  fn find_limitations(&self, part: PartLimitation) -> (Option<PartLimitation>, Option<PartLimitation>) {
    let (min, max) = part.get_val(self.part_type);
    match self.comparison {
      Ordering::Greater => {
        if max <= self.value {
          (None, Some(part))
        } else if min <= self.value {
          (Some(part.replace(self.part_type, self.value + 1, max)), Some(part.replace(self.part_type, min, self.value)))
        } else {
          (Some(part), None)
        }
      },
      Ordering::Less => {
        if min >= self.value {
          (None, Some(part))
        } else if max >= self.value {
          (Some(part.replace(self.part_type, min, self.value - 1)), Some(part.replace(self.part_type, self.value, max)))
        } else {
          (Some(part), None)
        }
      },
      Ordering::Equal => panic!("This should be impossible"),
    }
  }
}

struct RuleSet {
  rules: Vec<Rule>,
  else_outcome: Outcome,
}

#[derive(Clone, Copy)]
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

struct PartLimitation {
  x_min: u64,
  x_max: u64,
  m_min: u64,
  m_max: u64,
  a_min: u64,
  a_max: u64,
  s_min: u64,
  s_max: u64,
}

impl PartLimitation {
  fn get_val(&self, part_type: PartType) -> (u64, u64) {
    match part_type {
      PartType::X => (self.x_min, self.x_max),
      PartType::M => (self.m_min, self.m_max),
      PartType::A => (self.a_min, self.a_max),
      PartType::S => (self.s_min, self.s_max),
    }
  }

  fn replace(&self, part_type: PartType, new_min: u64, new_max: u64) -> PartLimitation {
    let mut x_min = self.x_min;
    let mut x_max = self.x_max;
    let mut m_min = self.m_min;
    let mut m_max = self.m_max;
    let mut a_min = self.a_min;
    let mut a_max = self.a_max;
    let mut s_min = self.s_min;
    let mut s_max = self.s_max;
    match part_type {
      PartType::X => {x_min = new_min; x_max = new_max},
      PartType::M => {m_min = new_min; m_max = new_max},
      PartType::A => {a_min = new_min; a_max = new_max},
      PartType::S => {s_min = new_min; s_max = new_max},
    }
    PartLimitation{x_min, x_max, m_min, m_max, a_min, a_max, s_min, s_max}
  }

  fn total_configs(&self) -> u64 {
    (self.x_max - self.x_min + 1) * (self.m_max - self.m_min + 1) * (self.a_max - self.a_min + 1) * (self.s_max - self.s_min + 1)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 19;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_19_b() {
    const EXAMPLE_ANSWER: u64 = 167409079868000;
    const ANSWER: Option<u64> = Some(116365820987729);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_19_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
