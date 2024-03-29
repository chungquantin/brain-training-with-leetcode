pub struct SegmentTree {
  pub start: usize,
  pub end: usize,
  pub sum: i32,
  pub left: Option<Box<SegmentTree>>,
  pub right: Option<Box<SegmentTree>>,
}

impl SegmentTree {
  // O(n)
  pub fn new(start: usize, end: usize, vals: &[i32]) -> Self {
    if start == end {
      return Self {
        start,
        end,
        sum: vals[start],
        left: None,
        right: None,
      };
    }
    let mid = start + (end - start) / 2;
    let left = Self::new(start, mid, vals);
    let right = Self::new(mid + 1, end, vals);
    let sum = left.sum + right.sum;
    Self {
      start,
      end,
      sum,
      left: Some(Box::new(left)),
      right: Some(Box::new(right)),
    }
  }

  // O(logn)
  pub fn update(&mut self, index: usize, val: i32) {
    // NOTE: If is leaf, update itself
    if self.start == self.end && self.end == index {
      self.sum = val;
      return;
    }
    // NOTE: If is not leaf, update left or right
    let mid = self.start + (self.end - self.start) / 2;
    if index <= mid {
      self.left.as_mut().unwrap().update(index, val);
    } else {
      self.right.as_mut().unwrap().update(index, val);
    }
    // NOTE: After update children, update self
    self.sum = self.left.as_ref().unwrap().sum + self.right.as_ref().unwrap().sum;
  }

  // O(logn)
  pub fn query(&self, start: usize, end: usize) -> i32 {
    // NOTE: Exact match
    if self.start == start && self.end == end {
      return self.sum;
    }
    let mid = self.start + (self.end - self.start) / 2;
    // NOTE: Range on the left
    if end <= mid {
      return self.left.as_ref().unwrap().query(start, end);
    // NOTE: Range on the right
    } else if start > mid {
      return self.right.as_ref().unwrap().query(start, end);
    // NOTE: Range on both sides
    } else {
      return self.left.as_ref().unwrap().query(start, mid)
        + self.right.as_ref().unwrap().query(mid + 1, end);
    }
  }
}

struct NumArray {
  tree: SegmentTree,
}

impl NumArray {
  fn new(nums: Vec<i32>) -> Self {
    let tree = SegmentTree::new(0, nums.len() - 1, &nums);
    Self { tree }
  }

  fn update(&mut self, index: i32, val: i32) {
    self.tree.update(index as usize, val);
  }

  fn sum_range(&self, left: i32, right: i32) -> i32 {
    self.tree.query(left as usize, right as usize)
  }
}
