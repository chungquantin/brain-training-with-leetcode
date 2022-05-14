pub fn floyd_warshall_algorithm(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
 let mut m = build_adjacent_matrix(n, edges);
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

 m.to_vec()
}

// Build adjacent matrix for directed weighted graph
fn build_adjacent_matrix(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
 let mut adjacent_m: Vec<Vec<i32>> = vec![vec![i32::MAX; n as usize]; n as usize];

 for edge in edges {
  let (s, t, w) = (edge[0], edge[1], edge[2]);
  adjacent_m[s as usize][t as usize] = w;
 }

 for i in 0..n as usize {
  adjacent_m[i][i] = 0;
 }

 adjacent_m
}
