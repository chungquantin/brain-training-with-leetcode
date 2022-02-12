pub struct Solution {}

impl Solution {
    // TIME: O(n) - SPACE: (1)
    // LeetCode: Runtime: 3ms - Memory Usage: 2.2MB
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut total: i32 = nums.len() as i32 * (nums.len() as i32 + 1) / 2;
        for num in nums {
            total -= num;
        }
        total
    }
}
