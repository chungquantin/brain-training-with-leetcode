pub struct Solution;

impl Solution {
 pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
  let mut max_area: i32 = 0;
  let mut stack: Vec<(usize, usize, i32)> = vec![]; // Monotonic Stack : Vec<(Index, Height)>

  for (index, height) in heights.iter().enumerate() {
   let mut start = index;
   while !stack.is_empty() && heights[stack.iter().last().unwrap().0] > *height {
    println!("{:?} {}", stack, max_area);
    let (_, top_index, top_height) = stack.pop().unwrap();
    start = top_index;
    let width = (index - top_index) as i32;
    max_area = std::cmp::max(max_area, top_height * width);
   }
   stack.push((index, start, *height));
  }

  println!("{:?} {}", stack, max_area);

  while !stack.is_empty() {
   let (_, top_index, top_height) = stack.pop().unwrap();
   let width = (heights.len() - top_index) as i32;
   max_area = std::cmp::max(max_area, top_height * width);
  }

  max_area
 }
}
