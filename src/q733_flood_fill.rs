pub struct Solution;

impl Solution {
 pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
  let mut image = image;
  let root_color = image[sr as usize][sc as usize];

  let mut stack = vec![(sr, sc)];
  while let Some((r, c)) = stack.pop() {
   let color = image[r as usize][c as usize];
   if color == new_color || color != root_color {
    continue;
   }
   image[r as usize][c as usize] = new_color;
   if r > 0 {
    stack.push((r - 1, c));
   }
   if r < image.len() as i32 - 1 {
    stack.push((r + 1, c));
   }
   if c > 0 {
    stack.push((r, c - 1));
   }
   if c < image[0].len() as i32 - 1 {
    stack.push((r, c + 1));
   }
  }

  image
 }
}
