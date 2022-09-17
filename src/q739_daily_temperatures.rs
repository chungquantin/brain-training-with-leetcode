pub struct Solution;

impl Solution {
 // TC: O(M + N) - M: Stack length, N: given temperatures length
 // Using monotonic stack data structure
 pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
  let mut answer = vec![0; temperatures.len()];
  let mut mono_stack = Vec::<(i32, usize)>::new();
  for (ind, temperature) in temperatures.iter().enumerate() {
   let cur = &temperature;
   while !mono_stack.is_empty() {
    let (top, top_ind) = mono_stack.last().unwrap();
    if top < cur {
     answer[*top_ind] = (ind - top_ind) as i32;
     mono_stack.pop();
    } else {
     break;
    }
   }
   mono_stack.push((*temperature, ind));
  }

  answer
 }
}
