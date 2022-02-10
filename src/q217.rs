use std::{collections::HashSet};

pub struct Solution {}
// LEETCODE: RunTime: 10ms - Memory: 3MB
impl Solution {
    pub fn contains_duplicate(nums: &Vec<i32>) -> bool {
        let c_nums = nums.clone();
        let mut set = HashSet::new();
        for i in c_nums {
            if set.contains(&i) {
                return true;
            }
            set.insert(i);
        }
        false
    }
    pub fn contains_duplicate_v2(nums: &Vec<i32>) -> bool {
        let c_nums = nums.clone();
        let set: HashSet<i32> = c_nums.into_iter().collect();
        return set.len() != nums.clone().len();
    }
}

