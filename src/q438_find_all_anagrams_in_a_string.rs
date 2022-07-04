pub struct Solution;

impl Solution {
 pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
  use std::collections::HashMap;
  use std::collections::VecDeque;

  let mut ans = vec![];
  let mut w = VecDeque::<(char, usize)>::new();
  let mut freq = HashMap::<char, i32>::new();

  for c in p.chars() {
   if freq.get(&c).is_none() {
    freq.insert(c, 1);
   } else {
    freq.insert(c, freq.get(&c).unwrap() + 1);
   }
  }

  for (ind, c) in s.chars().enumerate() {
   w.push_back((c, ind));
   if !freq.get(&c).is_none() {
    freq.insert(c, freq.get(&c).unwrap() - 1);
   }
   if w.len() == p.len() {
    let (front_el, front_ind) = w.pop_front().unwrap();
    if freq.values().find(|v| v > &&0).take().is_none() {
     ans.push(front_ind as i32);
    }
    if !freq.get(&front_el).is_none() {
     freq.insert(front_el, freq.get(&front_el).unwrap() + 1);
    }
   }
  }

  ans
 }
}
