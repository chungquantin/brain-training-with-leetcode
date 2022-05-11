pub struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        let mut missing_index: i32 = len as i32;
        let mut i = 0;
        while i < len {
            let current = i;
            let cur_num = nums[current];
            if cur_num == len as i32 {
                missing_index = current as i32;
            } else if current != (cur_num as usize) {
                let temp = nums[cur_num as usize];
                nums[current] = temp;
                nums[cur_num as usize] = cur_num;
                continue;
            }
            i += 1
        }

        missing_index
    }
}
