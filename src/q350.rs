pub struct Solution;

impl Solution {
 pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
  let (mut i1, l1, mut c1) = (0, nums1.len(), nums1.to_vec());
  let (mut i2, l2, mut c2) = (0, nums2.len(), nums2.to_vec());
  let mut f: Vec<i32> = Vec::new();
  c1.sort();
  c2.sort();
  loop {
   if i1 == l1 || i2 == l2 {
    break;
   }
   let (p1, p2) = (c1[i1], c2[i2]);
   if p1 > p2 {
    i2 += 1;
   } else if p1 < p2 {
    i1 += 1;
   } else {
    f.push(p1);
    i1 += 1;
    i2 += 1;
   }
  }
  return f;
 }
}
