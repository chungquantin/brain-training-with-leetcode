pub fn find_min_in_segment(input: Vec<i32>, l: usize, r: usize) -> i32 {
 if input.len() == 1 {
  return input[0];
 }
 let slice = &input[l..r + 1];
 let len = slice.len();
 let m = len / 2;
 let (left_part, right_part) = (&slice[0..m], &slice[m..len]);
 std::cmp::min(
  find_min_in_segment(left_part.to_vec(), 0, left_part.len() - 1),
  find_min_in_segment(right_part.to_vec(), 0, right_part.len() - 1),
 )
}

#[test]
fn test() {
 let input = vec![9, 2, 6, 3, 8, 5, 7];
 assert_eq!(find_min_in_segment(input.to_vec(), 3, 6), 3);
 assert_eq!(find_min_in_segment(input.to_vec(), 0, 6), 2);
 assert_eq!(find_min_in_segment(input.to_vec(), 3, 5), 3);
 assert_eq!(find_min_in_segment(input.to_vec(), 1, 4), 2);
}
