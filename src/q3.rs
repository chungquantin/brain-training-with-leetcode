use std::collections::HashMap;

pub struct Solution {}
impl Solution {
 pub fn length_of_longest_substring(s: String) -> i32 {
  let (mut l, mut c): (usize, HashMap<&u8, bool>) = (0, HashMap::new());
  for b in s.as_bytes() {
   if c.len() == 0 {
    c.insert(b, true);
   } else if c.get(b).is_none() {
    c.insert(b, true);
   } else {
    c.clear();
    c.insert(b, true);
   }
   let c_len = c.len();
   if l < c_len {
    l = c_len
   }
  }
  return l as i32;
 }
}
