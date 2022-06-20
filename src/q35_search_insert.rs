pub struct Solution;

impl Solution {
 pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
  let (mut s, mut e) = (0, nums.len() - 1);
  while s <= e {
   let m = (e + s) / 2;
   if target == nums[m] {
    return m as i32;
   } else if target < nums[m] {
    if m == 0 {
     break;
    }
    e = m - 1;
   } else if target > nums[m] {
    s = m + 1;
   }
  }
  return s as i32;
 }
}
