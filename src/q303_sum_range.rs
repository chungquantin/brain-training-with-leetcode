pub struct NumArray {
    s: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        NumArray { s: nums }
    }

    // Leetcode: Runtime: 115ms - Memory: 8.5MB
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sum = 0;
        let s = self.s.clone();
        let sub_array = &s[left as usize..(right + 1) as usize];
        for (i, _) in sub_array.into_iter().enumerate() {
            sum += &sub_array[i];
        }
        return sum;
    }
}
