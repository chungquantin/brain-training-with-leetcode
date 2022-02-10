use std::collections::HashSet;

// LEETCODE: RunTime: 10ms - Memory: 3MB
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool{
        let mut set = HashSet::new();
        for i in nums {
            if set.contains(&i) {
                return true;
            }
            set.insert(i);
        }
        false
    }
}
