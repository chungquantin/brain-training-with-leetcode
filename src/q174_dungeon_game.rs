use std::cmp::{max, min};
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
 // Using bottom-up dynamic programming
 pub fn calculate_minimum_hp_dp(dungeon: Vec<Vec<i32>>) -> i32 {
  let (r, c) = (dungeon.len(), dungeon[0].len());
  let mut dp = vec![vec![i32::MIN; c]; r];

  for y in (0..r).rev() {
   for x in (0..c).rev() {
    let val = dungeon[y][x];
    if x == c - 1 && y == r - 1 {
     // princess position
     dp[y][x] = val;
    } else if x == c - 1 && y != r - 1 {
     // right
     let new_val = val + dp[y + 1][x];
     dp[y][x] = if new_val < 0 { new_val } else { 0 };
    } else if x != c - 1 && y == r - 1 {
     // bottom
     let new_val = val + dp[y][x + 1];
     dp[y][x] = if new_val < 0 { new_val } else { 0 };
    } else {
     let down = val + dp[y + 1][x];
     let right = val + dp[y][x + 1];
     dp[y][x] = max(down, right);
    }
    dp[y][x] = min(val, dp[y][x]);
   }
  }

  if dp[0][0] < 0 {
   dp[0][0] * -1 + 1
  } else {
   1
  }
 }

 // Second approach: Using Dijkstra's algorithm with Max Heap priority queue
 // Space complexity: O(N^2)
 // Time complexity: max(O(E + VlogV), O(VlogV)) = O(VlogV) with V = vertexes => V = N^2 => O(N^2.logN)
 pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
  // Initialize priority queue with max heap
  // Build a heap cost O(N*LogN)
  let mut pq = BinaryHeap::<(i32, usize, usize)>::new(); // Accummulate value, X, Y
  let (r, c) = (dungeon.len(), dungeon[0].len());
  // Construct a dp table to memoize value
  let mut dp = vec![vec![i32::MIN; c]; r];
  let mut visited = vec![vec![0; c]; r];
  let (kx, ky) = (0, 0); // Knight X, Knight Y
  let (px, py) = (c - 1, r - 1); // Princess X, Princess Y

  // Knight is at Princess position
  if px == kx && py == ky {
   if dungeon[0][0] < 0 {
    return dungeon[0][0] * -1 + 1;
   } else {
    return 1;
   }
  }

  // Start at the Knight position (0, 0)
  pq.push((dungeon[ky][kx], kx, ky));
  dp[ky][kx] = dungeon[ky][kx];
  let dirs = vec![(0, 1), (1, 0)];

  let mut answer = 0;
  // Dijkstra's algorithm: O(E + VlogV)
  // Try all posible route in queue
  while !pq.is_empty() {
   match pq.pop() {
    Some(node) => {
     let (val, x, y) = node;
     visited[y][x] = 1;
     answer = min(answer, val);
     // Until the princess is saved
     let princess_is_saved = x == px && y == py;
     if princess_is_saved {
      break;
     }

     for dir in dirs.iter() {
      let (next_x, next_y) = (x + dir.0, y + dir.1);
      if next_x < c && next_y < r {
       let new_val = val + dungeon[next_y][next_x];
       if (dp[next_y][next_x] == i32::MIN && visited[next_y][next_x] == 0)
        || (new_val > dp[next_y][next_x])
       {
        dp[next_y][next_x] = new_val;
        pq.push((new_val, next_x, next_y));
       }
      }
     }
    }
    None => {
     break;
    }
   }
  }

  if answer < 0 {
   answer * -1 + 1
  } else {
   1
  }
 }
}

#[test]
fn main() {
 assert_eq!(
  Solution::calculate_minimum_hp_dp(vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]]),
  7
 );

 assert_eq!(Solution::calculate_minimum_hp_dp(vec![vec![0]]), 1);
 assert_eq!(Solution::calculate_minimum_hp_dp(vec![vec![2], vec![1]]), 1);
 assert_eq!(
  Solution::calculate_minimum_hp_dp(vec![vec![0, 0, 0, -2]]),
  3
 );
 assert_eq!(
  Solution::calculate_minimum_hp_dp(vec![vec![0, 0, 3, -4]]),
  2
 );
 assert_eq!(Solution::calculate_minimum_hp_dp(vec![vec![-3, -5]]), 9);
 assert_eq!(Solution::calculate_minimum_hp_dp(vec![vec![-3, 5]]), 4);
 assert_eq!(
  Solution::calculate_minimum_hp_dp(vec![vec![1, -3, 3], vec![0, -2, 0], vec![-3, -3, -3]]),
  3
 );

 assert_eq!(Solution::calculate_minimum_hp_dp(vec![vec![100]]), 1);
}
