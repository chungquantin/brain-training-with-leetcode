pub struct Solution {}

// 220. Contains Duplicate III

impl Solution {
    // TIME: O(nlogk) - SPACE: O(1)
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut i: i32 = 0;
        loop {
            if i == (nums.len() - 1) as i32 {
                break;
            }
            let mut j: i32 = i + 1;
            loop {
                let (diff, _) = Solution::abs(&i, &j);
                let (diff_num, diff_num_overflow) =
                    Solution::abs(&nums[i as usize], &nums[j as usize]);
                if diff_num_overflow {
                    break;
                }
                if diff <= k && diff_num <= t {
                    return true;
                }
                if j >= k + i || j >= (nums.len() - 1) as i32 {
                    break;
                }
                j += 1;
            }
            i += 1;
        }
        false
    }

    fn abs(x: &i32, y: &i32) -> (i32, bool) {
        if x > y {
            let (_, is_overflowing) = x.overflowing_sub(*y);
            return (x.saturating_sub(*y), is_overflowing);
        }
        let (_, is_overflowing) = y.overflowing_sub(*x);
        return (y.saturating_sub(*x), is_overflowing);
    }
}
