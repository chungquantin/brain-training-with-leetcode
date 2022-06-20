struct MyQueue {
 in_s: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
 fn new() -> Self {
  MyQueue { in_s: vec![] }
 }

 fn push(&mut self, x: i32) {
  self.in_s.push(x);
 }
 fn pop(&mut self) -> i32 {
  let mut out_s = vec![];
  while !self.empty() {
   out_s.push(self.in_s.pop().unwrap());
  }
  let pop_item = out_s.pop().unwrap();
  while out_s.len() != 0 {
   self.in_s.push(out_s.pop().unwrap());
  }
  pop_item
 }

 fn peek(&self) -> i32 {
  self.in_s[0]
 }

 fn empty(&self) -> bool {
  self.in_s.len() == 0
 }
}
