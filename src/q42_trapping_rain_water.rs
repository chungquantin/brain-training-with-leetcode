pub struct Solution;

impl Solution {
 // Solution: Using monotonic stack
 pub fn trap(height: Vec<i32>) -> i32 {
  let mut ans: i32 = 0;
  let mut stack: Vec<(i32, usize)> = vec![];

  for (ind, cur) in height.iter().enumerate() {
   while stack.len() > 1 && stack.last().unwrap().0 < *cur {
    let prev = stack.pop().unwrap();
    let cur_el = stack.last().unwrap();
    if cur_el.0 > prev.0 {
     let offset = if (cur_el.0 - cur) > 0 {
      (cur_el.0 - cur) * (ind - cur_el.1 - 1) as i32
     } else {
      0
     };
     let root = (cur_el.0 - prev.0) * ((ind - cur_el.1 - 1) as i32);
     ans += root - offset;
    }
   }
   stack.push((*cur, ind));
  }

  ans
 }

 // Solution: Using two pointer
 pub fn trap_using_two_pointers(height: Vec<i32>) -> i32 {
  let mut ans = 0;
  let (mut left_p, mut right_p) = (0, height.len() - 1);
  let (mut left_max, mut right_max) = (i32::MIN, i32::MIN);
  while left_p < right_p {
   if height[left_p] < height[right_p] {
    if height[left_p] > left_max {
     left_max = height[left_p];
    } else {
     ans += left_max - height[left_p];
     left_p += 1;
    }
   } else {
    if height[right_p] > right_max {
     right_max = height[right_p];
    } else {
     ans += right_max - height[right_p];
     right_p -= 1;
    }
   }
  }
  ans
 }
}
