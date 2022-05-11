pub struct Solution;

impl Solution {
 pub fn find_duplicate(nums: Vec<i32>) -> i32 {
  let mut nums = nums;
  let len = nums.len();
  let mut duplicate_num: i32 = len as i32;
  let mut i = 0;
  while i < len {
   let current = i;
   let cur_num = nums[current];
   if current + 1 != (cur_num as usize) {
    let temp = nums[cur_num as usize - 1];
    if temp == cur_num {
     duplicate_num = temp;
     break;
    }
    nums[current] = temp;
    nums[cur_num as usize - 1] = cur_num;
    continue;
   }
   i += 1
  }

  duplicate_num
 }

 pub fn find_duplicate_tortoise_hare(nums: Vec<i32>) -> i32 {
  let (mut tortoise, mut hare) = (nums[0], nums[nums[0] as usize]);
  while tortoise != hare {
   tortoise = nums[tortoise as usize];
   hare = nums[nums[hare as usize] as usize];
  }
  tortoise = 0;
  while tortoise != hare {
   tortoise = nums[tortoise as usize];
   hare = nums[hare as usize]
  }
  tortoise
 }
}
