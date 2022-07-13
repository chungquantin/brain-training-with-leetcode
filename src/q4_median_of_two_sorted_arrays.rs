pub struct Solution;

impl Solution {
 pub fn find_median_sorted_arrays(n1: Vec<i32>, n2: Vec<i32>) -> f64 {
  use std::cmp::{max, min};
  use std::mem::swap;

  // a - small, b - large
  let (a, b) = (&mut n1.clone(), &mut n2.clone());
  let total = n1.len() + n2.len();
  let half = total / 2;

  if a.len() > b.len() {
   swap(a, b);
  }

  let (mut l, mut r): (i32, i32) = (0, a.len() as i32 - 1);

  loop {
   let i: i32 = (l + r) as i32 / 2; // A
   let j: i32 = half as i32 - i as i32 - 2; // B

   let (left_a, right_a) = (
    if i >= 0 { a[i as usize] } else { i32::MIN },
    if (i as i32 + 1) < a.len() as i32 {
     a[i as usize + 1]
    } else {
     i32::MAX
    },
   );
   let (left_b, right_b) = (
    if j >= 0 { b[j as usize] } else { i32::MIN },
    if (j as i32 + 1) < b.len() as i32 {
     b[j as usize + 1]
    } else {
     i32::MAX
    },
   );

   if left_a <= right_b && left_b <= right_a {
    if total % 2 == 1 {
     return min(right_a, right_b) as f64;
    } else {
     return (max(right_a, right_b) as f64 + min(left_a, left_b) as f64) / 2.0;
    }
   } else if left_a > right_b {
    r = i - 1;
   } else {
    l = i + 1;
   }
  }
 }
}
