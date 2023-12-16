extern crate test;

pub fn main(contents: String) -> u32 {
  hashmap(contents)
}

enum Operation {
  Add(usize),
  Rm,
}

fn hashmap(contents: String) -> u32 {
  let mut map: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
  contents
    .split(',')
    .map(read_command)
    .for_each(|(box_num, label, operation)| {
      let box_list = &mut map[box_num as usize];
      match operation {
        Operation::Add(lens_length) => {
          if let Some(idx) = box_list.iter().position(|x| x.0 == label) {
            box_list[idx] = (label, lens_length);
          } else {
            box_list.push((label, lens_length));
          }
        },
        Operation::Rm => {
          if let Some(idx) = box_list.iter().position(|x| x.0 == label) {
            box_list.remove(idx);
          }
        }
      }
    });

  map
    .into_iter()
    .enumerate()
    .map(|(box_num, lens_box)| lens_box
      .into_iter()
      .enumerate()
      .map(|(pos, (_, lens_focus))| (box_num + 1) * (pos + 1) * lens_focus as usize)
      .sum::<usize>())
    .sum::<usize>() as u32
  //.fold( 0))
}

fn read_command(command: &str) -> (usize, &str, Operation) {
  let mut split = command.split(&['-', '=']);
  let label = split.next().unwrap();
  let operation = if command.ends_with('-') { Operation::Rm } else { Operation::Add(split.next().unwrap().parse::<usize>().unwrap()) };
  let box_num = label.bytes().fold(0, |acc, c| (acc + c)*17) as usize;
  (box_num, label, operation)
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 15;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_15_b() {
    const EXAMPLE_ANSWER: u32 = 145;
    const ANSWER: Option<u32> = Some(259333);
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_15_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
