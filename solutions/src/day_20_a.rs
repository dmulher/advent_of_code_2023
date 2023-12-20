use std::{collections::{hash_map::DefaultHasher, HashMap, VecDeque}, hash::{Hash, Hasher}};

extern crate test;

pub fn main(contents: String) -> u32 {
  do_everything(contents)
}

fn do_everything(contents: String) -> u32 {
  let mut modules = convert_input_to_modules(contents);
  let broadcast_module_name = hash_name("broadcaster");
  let mut module_pinger: VecDeque<(u64, bool, u64)> = VecDeque::new();
  let mut highs: Vec<u32> = vec![];
  let mut lows: Vec<u32> = vec![];
  while highs.is_empty() || !modules.values().all(|module| module.module_type.is_off()) && highs.len() <1000 {
    let mut pulses = (0, 0);
    module_pinger.push_back((broadcast_module_name, false, 0));
    while let Some((module_name, pulse_type, origin)) = module_pinger.pop_front() {
      if let Some(module) = modules.get_mut(&module_name) {
        let output_signal = send_signal(&mut module.module_type, pulse_type, origin);
        if let Some(signal) = output_signal {
          for destination_name in module.destinations.iter() {
            if signal {
              pulses.0 += 1;
            } else {
              pulses.1 += 1;
            }
            module_pinger.push_back((*destination_name, signal, module_name));
          }
        }
      }
    }
    highs.push(pulses.0);
    lows.push(pulses.1 + 1);
  }
  let count = highs.len();
  let mult = 1000 / count as u32;
  let add = 1000 % count;
  let high: u32 = highs.iter().sum();
  let low: u32 = lows.iter().sum();
  let total_high: u32 = high * mult + highs[0..add].iter().sum::<u32>();
  let total_low: u32 = low * mult + lows[0..add].iter().sum::<u32>();
  total_high * total_low
}

fn convert_input_to_modules(contents: String) -> HashMap<u64, Module> {
  let mut conj_origins: HashMap<u64, Vec::<u64>> = HashMap::new();
  let mut modules = contents
    .lines()
    .map(|line| {
      let (name, destinations) = line.split_once(" -> ").unwrap();
      (name, destinations.split(", ").map(|dest_name| hash_name(dest_name)).collect::<Vec<u64>>())
    })
    .map(|(name, destinations)| {
      if name.starts_with('%') {
        let name = hash_name(name.trim_start_matches('%'));
        (name, Module{module_type: ModuleType::FlipFlop(false), destinations})
      } else if name.starts_with('&') {
        let name = hash_name(name.trim_start_matches('&'));
        conj_origins.insert(name, vec![]);
        (name, Module{module_type: ModuleType::Conjunction(HashMap::new()), destinations})
        // Conjunction
      } else {
        // Broadcaster
        let name = hash_name(name);
        (name, Module{module_type: ModuleType::Broadcaster, destinations})
      }
    })
    .collect::<HashMap<u64, Module>>();

  for (key, val) in conj_origins.iter_mut() {
    modules.iter().for_each(|(name, module)| {
      if module.destinations.contains(key) {
        val.push(*name);
      }
    });
    let module = modules.get_mut(key).unwrap();
    if let ModuleType::Conjunction(states) = &mut module.module_type {

      val.into_iter().for_each(|origin| { states.insert(*origin, false); });
    }
  }

  modules
}

fn hash_name(name: &str) -> u64 {
  let mut h = DefaultHasher::new();
  name.hash(&mut h);
  h.finish()
}

#[derive(Debug)]
struct Module {
  module_type: ModuleType,
  destinations: Vec<u64>,
}

#[derive(Debug)]
enum ModuleType {
  FlipFlop(bool),
  Conjunction(HashMap<u64, bool>),
  Broadcaster,
}

impl ModuleType {
  fn is_off(&self) -> bool {
    match self {
      Self::FlipFlop(state) => !state,
      Self::Conjunction(states) => states.values().all(|state| !state),
      Self::Broadcaster => true,
    }
  }
}

fn send_signal(module: &mut ModuleType, signal: bool, origin: u64) -> Option<bool> {
  match (module, signal) {
    (ModuleType::FlipFlop(_), true) => None,
    (ModuleType::FlipFlop(state), false) => {std::mem::swap(state, &mut !*state); Some(*state)},
    (ModuleType::Conjunction(states), _) => {
      states.insert(origin, signal);
      let sent_signal = !states.values().all(|state| *state);
      Some(sent_signal)
    },
    (ModuleType::Broadcaster, _) => Some(signal),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 20;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_20_a() {
    const EXAMPLE_ANSWER: Option<u32> = Some(11687500);
    const ANSWER: Option<u32> = Some(839775244);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_20_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
