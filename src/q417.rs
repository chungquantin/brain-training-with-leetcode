pub struct Solution;
use std::collections::VecDeque;

// There is an error with this code, waiting for help
impl Solution {
 pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut result: Vec<Vec<i32>> = vec![];
  if heights.len() == 0 || heights[0].len() == 0 {
   return result;
  }
  let (v, h) = (heights.len(), heights[0].len());
  let pacific = &mut vec![vec![0; h]; v]; // top - left
  let alantic = &mut vec![vec![0; h]; v]; // right - bottom
  let p_points = &mut VecDeque::<(usize, usize)>::new();
  let a_points = &mut VecDeque::<(usize, usize)>::new();
  (0..v).for_each(|r| {
   p_points.push_back((r, 0));
   a_points.push_back((r, h - 1));
   pacific[r][0] = 1;
   alantic[r][h - 1] = 1;
  });
  (0..h).for_each(|c| {
   p_points.push_back((0, c));
   a_points.push_back((v - 1, c));
   pacific[0][c] = 1;
   alantic[v - 1][c] = 1;
  });
  Solution::bfs(heights.to_vec(), p_points, pacific);
  Solution::bfs(heights.to_vec(), a_points, alantic);
  for row in 0..v {
   for col in 0..h {
    if pacific[row][col] == 1 && alantic[row][col] == 1 {
     result.push(vec![row as i32, col as i32]);
    }
   }
  }

  result
 }

 pub fn bfs(
  heights: Vec<Vec<i32>>,
  queue: &mut VecDeque<(usize, usize)>,
  visited: &mut Vec<Vec<i32>>,
 ) {
  while let Some((row, col)) = queue.pop_front() {
   let dirs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
   for (_, dir) in dirs.to_vec().iter().enumerate() {
    let (new_row, new_col) = (row as i32 + dir.0, col as i32 + dir.1);
    if new_row < 0
     || new_row >= heights.len() as i32
     || new_col < 0
     || new_col >= heights[0].len() as i32
     || visited[new_row as usize][new_col as usize] == 1
     || heights[new_row as usize][new_col as usize] > heights[row][col]
    {
     continue;
    }
    visited[new_row as usize][new_col as usize] = 1;
    queue.push_back((new_row as usize, new_col as usize));
   }
  }
 }
}
