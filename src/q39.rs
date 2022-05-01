pub struct Solution;

impl Solution {
 pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  let mut result: Vec<Vec<i32>> = vec![];
  let mut dp_table = vec![vec![]; target as usize];
  let mut mut_nums = nums;
  mut_nums.sort();

  for i in 0..target {
   for num in mut_nums.iter() {
    if i + 1 - num >= 0 {
     dp_table[i as usize].push(i + 1 - num);
    }
   }
  }

  let mut i = 0;
  for row in dp_table.to_vec() {
   println!("{:?} - {:?}", i + 1, row);
   i += 1;
  }

  let mut stack = vec![target];
  while let Some(t) = stack.pop() {
   println!("{:?}", stack);
   let target_row = dp_table[t as usize - 1].to_vec();
   let mut i = target_row.len() as i32 - 1;
   while i >= 0 {
    let cur = target_row[i as usize];
    println!("{}", cur);
    if cur == 0 {
     result.push(vec![mut_nums[i as usize]]);
    } else {
     stack.push(cur);
    }
    i -= 1;
   }
  }

  result
 }
}
