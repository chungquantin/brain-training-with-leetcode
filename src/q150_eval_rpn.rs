pub struct Solution;

impl Solution {
 pub fn eval_rpn(tokens: Vec<String>) -> i32 {
  let notations = vec!["+", "-", "*", "/"];
  let mut stack = vec![];
  for token in tokens {
   if !notations.contains(&token.as_str()) {
    stack.push(token.parse::<i32>().unwrap());
   } else {
    let n = token.as_str();
    let right = stack.pop().unwrap();
    let left = stack.pop().unwrap();
    stack.push(Solution::gen_notation(n, left, right))
   }
  }
  return stack[stack.len() - 1];
 }

 fn gen_notation(notation: &str, left: i32, right: i32) -> i32 {
  if notation == "+" {
   return left + right;
  } else if notation == "-" {
   return left - right;
  } else if notation == "*" {
   return left * right;
  } else if notation == "/" {
   return left / right;
  }

  0
 }
}
