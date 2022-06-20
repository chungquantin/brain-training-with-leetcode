pub struct Solution;

use std::collections::HashMap;

/// Another approach is sorting the `nums` vector first then have the two points (front, end)
impl Solution {
  pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
      return vec![];
    }
    let mut res: Vec<Vec<i32>> = vec![];
    let mut res_map = HashMap::<Vec<i32>, bool>::new();
    let mut s = HashMap::<i32, i32>::new();
    for num in nums.iter() {
      s.insert(*num, s.get(&num).unwrap_or(&0) + 1);
    }

    for i in 0..nums.len() - 1 {
      let mut cs = s.clone();
      cs.insert(nums[i], cs.get(&nums[i]).unwrap() - 1);
      for j in i + 1..nums.len() {
        let r = 0 - nums[i] - nums[j];
        cs.insert(nums[j], cs.get(&nums[j]).unwrap() - 1);
        if !cs.get(&r).is_none() {
          let o = cs.get(&r).unwrap();
          if o > &0 {
            let mut t = vec![nums[i], nums[j], r];
            t.sort();
            res_map.insert(t, true);
          }
        }
      }
    }
    for key in res_map.keys() {
      res.push(key.to_vec());
    }

    res
  }
}
