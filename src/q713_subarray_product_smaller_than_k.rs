pub struct Solution;

impl Solution {
 pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
  let mut ans: i32 = 0;

  let mut left: usize = 0;
  let mut prod: i32 = 1;

  for right in 0..nums.len() {
   prod = prod * nums[right];

   while prod >= k && left <= right {
    prod = prod / nums[left];
    left += 1;
   }
   ans += (right - left + 1) as i32
  }

  ans
 }
}
