pub struct Solution;

impl Solution {
 pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
  let mut m = Solution::build_adjacent_matrix(n, edges);
  for k in 0..n as usize {
   for i in 0..n as usize {
    for j in 0..n as usize {
     if m[i][k] == i32::MAX || m[k][j] == i32::MAX {
      continue;
     }
     m[i][j] = std::cmp::min(m[i][j], m[i][k] + m[k][j])
    }
   }
  }
  let mut temp_result: (i32, i32) = (0, 0);
  for r in 0..n as usize {
   let mut temp: (i32, i32) = (0, 0);
   for i in 0..n as usize {
    if m[r][i] <= distance_threshold {
     temp = (r as i32, temp.1 + 1);
    }
   }
   if r == 0 || temp.0 > temp_result.0 && temp.1 <= temp_result.1 {
    temp_result = temp;
   }
  }

  temp_result.0
 }

 // Build adjacency matrix for bi-directional graph
 fn build_adjacent_matrix(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut adjacent_m: Vec<Vec<i32>> = vec![vec![i32::MAX; n as usize]; n as usize];

  for edge in edges {
   let (s, t, w) = (edge[0], edge[1], edge[2]);
   adjacent_m[s as usize][t as usize] = w;
   adjacent_m[t as usize][s as usize] = w;
  }

  for i in 0..n as usize {
   adjacent_m[i][i] = 0;
  }

  adjacent_m
 }
}
