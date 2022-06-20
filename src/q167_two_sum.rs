pub struct Solution;

impl Solution {
 pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
  let mut i = 0;
  while i < numbers.len() - 1 {
   let j = i + 1;
   while j < number.len() {
    let (mut p1, mut p2) = (numbers[i], numbers[j]);
    if p1 + p2 == target {
     return vec![i, j];
    }
    j += 1;
   }
   i += 1;
  }
 }
 vec![]
}
