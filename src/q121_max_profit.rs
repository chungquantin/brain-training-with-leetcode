pub struct Solution {}

impl Solution {
    // Sliding Windows
    // 121. Best Time To Buy and Sell Stock
    // Leetcode: RunTime: 22ms - Memory: 2.8MB
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut b: i32 = 0;
        let mut s: i32 = 1;
        let mut profit: i32 = 0;
        loop {
            if s as usize == prices.len() {
                break;
            }
            if prices[s as usize] < prices[b as usize] {
                b = s;
                s += 1;
            } else {
                let p = prices[s as usize] - prices[b as usize];
                if p > profit {
                    profit = p;
                }
                s += 1;
            }
        }
        profit
    }
}
