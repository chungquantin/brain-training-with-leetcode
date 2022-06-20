pub struct Solution;

impl Solution {
 pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let mut mut_nums = nums.to_vec();
  let len = nums.len();
  mut_nums.sort();
  let temp: &mut Vec<i32> = &mut vec![];
  let mut dup_check = std::collections::HashMap::<Vec<i32>, bool>::new();
  let mut result: Vec<Vec<i32>> = vec![];
  if len == 0 {
   return result;
  }

  let mut stack: Vec<(Vec<i32>, usize)> = vec![(mut_nums, 0)];
  while let Some((s_nums, start_index)) = stack.pop() {
   let n = s_nums.to_vec();
   for i in start_index..len {
    temp.push(n[i]);
    stack.push((n.to_vec(), i + 1));
   }
   if dup_check.get(&temp.clone()).is_none() {
    dup_check.insert(temp.to_vec(), true);
    result.push(temp.to_vec());
   }
   temp.pop();
  }
  result
 }
}
