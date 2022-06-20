pub struct Solution;

impl Solution {
 pub fn length_of_last_word(s: String) -> i32 {
  if let Some(last) = s.split_whitespace().last() {
   last.len() as i32
  } else {
   0
  }
 }
}
