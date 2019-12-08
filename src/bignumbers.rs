// reference for later

use num_rational::BigRational;
use num_traits::ops::checked::CheckedMul;

pub fn main() {
  let s1 = "531137992816767098";
  let s2 = "200137992816767098";
  let one: BigRational = s1.parse().unwrap();
  let two: BigRational = s2.parse().unwrap();
  let result = one.checked_mul(&two);
  println!("{:?}", result);
}
