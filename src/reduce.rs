use crate::params::*;

pub const QINV: i32 = 58728449; // q^(-1) mod 2^32
pub const Q_PRIME: i64 = 4236238847; // q * q' = -1 mod 2^32

/// For finite field element a with -2^{31}Q <= a <= Q*2^31,
/// compute r \equiv a*2^{-32} (mod Q) such that -Q < r < Q.
///
/// Returns r.
// pub fn montgomery_reduce(a: i64) -> i32 {
//   let mut t = (a as i32).wrapping_mul(QINV) as i64;
//   t = (a as i64 - t * Q as i64) >> 32;
//   t as i32
// }

pub fn montgomery_reduce(a: i64) -> i32 {
  let mut t = (a as i32).wrapping_mul(Q_PRIME as i32) as i64;
  t = t & 0xFFFFFFFF; // mod 2^32
  let mut u = t * Q as i64;
  u = a.wrapping_add(u);
  t = u >> 32 as i64;
  if t >= Q as i64 {
    t -= Q as i64;
  }
  t as i32
}

/// For finite field element a with a <= 2^{31} - 2^{22} - 1,
/// compute r \equiv a (mod Q) such that -6283009 <= r <= 6283007.
//
/// Returns r.
// pub fn reduce32(a: i32) -> i32 {
//   let mut t = (a + (1 << 22)) >> 23;
//   t = a - t * Q as i32;
//   // println!("a: {}, t: {}", a, t);
//   t
// }

// input を -Q/2 < input <= Q/2 にする
pub fn reduce32(a: i32) -> i32 {
  let t;
  if a > Q as i32/ 2 {
    t = a - Q as i32;
  } else if a <= -(Q as i32) / 2 {
    t = a + Q as i32;
  } else {
    t = a;
  }
  assert!(t >= -(Q as i32) / 2 && t <= Q as i32 / 2, "a: {}", a);
  t
}

/// Add Q if input coefficient is negative.
///
/// Returns r.
pub fn caddq(a: i32) -> i32 {
  a + ((a >> 31) & Q as i32)
}
