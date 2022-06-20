use std::collections::VecDeque;

pub struct Solution;

impl Solution {
 pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut mat = mat;
  let (rows, cols) = (mat.len(), mat[0].len());
  let mut queue = VecDeque::new();

  for i in 0..rows {
   for j in 0..cols {
    if mat[i][j] == 0 {
     queue.push_back((i, j));
    } else {
     mat[i][j] = -1;
    }
   }
  }

  let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
  while let Some((r, c)) = queue.pop_front() {
   for (dr, dc) in dirs.iter() {
    let (row, col) = (r as i32 + dr, c as i32 + dc);
    if row < 0
     || row == mat.len() as i32
     || col < 0
     || col == mat[0].len() as i32
     || mat[row as usize][col as usize] != -1
    {
     continue;
    }
    mat[row as usize][col as usize] = mat[r][c] + 1;
    queue.push_back((row as usize, col as usize));
   }
  }

  mat
 }
}
