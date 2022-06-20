pub struct Solution;

impl Solution {
 pub fn check_inclusion(s1: String, s2: String) -> bool {
  if s1.len() > s2.len() {
   return false;
  }
  let (mut f1, b1, mut sum1) = (vec![0; 26], s1.as_bytes(), 0u64);
  for b in b1 {
   f1[(b - 97) as usize] += 1;
   sum1 += *b as u64;
  }
  let (b2, mut window, mut wc, mut f2, mut i) = (s2.as_bytes(), 0u64, 0, vec![0; 26], 0);
  while i <= s2.len() {
   if window < sum1 && wc < s1.len() {
    if i != s2.len() {
     f2[(b2[i] - 97) as usize] += 1;
     window += b2[i] as u64;
     wc += 1;
    }
    i += 1;
   } else {
    if window == sum1 {
     let mut is_false = false;
     for c in 0..26 {
      if f1[c] != f2[c] {
       is_false = true
      }
     }
     if !is_false {
      return true;
     }
    }
    if i == s2.len() {
     break;
    }
    f2[(b2[i - b1.len()] - 97) as usize] -= 1;
    window -= b2[i - b1.len()] as u64;
    wc -= 1;
   }
  }
  false
 }
}
