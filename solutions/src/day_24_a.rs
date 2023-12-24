extern crate test;

pub fn main(contents: String) -> u32 {
  check_intersections(contents)
}

fn check_intersections(contents: String) -> u32 {
  let min_coords = Coord{x: 200000000000000f32, y: 200000000000000f32, z: 200000000000000f32};
  let max_coords = Coord{x: 400000000000000f32, y: 400000000000000f32, z: 400000000000000f32};
  let vectors: Vec<Vector> = contents
    .lines()
    .map(input_to_vector)
    .collect();

  (0..vectors.len())
    .map(|v1| (v1+1..vectors.len())
      .filter(|v2| sim_eq(&vectors[v1], &vectors[*v2], &min_coords, &max_coords)).count()).sum::<usize>() as u32
}

fn input_to_vector(input: &str) -> Vector {
  // We ignore the z axis for now
  let (initial_pos_str, trajectory_str) = input.split_once(" @ ").unwrap();
  let initial_pos = input_to_coord(initial_pos_str);
  let trajectory = input_to_coord(trajectory_str);

  let x = Equation{coef: trajectory.x, constant: initial_pos.x};
  let y = Equation{coef: trajectory.y, constant: initial_pos.y};
  let z = Equation{coef: trajectory.z, constant: initial_pos.z};
  Vector{x, y, z}
}

fn sim_eq(a: &Vector, b: &Vector, min_coords: &Coord, max_coords: &Coord) -> bool {
  let s = Equation{coef: a.x.coef / b.x.coef, constant: (a.x.constant - b.x.constant) / b.x.coef}; // This is b's variable
  let t_div = a.y.coef - s.coef * b.y.coef;
  if t_div == 0.0 {
    return false;
  }
  let t = (b.y.constant - a.y.constant + b.y.coef * s.constant) / t_div;
  let intersection = Coord{x: a.x.coef * t + a.x.constant, y: a.y.coef * t + a.y.constant, z: a.z.coef * t + a.z.constant};

  let in_the_past = ((intersection.x < a.x.constant) ^ (a.x.coef < 0.0))
    || ((intersection.x < b.x.constant) ^ (b.x.coef < 0.0))
    || ((intersection.y < a.y.constant) ^ (a.y.coef < 0.0))
    || ((intersection.y < b.y.constant) ^ (b.y.coef < 0.0));

  let out_of_range = intersection.x < min_coords.x || intersection.x > max_coords.x || intersection.y < min_coords.y || intersection.y > max_coords.y;

  !in_the_past && !out_of_range
}

fn input_to_coord(input: &str) -> Coord {
  let mut split = input.split(',').map(|i| i.trim().parse::<f32>().unwrap());
  let x = split.next().unwrap();
  let y = split.next().unwrap();
  let z = split.next().unwrap();

  Coord{x, y, z}
}

#[derive(Debug)]
struct Coord {
  x: f32,
  y: f32,
  z: f32,
}

#[derive(Debug)]
struct Vector {
  x: Equation,
  y: Equation,
  z: Equation,
}

#[derive(Debug)]
struct Equation {
  coef: f32,
  constant: f32,
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 24;
  const PART: utils::Part = utils::Part::A;

  #[test]
  fn test_day_24_a() {
    const EXAMPLE_ANSWER: Option<u32> = None;
    const ANSWER: Option<u32> = Some(15107); // < 15108
    match utils::run_method::<u32>(&main, DAY, PART, (EXAMPLE_ANSWER, ANSWER)) {
      Err(message) => panic!("{}", message),
      Ok(val) if ANSWER.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_24_a(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
