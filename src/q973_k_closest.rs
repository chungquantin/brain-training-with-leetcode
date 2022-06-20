pub struct Solution;

impl Solution {
 pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
  let mut m = -1;
  let mut m_index = 0;
  let (x, y) = (0, 1);
  let mut k_points: Vec<Vec<i32>> = vec![];
  for (i, p) in points.iter().enumerate() {
   let p_dist = p[x].pow(2) + p[y].pow(2);
   if k_points.len() < k as usize {
    if p_dist > m {
     m = p_dist;
     m_index = i;
    }
    k_points.push(p.to_vec());
   } else if p_dist < m {
    k_points[m_index] = p.to_vec();
    let mut t = -1;
    let mut t_index = 0;
    for (j, k) in k_points.iter().enumerate() {
     let k_dist = k[x].pow(2) + k[y].pow(2);
     if k_dist > t {
      t = k_dist;
      t_index = j;
     }
    }
    m_index = t_index;
    m = t;
   }
  }
  k_points
 }
}
