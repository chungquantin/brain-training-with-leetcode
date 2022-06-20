pub struct Solution;

impl Solution {
 pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
  let mut grid = grid;
  let (h, w) = (grid.len(), grid[0].len());
  fn dfs(i: i32, j: i32, h: i32, w: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
   if 0 <= i && i < h && 0 <= j && j < w {
    let area = grid[i as usize][j as usize];
    if area == 1 {
     grid[i as usize][j as usize] = 0;
     return 1
      + dfs(i - 1, j, h, w, grid)
      + dfs(i + 1, j, h, w, grid)
      + dfs(i, j - 1, h, w, grid)
      + dfs(i, j + 1, h, w, grid);
    }
   }
   return 0;
  }
  let mut max = vec![];
  for i in 0..h {
   for j in 0..w {
    max.push(dfs(i as i32, j as i32, h as i32, w as i32, &mut grid));
   }
  }
  return *max.iter().max().unwrap();
 }
}
