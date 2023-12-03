use std::fs;
use std::io::Write;
use std::time::Instant;
use std::time::Duration;

pub fn create_file_to_write_to(file_name: &str) -> fs::File {
  fs::File::create(file_name).unwrap()
}

pub fn write_string_to_file(file: &mut fs::File, text: String) {
  writeln!(file, "{}", text).unwrap();
}

pub fn read_file_to_string(file_name: &str) -> String {
  fs::read_to_string(file_name).expect("File was not found")
}

pub fn run_method<T: std::fmt::Debug + std::cmp::PartialEq>(method: &dyn Fn(String) -> T, file_name: &str, iterations: u128, answer: Option<T>, test_name: &str) {
  let now = Instant::now();
  let response = method(read_file_to_string(file_name));
  let elapsed = now.elapsed().as_micros();
  if answer.is_none() {
    println!("{test_name}: Final response: {:?}", response);
  }
  match answer {
    Some(ans) => if response != ans { panic!("Answer is not correct") },
    None => (),
  }

  if Duration::from_micros((elapsed * iterations) as u64) > Duration::from_secs(10) {
    panic!("{test_name}: It took {elapsed}μs to complete once, it is going to take over 10 seconds to do that {iterations} times.");
  }

  get_average_run_time::<T>(method, file_name, iterations, test_name);
}

fn get_average_run_time<T>(method: &dyn Fn(String) -> T, file_name: &str, iterations: u128, test_name: &str) {
  let mut total_time: u128 = 0;
  for _ in 1..iterations+1 {
    let now = Instant::now();
    method(read_file_to_string(file_name));
    let elapsed = now.elapsed().as_nanos();
    total_time += elapsed;
  }
  println!("{test_name}: Avg time elapsed over {iterations} iterations: {}", total_time.div_euclid(iterations));
}

pub fn get_int_from_string_slice<T: std::str::FromStr>(slice: Option<&str>, default: T) -> T {
  slice.unwrap_or("").parse::<T>().unwrap_or(default)
}

pub fn convert_lower_char_to_bin_rep(c: char) -> u32 {
  1u32 << ((c as u8) - 96)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
