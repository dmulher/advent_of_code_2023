use std::fs;
use std::io::Write;
// use std::time::Instant;
// use std::time::Duration;

pub fn create_file_to_write_to(file_name: &str) -> fs::File {
  fs::File::create(file_name).unwrap()
}

pub fn write_string_to_file(file: &mut fs::File, text: String) {
  writeln!(file, "{}", text).unwrap();
}

pub fn read_file_to_string(file_name: &str) -> String {
  fs::read_to_string(file_name).expect("File was not found")
}

pub enum Part {
  A,
  B,
}

impl Part {
  pub fn lower_name(self) -> char {
    match self {
      Part::A => 'a',
      Part::B => 'b'
    }
  }
}

pub fn run_method<T: std::fmt::Debug + std::cmp::PartialEq + std::fmt::Display>(method: &dyn Fn(String) -> T, day: u8, part: Part, answers: (T, Option<T>)) -> Result<T, String> {
  let input_file_name = get_file_name(day, None);
  let example_file_name = get_file_name(day, Some(part));
  let (test_answer, part_answer) = answers;

  let test_response = method(read_file_to_string(example_file_name.as_str()));
  if test_response != test_answer {
    return Err(format!("Test response was incorrect. Expected: {test_answer}. Actual: {test_response}"));
  }

  let response = method(read_file_to_string(input_file_name.as_str()));
  match part_answer {
    Some(ans) if ans != response => Err(format!("Response was incorrect. Expected: {ans}. Actual: {response}")),
    _ => Ok(response),
  }
}

// fn get_average_run_time<T>(method: &dyn Fn(String) -> T, file_name: &str, iterations: u128, test_name: &str) {
//   let mut total_time: u128 = 0;
//   for _ in 1..iterations+1 {
//     let now = Instant::now();
//     method(read_file_to_string(file_name));
//     let elapsed = now.elapsed().as_nanos();
//     total_time += elapsed;
//   }
//   println!("{test_name}: Avg time elapsed over {iterations} iterations: {}", total_time.div_euclid(iterations));
// }

pub fn get_int_from_string_slice<T: std::str::FromStr>(slice: Option<&str>, default: T) -> T {
  slice.unwrap_or("").parse::<T>().unwrap_or(default)
}

pub fn convert_lower_char_to_bin_rep(c: char) -> u32 {
  1u32 << ((c as u8) - 96)
}

pub fn get_file_name(day: u8, part: Option<Part>) -> String {
  let day_num = format!("{:0>2}", day);
  let part_str = match part {
    Some(part_char) => format!("_{}", part_char.lower_name()),
    None => "".to_string(),
  };
  format!("inputs/day_{day_num}{part_str}.txt")
}
