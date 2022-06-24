pub struct NumMatrix {
 prefix_sum: Vec<Vec<i32>>,
 m: usize,
 n: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
 pub fn new(matrix: Vec<Vec<i32>>) -> Self {
  let m = matrix.len();
  let n = matrix[0].len();
  // Space complexity: O(1)
  // Using the Matrix (which is already in the heap)
  let mut ps = matrix.clone();

  // Time complexity: O(N)
  for c in 1..m {
   ps[0][c] += ps[0][c - 1];
  }

  // Time complexity: O(N)
  for r in 1..m {
   ps[r][0] += ps[r - 1][0];
  }

  // Time complexity: O(N^2)
  for i in 1..m {
   for j in 1..n {
    ps[i][j] += ps[i - 1][j] + ps[i][j - 1] - ps[i - 1][j - 1];
   }
  }

  NumMatrix {
   prefix_sum: ps,
   m,
   n,
  }
 }

 fn lookup(&self, row: usize, col: usize) -> i32 {
  if row < self.m && col < self.n {
   self.prefix_sum[row][col]
  } else {
   0
  }
 }

 pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
  let (row1, row2, col1, col2) = (row1 as usize, row2 as usize, col1 as usize, col2 as usize);
  self.lookup(row2, col2)
   - self.lookup(row2, col1.wrapping_sub(1))
   - self.lookup(row1.wrapping_sub(1), col2)
   + self.lookup(row1.wrapping_sub(1), col1.wrapping_sub(1))
 }
}
