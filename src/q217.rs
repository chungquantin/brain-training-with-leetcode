use std::{collections::HashSet};

pub struct Solution {}
impl Solution {
    // LEETCODE: RunTime: 10ms - Memory: 3MB
    // TIME: O(n) - SPACE: O(n)
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for i in nums {
            if set.contains(&i) {
                return true;
            }
            set.insert(i);
        }
        false
    }
    // LEETCODE: RunTime: 23ms - Memory: 3.5MB
    pub fn contains_duplicate_v2(nums: Vec<i32>) -> bool {
        let set: HashSet<i32> = nums.clone().into_iter().collect();
        return set.len() != nums.clone().len();
    }
}
