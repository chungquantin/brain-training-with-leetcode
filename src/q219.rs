use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    // LEETCODE: RunTime: 52ms - Memory: 6.9MB
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 {
            return false;
        }
        let mut occur_map = HashMap::new();
        let ref_nums = &nums;
        for (i, num) in ref_nums.into_iter().enumerate() {
            if occur_map.contains_key(num) && (i - occur_map[num] <= k as usize) {
                return true;
            }
            occur_map.insert(num, i);
        }
        return false;
    }
}
