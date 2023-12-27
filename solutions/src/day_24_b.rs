use std::ops::{Add, Mul, Sub, Div};

extern crate test;
extern crate num_bigint;

use num_bigint::{BigInt, Sign};

pub fn main(contents: String) -> BigInt {
  check_intersections(contents)
}

fn check_intersections(contents: String) -> BigInt {
  let vectors: Vec<Hailstone> = contents
    .lines()
    .map(input_to_vector)
    .collect();

  solve(vectors)
}

fn input_to_vector(input: &str) -> Hailstone {
  // We ignore the z axis for now
  let (initial_pos_str, trajectory_str) = input.split_once(" @ ").unwrap();
  let initial_pos = input_to_3d_vec(initial_pos_str);
  let trajectory = input_to_3d_vec(trajectory_str);
  Hailstone{p: initial_pos, v: trajectory}
}

fn input_to_3d_vec(input: &str) -> Vec3 {
  let mut split = input.split(',').map(|i| i.trim().parse::<BigInt>().unwrap());
  let x = split.next().unwrap();
  let y = split.next().unwrap();
  let z = split.next().unwrap();

  Vec3{x, y, z}
}

fn solve(hailstones: Vec<Hailstone>) -> BigInt {
  let h1 = hailstones[0].clone();

  let mut h2_op = None;
  let mut h3_op = None;
  for h in hailstones.iter() {
    if h2_op.is_none() {
      if h1.v.is_independant(&h.v) {
        h2_op = Some(h);
      }
    } else {
      if h1.v.is_independant(&h.v) && h2_op.unwrap().v.is_independant(&h.v) {
        h3_op = Some(h);
      }
    }
  }

  let h2 = h2_op.unwrap();
  let h3 = h3_op.unwrap();

  let h0 = find_hailstone_position(h1, h2.clone(), h3.clone());
  h0.x + h0.y + h0.z
}

#[derive(Clone, Debug)]
struct Hailstone {
  p: Vec3,
  v: Vec3,
}

#[derive(Clone, Debug)]
struct Vec3 {
  x: BigInt,
  y: BigInt,
  z: BigInt,
}

impl Vec3 {
  fn cross(&self, other: &Vec3) -> Vec3 {
    Vec3 {
      x: self.y.clone() * other.z.clone() - self.z.clone() * other.y.clone(),
      y: self.z.clone() * other.x.clone() - self.x.clone() * other.z.clone(),
      z: self.x.clone() * other.y.clone() - self.y.clone() * other.x.clone()
    }
  }

  fn dot(&self, other: &Vec3) -> BigInt {
    self.x.clone() * other.x.clone() + self.y.clone() * other.y.clone() + self.z.clone() * other.z.clone()
  }

  fn is_independant(&self, other: &Vec3) -> bool {
    let cross = self.cross(other);
    let zero = BigInt::new(Sign::NoSign, vec![0]);
    cross.x != zero || cross.y != zero || cross.z != zero
  }
}

impl Add<Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Self::Output {
    Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
  }
}

impl Sub<Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Vec3) -> Self::Output {
    Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
  }
}

impl Mul<&Vec3> for BigInt {
  type Output = Vec3;

  fn mul(self, rhs: &Vec3) -> Self::Output {
    Vec3{x: self.clone() * rhs.x.clone(), y: self.clone() * rhs.y.clone(), z: self.clone() * rhs.z.clone()}
  }
}

impl Div<BigInt> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: BigInt) -> Self::Output {
    Vec3{x: self.x / rhs.clone(), y: self.y / rhs.clone(), z: self.z / rhs.clone()}
  }
}

fn find_hailstone_position(h1: Hailstone, h2: Hailstone, h3: Hailstone) -> Vec3 {
  let (a, a_m) = find_plane(&h1, &h2);
  let (b, b_m) = find_plane(&h1, &h3);
  let (c, c_m) = find_plane(&h2, &h3);

  let t = a.dot(&b.cross(&c));
  let w = lin(&b.cross(&c), &c.cross(&a), &a.cross(&b), a_m, b_m, c_m) / t;
  let w1 = h1.v - w.clone();
  let w2 = h2.v - w.clone();
  let w_cross = w1.cross(&w2);

  let w1_m = w_cross.dot(&h2.p.cross(&w2));
  let w2_m = w_cross.dot(&h1.p.cross(&w1));
  let w_cross_m = h1.p.dot(&w_cross);

  let s = w_cross.dot(&w_cross);

  let stone = lin(&w1, &w2, &w_cross, w1_m, -w2_m, w_cross_m) / s;
  stone
}

fn find_plane(h1: &Hailstone, h2: &Hailstone) -> (Vec3, BigInt) {
  let p = h1.p.clone() - h2.p.clone();
  let v = h1.v.clone() - h2.v.clone();
  let c = h1.v.cross(&h2.v);
  (p.cross(&v), p.dot(&c))
}

fn lin(a: &Vec3, b: &Vec3, c: &Vec3, a_m: BigInt, b_m: BigInt, c_m: BigInt) -> Vec3 {
  let a = a_m * a;
  let b = b_m * b;
  let c = c_m * c;
  a + b + c
}

#[cfg(test)]
mod tests {
  use super::*;
  use num_bigint::ToBigInt;
use test::Bencher;
  use utils::read_file_to_string;

  const DAY: u8 = 24;
  const PART: utils::Part = utils::Part::B;

  #[test]
  fn test_day_24_b() {
    const EXAMPLE_ANSWER: Option<BigInt> = None;
    let answer: Option<BigInt> = 856642398547748i64.to_bigint();
    match utils::run_method::<BigInt>(&main, DAY, PART, (EXAMPLE_ANSWER, answer.clone())) {
      Err(message) => panic!("{}", message),
      Ok(val) if answer.is_none() => println!("Answer for day {DAY}-{} = {val}", PART.lower_name()),
      _ => (),
    }
  }

  #[bench]
  fn bench_day_24_b(b: &mut Bencher) {
    let input = read_file_to_string(utils::get_file_name(DAY, None).as_str());
    b.iter(|| main(input.clone()));
  }
}
