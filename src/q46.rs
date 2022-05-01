pub struct Solution;

impl Solution {
 pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let len = nums.len();
  let result: &mut Vec<Vec<i32>> = &mut vec![];
  if len == 0 {
   return result.to_vec();
  }

  Solution::backtrack(nums, &mut vec![], result);
  result.to_vec()
 }

 pub fn backtrack(nums: Vec<i32>, templist: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
  if templist.len() == nums.to_vec().len() {
   result.push(templist.to_vec());
  } else {
   for i in 0..nums.len() {
    if templist.contains(&nums[i]) {
     continue;
    };
    templist.push(nums[i]);
    Solution::backtrack(nums.to_vec(), templist, result);
    templist.pop();
   }
  }
 }
}
