pub struct Solution;

impl Solution {
 pub fn interval_intersection(
  first_list: Vec<Vec<i32>>,
  second_list: Vec<Vec<i32>>,
 ) -> Vec<Vec<i32>> {
  let mut intersect = vec![];
  let (start, end) = (0, 1);
  let (mut p1, mut p2) = (0, 0);
  let (s1, s2) = (first_list.iter().len(), second_list.iter().len());

  while p1 < s1 && p2 < s2 {
   let mut res = vec![0; 2];
   let (c1, c2) = (&first_list[p1], &second_list[p2]);
   res[start] = std::cmp::max(c1[start], c2[start]);
   res[end] = std::cmp::min(c1[end], c2[end]);
   if res[end] >= res[start] {
    intersect.push(res.to_vec());
   }
   if res[end] == c1[end] {
    p1 += 1;
   } else {
    p2 += 1;
   }
  }

  intersect
 }
}
