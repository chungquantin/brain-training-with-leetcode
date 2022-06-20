use std::collections::HashMap;

pub struct Solution {}
impl Solution {
 pub fn length_of_longest_substring(s: String) -> i32 {
  let (mut l, mut c): (usize, HashMap<&u8, usize>) = (0, HashMap::new());
  for (i, b) in s.as_bytes().iter().enumerate() {
   let clone = c.clone();
   if c.get(&b).is_none() {
    c.insert(b, i);
   } else {
    let pos = clone.get(b);
    for (k, j) in &clone {
     if c.get(k) > pos {
      c.insert(k, *j);
     } else {
      c.remove(k);
     }
    }
    c.insert(b, i);
   }
   let c_len = c.len();
   if l < c_len {
    l = c_len
   }
  }
  return l as i32;
 }
}
