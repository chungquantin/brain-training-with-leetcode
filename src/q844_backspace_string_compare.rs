pub struct Solution;

impl Solution {
 pub fn backspace_compare(s: String, t: String) -> bool {
  let (mut stack_s, mut stack_t) = (vec![], vec![]);

  for c in s.chars() {
   if c == '#' {
    stack_s.pop();
   } else {
    stack_s.push(c);
   }
  }

  for c in t.chars() {
   if c == '#' {
    stack_t.pop();
   } else {
    stack_t.push(c);
   }
  }
  stack_s == stack_t
 }
}
