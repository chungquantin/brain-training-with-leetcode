pub struct Solution;

impl Solution {
 pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
  let (mut left, mut right) = (0, nums.len() - 1);
  let mut last = right as i32;
  let mut new_vec: Vec<i32> = vec![0; nums.len()];
  for _ in 0..nums.len() {
   let (anl, anr) = (nums[left].abs(), nums[right].abs());
   if anl > anr {
    new_vec[last as usize] = anl.pow(2);
    left += 1;
   } else {
    new_vec[last as usize] = anr.pow(2);
    right -= 1;
   }
   last -= 1;
  }

  new_vec
 }
}
