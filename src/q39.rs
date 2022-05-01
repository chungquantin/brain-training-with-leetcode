pub struct Solution;

impl Solution {
 pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  let result = &mut vec![];
  let dup_check = &mut std::collections::HashMap::<Vec<i32>, bool>::new();
  Solution::backtrack(nums, &mut target.clone(), &mut vec![], result, dup_check);
  result.to_vec()
 }

 pub fn backtrack(
  nums: Vec<i32>,
  remain: &mut i32,
  temp_list: &mut Vec<i32>,
  result: &mut Vec<Vec<i32>>,
  dup_check: &mut std::collections::HashMap<Vec<i32>, bool>,
 ) {
  if *remain == 0 {
   temp_list.sort();
   // if dup_check.get(temp_list).is_none() {
   result.push(temp_list.to_vec());
   // dup_check.insert(temp_list.to_vec(), true);
   return;
   // }
  }
  if *remain < 0 {
   return;
  }
  for i in 0..nums.len() {
   *remain -= nums[i];
   temp_list.push(nums[i]);
   Solution::backtrack(nums.to_vec(), remain, temp_list, result, dup_check);
   temp_list.pop();
  }
 }
}
