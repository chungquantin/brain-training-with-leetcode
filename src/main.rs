use std::borrow::Borrow;

mod q217;
fn main() {
    let nums = vec![1, 2, 3, 1];
    println!(
        "Leetcode solution 217: {:?}",
        q217::Solution::contains_duplicate(nums.borrow())
    );
    println!(
        "Leetcode solution 217 (v2): {:?}",
        q217::Solution::contains_duplicate_v2(nums.borrow())
    );
}
