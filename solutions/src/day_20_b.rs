use std::{collections::{hash_map::DefaultHasher, HashMap, VecDeque}, hash::{Hash, Hasher}};
use utils::maths::Lcm;

extern crate test;

pub fn main(contents: String) -> u64 {
  find_output_signal(contents)
}

fn find_output_signal(contents: String) -> u64 {
  let broadcast_module_name = hash_name("broadcaster");
  let mut modules = convert_input_to_modules(contents);
  let inputs = get_inputs(&modules);

  let final_module_name = hash_name("zh");
  let feeders = inputs.get(&final_module_name).unwrap();
  let mut feeder_feeders = feeders.iter().flat_map(|mod_name| inputs.get(mod_name).unwrap()).map(|name| *name).collect::<Vec<u64>>();
  // Let's assume they follow the same looping rules as day 8
  let mut cycles: Vec<u64> = vec![];

  let mut count = 0;
  while !feeder_feeders.is_empty() {
    count += 1;
    let mut module_pinger: VecDeque<(u64, bool, u64)> = VecDeque::new();
    module_pinger.push_back((broadcast_module_name, false, 0));
    while let Some((module_name, pulse_type, origin)) = module_pinger.pop_front() {
      if let Some(module) = modules.get_mut(&module_name) {
        let output_signal = send_signal(&mut module.module_type, pulse_type, origin);
        if let Some(signal) = output_signal {
          if !signal && feeder_feeders.contains(&module_name) {
            feeder_feeders.remove(feeder_feeders.iter().position(|name| name == &module_name).unwrap());
            cycles.push(count);
          }
          for destination_name in module.destinations.iter() {
            module_pinger.push_back((*destination_name, signal, module_name));
          }
        }
      }
    }
    if count % 10000000 == 0 {
      println!("{count} => {}", cycles.len());
    }
  }
  cycles.into_iter().reduce(|acc, val| u64::lcm(acc, val)).unwrap()
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

fn get_inputs(modules: &HashMap<u64, Module>) -> HashMap<u64, Vec<u64>> {
  let mut inputs: HashMap<u64, Vec<u64>> = HashMap::new();
  for (key, module) in modules.iter() {
    for dest in module.destinations.iter() {
      if !inputs.contains_key(dest) {
        inputs.insert(*dest, vec![]);
      }
      inputs.get_mut(dest).unwrap().push(*key);
    }
  }
  inputs
}

// fn get_requirements(modules: &HashMap<u64, Module>, inputs: &HashMap<u64, Vec<u64>>, final_module: u64, requirement: bool, origin_name: u64) -> HashMap<u64, bool> {
//   let mut requirements: HashMap<u64, bool> = HashMap::new();
//   let mut requirement_queue: VecDeque<(u64, bool)> = VecDeque::new();
//   requirement_queue.push_back((final_module, requirement));
//   while let Some((name, requirement)) = requirement_queue.pop_front() {
//     if name == origin_name {
//       continue;
//     }
//     requirements.insert(name, requirement);
//     let input_names = inputs.get(&name).unwrap();
//     let input_requirement = match modules.get(&name).unwrap().module_type {
//       ModuleType::FlipFlop(_) => false,
//       ModuleType::Conjunction(_) => !requirement, // Assuming that all true is enough. Huge assumption though.
//       _ => panic!("Should be impossible"),
//     };
//   }
//   requirements
// }

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
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_20_b() {
    const EXAMPLE_ANSWER: Option<u64> = None;
    const ANSWER: Option<u64> = Some(207787533680413);
    match utils::run_method::<u64>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_20_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
