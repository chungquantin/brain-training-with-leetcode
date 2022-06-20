pub struct Solution;

impl Solution {
 pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
  let l = nums.len();
  let (mut l2r, mut r2l, mut result) = (vec![0; l], vec![0; l], vec![0; l]);
  for i in 0..nums.len() {
   if i == 0 {
    l2r[i] = 1;
    r2l[l - 1 - i] = 1;
   } else if i == 1 {
    l2r[i] = nums[i - 1];
    r2l[l - 1 - i] = nums[l - i];
   } else if i > 1 {
    l2r[i] = l2r[i - 1] * nums[i - 1];
    r2l[l - 1 - i] = r2l[l - i] * nums[l - i];
   }
  }

  for i in 0..l {
   result[i] = l2r[i] * r2l[i];
  }

  result
 }
}
