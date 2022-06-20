pub struct Solution;

impl Solution {
 pub fn search(nums: Vec<i32>, target: i32) -> i32 {
  let len = nums.len();
  if len == 1 && nums[0] == target {
   return 0;
  }

  if target >= nums[0] {
   let mut i = 0;
   while i < len {
    let cur = nums[i];
    if cur == target {
     return i as i32;
    }
    if cur < nums[i] {
     return -1;
    }
    i += 1;
   }
  } else if target <= nums[len - 1] {
   let mut i = len - 1;
   while i > 0 {
    let cur = nums[i];
    if cur == target {
     return i as i32;
    }
    if cur > nums[i] {
     return -1;
    }
    i -= 1;
   }
  }

  return -1;
 }
}
