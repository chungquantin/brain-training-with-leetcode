pub struct Solution;

impl Solution {
 pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
  let (mut i1, mut s1, mut i2, mut s2) = (0, false, 0, false);

  if m == 0 {
   nums1.append(nums2);
  } else if n != 0 {
   let (mut p1, mut p2) = (nums1[i1], nums2[i2]);
   let mut f = vec![];
   loop {
    let (um, un) = (m as usize, n as usize);
    if s1 && s2 {
     nums1.clear();
     nums1.append(&mut f);
     break;
    }
    if p1 < p2 && !s1 || s2 {
     f.push(p1);
     i1 += 1;
     if i1 < um {
      p1 = nums1[i1];
     } else {
      s1 = true;
     }
    } else if !s2 || s1 {
     f.push(p2);
     i2 += 1;
     if i2 < un {
      p2 = nums2[i2];
     } else {
      s2 = true;
     }
    }
   }
  }

  return nums1.to_vec();
 }
}
