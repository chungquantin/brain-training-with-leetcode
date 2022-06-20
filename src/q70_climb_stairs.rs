pub struct Solution {}

// 70. Climbing stairs

impl Solution {
    // Fibonnaci sequence theory
    // Bottom up dynamic programming
    // TIME: O(n) - SPACE: O(1)
    // LeetCode: Runtime: 0ms - Memory Usage: 2.2MB
    pub fn climb_stairs(n: i32) -> i32 {
        let mut one = 0;
        let mut two = 1;

        for _ in 0..n - 1 {
            let temp = one + two;
            one = two;
            two = temp;
        }

        return one + two;
    }
}
