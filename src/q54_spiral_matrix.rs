pub struct Solution;

impl Solution {
 pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
  let ans: &mut Vec<i32> = &mut vec![];
  let (r, c) = (matrix.len(), matrix[0].len());
  let visited: &mut Vec<Vec<i32>> = &mut vec![vec![0; c]; r];

  if c == 1 {
   for row in matrix.iter() {
    ans.push(row[0]);
   }
   ans.to_vec()
  } else {
   // Top : 0 - Right : 1 - Bottom : 2 - Left : 3
   Solution::dfs(
    matrix.to_vec(),
    (0, 0),
    &mut (1, c - 1, r - 1, 0),
    visited,
    ans,
    1,
   );
   ans.to_vec()
  }
 }

 fn dfs(
  matrix: Vec<Vec<i32>>,
  p: (usize, usize),
  b: &mut (usize, usize, usize, usize),
  visited: &mut Vec<Vec<i32>>,
  ans: &mut Vec<i32>,
  direction: i32,
 ) {
  let (x, y) = p;
  let (r, c) = (matrix.len(), matrix[0].len());
  if ans.len() == r * c {
   return;
  }
  let (tb, rb, bb, lb) = b;
  ans.push(matrix[y][x]);
  visited[y][x] = 1;

  match direction {
   0 => {
    if y == tb.clone() {
     b.0 += 1;
     Solution::dfs(matrix.to_vec(), (x + 1, y), b, visited, ans, 1);
    } else if y as i32 - 1 > -1 && visited[y - 1][x] == 0 {
     Solution::dfs(matrix.to_vec(), (x, y - 1), b, visited, ans, 0)
    }
   }
   1 => {
    if x == rb.clone() {
     b.1 -= 1;
     Solution::dfs(matrix.to_vec(), (x, y + 1), b, visited, ans, 2);
    } else if x + 1 < c && visited[y][x + 1] == 0 {
     Solution::dfs(matrix.to_vec(), (x + 1, y), b, visited, ans, 1)
    }
   }
   2 => {
    if y == bb.clone() {
     b.2 -= 1;
     Solution::dfs(matrix.to_vec(), (x - 1, y), b, visited, ans, 3);
    } else if y + 1 < r && visited[y + 1][x] == 0 {
     Solution::dfs(matrix.to_vec(), (x, y + 1), b, visited, ans, 2)
    }
   }
   3 => {
    if x == lb.clone() {
     b.3 += 1;
     Solution::dfs(matrix.to_vec(), (x, y - 1), b, visited, ans, 0);
    } else if x as i32 - 1 > -1 && visited[y][x - 1] == 0 {
     Solution::dfs(matrix.to_vec(), (x - 1, y), b, visited, ans, 3)
    }
   }
   _ => unreachable!(),
  }
 }
}
