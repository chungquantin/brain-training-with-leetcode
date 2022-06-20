pub struct BFSSolution;
pub struct DFSSolution;
pub struct SolutionHelper;

impl SolutionHelper {
 fn is_edge(row: i32, col: i32, grid: Vec<Vec<char>>) -> bool {
  row < 0 || row > (grid.len() - 1) as i32 || col < 0 || col > (grid[0].len() - 1) as i32
 }
}

impl DFSSolution {
 pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
  let grid = &mut grid.to_vec();
  let mut island_count = 0;
  for row in 0..grid.len() {
   for col in 0..grid[0].len() {
    if grid[row][col] == '1' {
     DFSSolution::erase_island(grid, row as i32, col as i32);
     island_count += 1;
    }
   }
  }
  island_count
 }

 pub fn erase_island(grid: &mut Vec<Vec<char>>, row: i32, col: i32) {
  if SolutionHelper::is_edge(row, col, grid.to_vec()) || grid[row as usize][col as usize] == '0' {
   return;
  }

  grid[row as usize][col as usize] = '0';

  let dirs: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
  for dir in dirs.to_vec() {
   DFSSolution::erase_island(grid, row + dir.0, col + dir.1);
  }
 }
}

impl BFSSolution {
 pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
  let mut island_count = 0;
  let visited = &mut vec![vec![0; grid[0].len()]; grid.len()];

  for row in 0..grid.len() {
   for col in 0..grid[0].len() {
    if visited[row][col] == 0 && grid[row][col] == '1' {
     BFSSolution::breadth_first_search(grid.to_vec(), visited, row, col);
     island_count += 1;
    }
   }
  }

  island_count
 }

 fn breadth_first_search(
  grid: Vec<Vec<char>>,
  visited: &mut Vec<Vec<i32>>,
  row: usize,
  col: usize,
 ) {
  visited[row][col] = 1;
  let dirs: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
  for dir in dirs.to_vec() {
   let (new_row, new_col) = (row as i32 + dir.0, col as i32 + dir.1);
   if !SolutionHelper::is_edge(new_row, new_col, grid.to_vec())
    && visited[new_row as usize][new_col as usize] == 0
    && grid[new_row as usize][new_col as usize] == '1'
   {
    BFSSolution::breadth_first_search(grid.to_vec(), visited, new_row as usize, new_col as usize);
   }
  }
 }
}
