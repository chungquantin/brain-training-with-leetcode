use std::collections::VecDeque;
pub struct Solution;

impl Solution {
 pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
  let mut grid = grid;
  let mut queue = VecDeque::<(usize, usize)>::new();
  let (mut time, mut fresh_oranges) = (0, 0);
  let (rows, cols) = (grid.len(), grid[0].len());
  for r in 0..rows {
   for c in 0..cols {
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

  let dirs: Vec<Vec<i32>> = vec![vec![0, 1], vec![0, -1], vec![1, 0], vec![-1, 0]];
  while queue.len() > 0 && fresh_oranges > 0 {
   for _ in 0..queue.len() {
    let (r, c) = queue.pop_front().unwrap();
    for dir in dirs.iter() {
     let (row, col) = (dir[0] + r as i32, dir[1] + c as i32);
     if row < 0
      || row == grid.len() as i32
      || col < 0
      || col == grid[0].len() as i32
      || grid[row as usize][col as usize] != 1
     {
      continue;
     }
     grid[row as usize][col as usize] = 2;
     queue.push_back((row as usize, col as usize));
     fresh_oranges -= 1;
    }
   }
   time += 1;
  }
  return if fresh_oranges > 0 { -1 } else { time };
 }
}
