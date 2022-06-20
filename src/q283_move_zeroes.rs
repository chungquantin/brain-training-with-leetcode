pub struct Solution;

impl Solution {
 pub fn move_zeroes(nums: &mut Vec<i32>) {
  let mut i = 0;
  let mut offset = 0;
  while i < nums.len() - offset {
   if nums[i] == 0 {
    nums.push(nums[i]);
    nums.remove(i);
    offset += 1;
   } else {
    i += 1;
   }
  }
 }
}
