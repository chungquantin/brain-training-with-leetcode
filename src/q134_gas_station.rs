pub struct Solution;

impl Solution {
 pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
  let mut n = gas.len();

  let (mut curr_tank, mut total_tank, mut starting_point) = (0, 0, 0);
  let mut i = 0;
  while i < n {
   let val = gas[i] - cost[i];
   total_tank += val;
   curr_tank += val;
   if curr_tank < 0 {
    curr_tank = 0;
    starting_point = i;
   }

   i += 1;
  }

  if total_tank > 0 {
   starting_point
  } else {
   -1
  }
 }
}
