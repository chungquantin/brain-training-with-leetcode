pub struct Solution {}

// 136. Single number (Bitwise Manipulation with XOR)
// TIME: O(n) - SPACE: O(1)
// Runtime: 0ms - Memory: 2.3MB
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let res_num = nums.into_iter().reduce(|a, b| a ^ b).unwrap_or(0);
        return res_num;
    }
}
