pub struct Solution;

impl Solution {
 pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
  let mut ans = 0;
  let visited = &mut vec![0; is_connected.len()];

  for node in 0..is_connected.len() {
   if visited[node] == 0 {
    visited[node] = 1;
    Solution::dfs(node, is_connected.to_vec(), visited);
    ans += 1;
   }
  }
  ans
 }

 pub fn dfs(node: usize, is_connected: Vec<Vec<i32>>, visited: &mut Vec<usize>) {
  for (node, adj) in is_connected[node].iter().enumerate() {
   if adj == &1 && visited[node] == 0 {
    visited[node] = 1;
    Solution::dfs(node, is_connected.to_vec(), visited);
   }
  }
 }
}
