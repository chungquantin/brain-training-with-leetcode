use std::collections::HashMap;

pub struct Solution;

impl Solution {
  // Time limit exceeded
  pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut map = HashMap::new();
    for word in word_dict.iter() {
      map.insert(word, true);
    }
    let mut breakable = false;
    let mut word: String = String::from("");
    for c in s.chars() {
      word.push_str(&c.to_string());
      if map.get(&word).is_none() {
        continue;
      } else {
        if true
          && if word.len() < s.len() {
            Solution::word_break(s[word.len()..s.len()].to_string(), word_dict.to_vec())
          } else {
            true
          }
        {
          breakable = true;
          break;
        };
      }
    }

    return breakable;
  }
}
