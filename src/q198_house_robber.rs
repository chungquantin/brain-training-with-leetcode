pub struct Solution;

impl Solution {
 pub fn rob(nums: Vec<i32>) -> i32 {
  let mut memo = nums.to_vec();
  let mut answer = 0;

  let (mut ws, mut we) = (0, 2);

  if nums.len() < 3 {
   return std::cmp::max(nums[0], nums[nums.len() - 1]);
  }

  for num in nums.to_vec() {
   answer = std::cmp::max(num, answer);
  }

  while ws < nums.len() - 2 {
   let start = nums[ws];

   for i in [0, 1] {
    if we + i < nums.len() {
     let end = nums[we + i];
     let val = (memo[ws] + start + end) - if memo[ws] > 0 { start } else { 0 };
     memo[we + i] = std::cmp::max(memo[we + i], val);
     answer = std::cmp::max(answer, memo[we + i]);
    }
   }

   we += 1;
   ws += 1;
  }

  answer
 }
}
