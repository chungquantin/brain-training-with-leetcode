pub struct Solution;

impl Solution {
 pub fn fill_cups(amount: Vec<i32>) -> i32 {
  use std::collections::BinaryHeap;

  let mut ans = 0;
  let mut pq = BinaryHeap::<i32>::new();

  for num in amount.iter() {
   pq.push(*num);
  }

  loop {
   let mut max_first = pq.pop().unwrap_or(0);
   let mut max_second = pq.pop().unwrap_or(0);

   if max_first == 0 && max_second == 0 {
    break;
   }

   if max_first > 0 {
    max_first -= 1;
    pq.push(max_first);
   }

   if max_second > 0 {
    max_second -= 1;
    pq.push(max_second);
   }

   ans += 1
  }

  ans
 }
}
