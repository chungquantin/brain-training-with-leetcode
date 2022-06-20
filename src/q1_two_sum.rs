pub struct Solution;

impl Solution {
 pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let (mut a, mut b): (usize, usize) = (0, 0);
  let mut map = std::collections::HashMap::new();
  for (k, v) in nums.iter().enumerate() {
   if map.get(&(target - v)).is_none() {
    map.insert(v, k);
   } else {
    a = k;
    b = *map.get(&(target - v)).unwrap();
   }
  }
  if a > b {
   return vec![b as i32, a as i32];
  } else {
   return vec![a as i32, b as i32];
  }
 }
}
