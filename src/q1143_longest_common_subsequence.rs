pub struct Solution;

/* Table of DP results:
   S|    t  e  z  c  a  t  l  i  p  o  c  a
T ji| 0  1  2  3  4  5  6  7  8  9 10 11 12
----+--------------------------------------
  0 | 0  0  0  0  0  0  0  0  0  0  0  0  0
q 1 | 0  0  0  0  0  0  0  0  0  0  0  0  0
u 2 | 0  0  0  0  0  0  0  0  0  0  0  0  0
e 3 | 0  0  1  1  1  1  1  1  1  1  1  1  1
t 4 | 0  1  1  1  1  1  2  2  2  2  2  2  2
z 5 | 0  1  1  2  2  2  2  2  2  2  2  2  2
a 6 | 0  1  1  2  2  3  3  3  3  3  3  3  3
l 7 | 0  1  1  2  2  3  3  4  4  4  4  4  4
c 8 | 0  1  1  2  3  3  3  4  4  4  4  5  5
o 9 | 0  1  1  2  3  3  3  4  4  4  5  5  5
a 10| 0  1  1  2  3  4  4  4  4  4  5  5  6
t 11| 0  1  1  2  3  4  5  5  5  5  5  5  6
l 12| 0  1  1  2  3  4  5  6  6  6  6  6  6
*/

impl Solution {
 pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
  let (l1, l2) = (text1.len(), text2.len());
  let mut dp = vec![vec![0; l2 + 1]; l1 + 1];

  for i in 0..l1 + 1 {
   for j in 0..l2 + 1 {
    if i == 0 || j == 0 {
     dp[i][j] = 0;
    } else {
     dp[i][j] = std::cmp::max(dp[i][j - 1], dp[i - 1][j]);
     if text1.chars().nth(i - 1).unwrap() == text2.chars().nth(j - 1).unwrap() {
      dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j - 1] + 1);
     }
    }
   }
  }

  dp[l1][l2]
 }
}
