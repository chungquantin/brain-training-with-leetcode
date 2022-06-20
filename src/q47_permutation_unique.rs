pub struct Solution;

impl Solution {
 pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let len = nums.len();
  let mut m_nums = nums;
  let result: &mut Vec<Vec<i32>> = &mut vec![];
  m_nums.sort();

  Solution::backtrack(m_nums, &mut vec![], result, &mut vec![false; len]);
  result.to_vec()
 }

 pub fn backtrack(
  nums: Vec<i32>,
  templist: &mut Vec<i32>,
  result: &mut Vec<Vec<i32>>,
  visited: &mut Vec<bool>,
 ) {
  if templist.len() == nums.to_vec().len() {
   result.push(templist.to_vec());
  } else {
   for i in 0..nums.len() {
    if visited[i] || i > 0 && nums[i] == nums[i - 1] && !visited[i - 1] {
     continue;
    }
    visited[i] = true;
    templist.push(nums[i]);
    Solution::backtrack(nums.to_vec(), templist, result, visited);
    visited[i] = false;
    templist.pop();
   }
  }
 }
}
