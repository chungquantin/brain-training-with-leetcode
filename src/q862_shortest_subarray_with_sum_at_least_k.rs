pub struct Solution;

impl Solution {
 pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
  let mut ans = i32::MAX;
  // Initialize Monotically Increasing Queue (convert this to #209)
  let mut deque = std::collections::VecDeque::<(i32, usize)>::new();
  // Applying Atomic Code strategy into this solution, we can braek it down into three main parts:
  let mut sum = 0;
  for i in 0..nums.len() {
   sum += nums[i];

   // First: Compare sum and target to update the answer
   if sum >= k {
    ans = std::cmp::min(ans, i as i32 + 1);
   }

   let mut cur = (i32::MIN, usize::MIN);
   // Second: Shrinking down the window size from front (Similar to Kadance's algorithm)
   while !deque.is_empty() && sum - deque.front().unwrap().0 >= k {
    cur = *deque.front().unwrap();
    deque.pop_front();
   }

   // Second (continue): After shrinking down, update the answer
   if cur.0 != i32::MIN {
    ans = std::cmp::min(ans, i as i32 - cur.1 as i32);
   }

   // If the new number element breaks the monotorically increasing pattern,
   // Remove element higher than current sum from deque
   while !deque.is_empty() && deque.back().unwrap().0 >= sum {
    deque.pop_back();
   }
   deque.push_back((sum, i));
  }

  return if ans == i32::MAX { -1 } else { ans };
 }
}
