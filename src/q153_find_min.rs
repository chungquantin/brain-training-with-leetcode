pub struct Solution;

impl Solution {
 pub fn find_min(nums: Vec<i32>) -> i32 {
  let (front, last) = (nums[0], nums[nums.len() - 1]);
  if front > last {
   let mut i = nums.len() - 1;
   let mut next = nums[i - 1];
   let mut min = last;
   // println!("{} {} {:?}", next, min, nums.to_vec());
   while next <= min {
    min = next;
    next = nums[i - 1];
    i -= 1;
   }
   return min;
  } else {
   return front;
  }
 }
}
