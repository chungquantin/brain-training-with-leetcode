pub struct Solution;

impl Solution {
 pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
  let mut ans: Vec<i32> = vec![];
  let mut pos = std::collections::HashMap::<i32, i32>::new();

  let mut i = nums2.len() - 1;
  while i > 0 {
   let mut stack = nums2[0..i].to_vec();
   while stack.len() > 0 && stack.last().unwrap() < &nums2[i] {
    pos.insert(stack.pop().unwrap(), nums2[i]);
   }
   i -= 1;
  }

  for num in nums1 {
   if pos.get(&num).is_none() {
    ans.push(-1);
   } else {
    ans.push(*pos.get(&num).unwrap());
   };
  }

  ans
 }
}
