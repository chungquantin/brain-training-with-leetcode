pub struct Solution;

impl Solution {
 // Using bytes array to store the frequency except map will reduce the space complexity
 // and avoid overhead (fixed-size array)
 pub fn min_window(s: String, t: String) -> String {
  let mut ans = (s.clone() + "a").clone();
  let mut freq: Vec<i32> = vec![0; 128];
  let (mut start, mut end) = (0, 0);
  let mut counter = t.len();

  for c in t.chars() {
   freq[c as usize] += 1;
  }

  while end < s.len() {
   let eb = s.bytes().nth(end).unwrap();
   if freq[eb as usize] > 0 {
    counter -= 1;
   }
   freq[eb as usize] -= 1;
   while counter == 0 {
    let slice = String::from(&s[start..end + 1]);
    if slice.len() < ans.len() {
     ans = slice;
    }
    let sb = s.bytes().nth(start).unwrap();
    if freq[sb as usize] == 0 {
     counter += 1;
    }
    freq[sb as usize] += 1;
    start += 1;
   }

   end += 1;
  }

  if ans.len() == s.len() + 1 {
   return String::from("");
  }

  ans
 }
}
