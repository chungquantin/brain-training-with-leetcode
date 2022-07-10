pub struct Solution;

impl Solution {
 pub fn convert(s: String, num_rows: i32) -> String {
  let mut floors: Vec<String> = vec![String::from(""); num_rows as usize];

  if num_rows < 2 {
   return s.into();
  }

  let mut floor = 0;
  let mut down: bool = true;

  for c in s.chars() {
   floors[floor as usize].push(c);
   floor += if down { 1 } else { -1 };
   down = down == (floor > 0 && floor < num_rows - 1);
  }

  floors.concat()
 }
}
