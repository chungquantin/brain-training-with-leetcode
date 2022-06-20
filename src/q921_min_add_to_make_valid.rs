pub struct Solution;

impl Solution {
 pub fn min_add_to_make_valid(s: String) -> i32 {
  let mut moves = 0;
  let mut stack = vec![];
  for c in s.chars() {
   if c == '(' {
    stack.push(c);
   } else if c == ')' {
    if stack.len() > 0 {
     stack.pop();
    } else {
     moves += 1;
    }
   }
  }

  moves + stack.len() as i32
 }
}
