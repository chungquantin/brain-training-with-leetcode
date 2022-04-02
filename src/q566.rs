pub struct Solution;

impl Solution {
 pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
  let mut result = vec![];
  let mut new_vec = vec![];
  let mut new_vec_len = 0;
  for m in mat.iter() {
   for item in m.iter() {
    new_vec.push(*item);
    new_vec_len += 1;
    if new_vec_len == c as usize {
     result.push(new_vec.to_vec());
     new_vec = vec![];
     new_vec_len = 0;
    }
   }
  }

  println!("{:?} {:?}", c, result);

  if result.len() == r as usize && result[0].len() == c as usize {
   result
  } else {
   mat
  }
 }
}
