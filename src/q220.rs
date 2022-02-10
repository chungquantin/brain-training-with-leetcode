pub struct Solution {}

// 220. Contains Duplicate III

impl Solution {
    // TIME: O(n^2) - SPACE: O(1)
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut i: i32 = 0;
        loop {
            if i == (nums.len() - 1) as i32 {
                break;
            }
            let mut j: i32 = i + 1;
            loop {
                if (Solution::abs(&i, &j) <= k)
                    && (Solution::abs(&nums[i as usize], &nums[j as usize]) <= t)
                {
                    return true;
                }
                if j == (nums.len() - 1) as i32 {
                    break;
                }
                j += 1;
            }
            i += 1;
        }
        return false;
    }

    fn abs(x: &i32, y: &i32) -> i32 {
        if x > y {
            return x.saturating_sub(*y);
        }
        return y.saturating_sub(*x);
    }
}
