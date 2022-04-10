pub struct Solution;

impl Solution {
 pub fn zero_one_knapsack(values: Vec<i32>, weights: Vec<i32>, w: i32) -> i32 {
  let n = weights.len();
  let mut dp = vec![vec![w as usize + 1; w as usize + 1]; n + 1];

  dp[0][0] = 0;
  for i in 1..(n + 1) {
   for wi in 1..(w as usize + 1) {
    if weights[i - 1] <= w {
     dp[i][wi] = std::cmp::max(
      values[i - 1] as usize + dp[i - 1][wi - weights[i - 1] as usize],
      dp[i - 1][wi],
     )
    } else {
     dp[i][wi] = dp[i - 1][wi];
    }
   }
  }

  dp[n as usize][w as usize] as i32
 }
}
