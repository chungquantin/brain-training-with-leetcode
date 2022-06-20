pub struct Solution;

use std::collections::HashMap;

impl Solution {
 pub fn can_construct(ransom_note: String, magazine: String) -> bool {
  let (mut occurence_ransom, mut occurence_magazine) =
   (HashMap::<char, i32>::new(), HashMap::<char, i32>::new());

  for c in magazine.chars() {
   let opt_cur = occurence_magazine.get(&c);
   if !opt_cur.is_none() {
    occurence_magazine.insert(c, opt_cur.unwrap() + 1);
   } else {
    occurence_magazine.insert(c, 1);
   }
  }

  for c in ransom_note.chars() {
   let opt_cur = occurence_ransom.get(&c);
   if !opt_cur.is_none() {
    occurence_ransom.insert(c, opt_cur.unwrap() + 1);
   } else {
    occurence_ransom.insert(c, 1);
   }
  }
  for key in occurence_ransom.keys() {
   let opt_mag = occurence_magazine.get(key);
   if opt_mag.is_none() || occurence_ransom.get(key).unwrap() > opt_mag.unwrap() {
    return false;
   }
  }

  true
 }
}
