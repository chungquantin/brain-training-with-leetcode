pub struct Solution;

impl Solution {
 // Using stack
 pub fn longest_palindrome(s: String) -> i32 {
  let mut stack = vec![];
  let mut count = 0;
  let mut chars: Vec<char> = s.chars().collect();
  chars.sort();
  for c in chars {
   if stack.len() != 0 && stack[stack.len() - 1] == c {
    stack.pop();
    count += 2;
   } else {
    stack.push(c);
   }
  }
  if stack.len() > 0 {
   count + 1
  } else {
   count
  }
 }
}
