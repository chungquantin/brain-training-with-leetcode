pub struct Solution {}

impl Solution {
    // TIME: O(n^2) - SPACE: O(1)
    // Time Limit Exceeded
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut k = 1;
        let mut final_sum: i32 = nums[0];
        loop {
            let mut window_sum: i32 = 0;
            if k > nums.len() {
                break;
            }
            for i in 0..k {
                window_sum += nums[i];
            }
            if window_sum > final_sum {
                final_sum = window_sum
            }
            for j in k..nums.len() {
                let sum: i32 = (window_sum - nums[j - k]) + nums[j];
                window_sum = sum;
                if final_sum < window_sum {
                    final_sum = window_sum
                }
            }
            k += 1;
        }
        return final_sum;
    }

    // Two pointer
    // TIME: O(n) - SPACE: O(1)
    // Leetcode: Runtime: 19ms - Memory: 3.3MB
    pub fn max_sub_array_v2(nums: Vec<i32>) -> i32 {
        let mut m = nums[0];
        let mut e = nums[0];
        for i in 1..nums.len() {
            if nums[i] > e + nums[i] {
                e = nums[i];
            } else {
                e += nums[i];
            }
            if e > m {
                m = e
            }
        }
        return m;
    }

    pub fn max_sub_array_prefix_sum(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut prefix_sum: Vec<i32> = vec![i32::MIN; nums.len() + 1];
        prefix_sum[0] = 0;
        for ind in 1..nums.len() + 1 {
            prefix_sum[ind] = prefix_sum[ind - 1] + nums[ind - 1];
        }
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        for ind in 0..nums.len() {
            min = std::cmp::min(min, prefix_sum[0]);
            max = std::cmp::max(max, prefix_sum[ind + 1] - min);
        }
        max
    }
}
