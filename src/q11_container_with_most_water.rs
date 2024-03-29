pub struct Solution;

impl Solution {
 pub fn max_area(height: Vec<i32>) -> i32 {
  let mut max = 0;
  let s = height.len();

  let (mut left, mut right) = (0, s - 1);
  while left < right {
   println!("{} {}", left, right);
   let h = std::cmp::min(height[left], height[right]);
   max = std::cmp::max(max, h * (right - left) as i32);
   if height[left] < height[right] {
    left += 1;
   } else {
    right -= 1;
   }
  }

  max
 }
}
