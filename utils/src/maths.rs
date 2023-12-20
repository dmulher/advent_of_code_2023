pub trait Lcm {
  fn lcm(a: Self, b: Self) -> Self;
  fn gcd(a: Self, b: Self) -> Self;
}

impl Lcm for u64 {
  fn lcm(a: u64, b: u64) -> u64 {
    a * b / u64::gcd(a, b)
  }
  fn gcd(a: u64, b: u64) -> u64 {
    match b {
      0 => a,
      _ => u64::gcd(b, a % b)
    }
  }
}

