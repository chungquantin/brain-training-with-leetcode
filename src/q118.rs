pub struct Solution;

impl Solution {
 pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
  let u = num_rows as usize;
  let mut result = Vec::new();
  let mut prev = vec![];
  for i in 0..u {
   if i == 0 {
    result.push(vec![1]);
   } else {
    let mut cur_vec = vec![0; i + 1];
    cur_vec[0] = 1;
    cur_vec[i] = 1;
    for j in 1..i {
     cur_vec[j] = prev[j - 1] + prev[j];
    }
    let cur_prev = cur_vec.to_vec();
    result.push(cur_vec);
    prev = cur_prev;
   }
  }

  result
 }
}
