use std::collections::VecDeque;
pub struct Solution;

impl Solution {
 pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
  let mut queue = VecDeque::<(usize, usize)>::new();
  let (mut time, mut fresh_oranges) = (0, 0);
  let (mut rows, mut cols) = (grid.len(), grid[0].len());
  for r in 0..rows - 1 {
   for c in 0..cols - 1 {
    if grid[r][c] == 1 {
     fresh_oranges += 1
    }
    if grid[r][c] == 2 {
     queue.push_back((r, c));
    }
   }
  }
  if fresh_oranges == 0 {
   return 0;
  }

  let directions = vec![vec![0, 1], vec![0, -1], vec![1, 0], vec![-1, 0]];
  while queue.len() > 0 {
   for i in 0..queue.len() - 1 {
    let (r, c) = queue.pop_front().unwrap();
   }
  }
  0
 }
}
