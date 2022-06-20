pub struct Solution;

impl Solution {
 pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let len = nums.len();
  let mut temp: Vec<i32> = vec![];
  let mut result: Vec<Vec<i32>> = vec![];

  let mut stack: Vec<(Vec<i32>, usize)> = vec![(nums, 0)];
  while let Some((s_nums, start_index)) = stack.pop() {
   let n = s_nums.to_vec();
   for i in start_index..len {
    temp.push(n[i]);
    stack.push((n.to_vec(), i + 1));
   }
   result.push(temp.to_vec());
   temp.pop();
  }
  result
 }
}
