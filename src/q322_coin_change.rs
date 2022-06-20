pub struct Solution;

impl Solution {
 // Time complexity: O(amount * coins.len())
 pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
  let mut dp = vec![amount + 1; amount as usize + 1];
  dp[0] = 0;

  for a in 1..amount + 1 {
   for coin in coins.iter() {
    if a - coin >= 0 {
     // Iterate through all coin types and compare the "1 + (a - coin)" with current value "a"
     dp[a as usize] = std::cmp::min(dp[a as usize], 1 + dp[(a - coin) as usize]);
    }
   }
  }

  return if dp[amount as usize] != amount + 1 {
   dp[amount as usize]
  } else {
   -1
  };
 }
}
