use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SortAlgorithm {
 pub inputs: Vec<Vec<i32>>,
 pub sort: i32, // 1 - ACS, -1 - DCS
}

#[derive(Debug)]
pub enum SortAlgorithmName {
 BubbleSort,
 InsertionSort,
 MergeSort,
}

impl SortAlgorithm {
 pub fn frame(self: &Self, name: SortAlgorithmName, output_shown: bool) {
  println!("=====> Sort algorithm: {:?}", name);
  let start = Instant::now();
  for (ind, input) in self.inputs.to_vec().iter().enumerate() {
   let nums = input.to_vec();
   let output = match name {
    SortAlgorithmName::BubbleSort => self.bubble_sort(nums),
    SortAlgorithmName::InsertionSort => self.insertion_sort(nums),
    SortAlgorithmName::MergeSort => self.merge_sort(nums),
   };
   let duration = start.elapsed();
   SortAlgorithm::print(output_shown, ind, duration, output);
  }
 }

 /// ## Bubble Sort - Time Complexity: O(N^2)
 /// - Average Time Complexity: O(N^2)
 /// - Space Complexity: O(1)
 /// - Worst case: O(N^2)
 /// - O(N) if nearly sorted
 /// Bubble sort has many of the same properties as insertion sort, but has slightly higher overhead. In the case of nearly sorted data, bubble sort takes O(n) time, but requires at least 2 passes through the data (whereas insertion sort requires something more like 1 pass).
 pub fn bubble_sort(self: &Self, input: Vec<i32>) -> Vec<i32> {
  let mut nums = input.to_vec();
  for i in 0..nums.len() {
   for j in i..nums.len() {
    let cond = if self.sort == 1 {
     nums[i] > nums[j]
    } else {
     nums[i] < nums[j]
    };
    if cond {
     nums.swap(i, j);
    }
   }
  }

  nums.to_vec()
 }

 /// ## Insertion Sort - Time Complexity: O(N^2)
 /// - Average Time Complexity: O(N^2)
 /// - Space ComplexityL O(1)
 /// - O(N) if nearly sorted
 /// Note: Better than Bubble sort because only visit the element
 /// once while bubble sort visit every element twice
 pub fn insertion_sort(self: &Self, input: Vec<i32>) -> Vec<i32> {
  let mut nums = input.to_vec();

  for i in 1..nums.len() {
   let cur = nums[i];
   let mut j = i - 1;
   while nums[j] > cur {
    nums.swap(j + 1, j);
    if j == 0 {
     break;
    }
    j -= 1;
   }
  }

  nums.to_vec()
 }

 /// ## Merge Sort (stable) - Time Complexity: O(N*logN)
 /// Mergesort is a divide and conquer algorithm that was invented by John von Neumann in 1945.
 /// - **Average Time Complexity: O(N*LogN)**
 /// LogN: Slashing the array into two halves: left and right
 /// N: Linearly scan through the array to reorder
 /// - Worst case scenario: `O(N*LogN)`
 /// - Best case scenario `O(n log n)`
 /// - Space Complexity: `O(N)` for two halves of each recursion
 pub fn merge_sort(self: &Self, input: Vec<i32>) -> Vec<i32> {
  let nums = &mut input.to_vec();
  SortAlgorithm::merge_sort_helper(nums);
  nums.to_vec()
 }

 fn merge_sort_helper(nums: &mut [i32]) {
  let mid = nums.len() / 2;
  if nums.len() > 1 {
   SortAlgorithm::merge_sort_helper(&mut nums[..mid]);
   SortAlgorithm::merge_sort_helper(&mut nums[mid..]);
   SortAlgorithm::merge(nums, mid);
  }
 }

 fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
  let left_half = arr[..mid].to_vec();
  let right_half = arr[mid..].to_vec();

  let mut l = 0;
  let mut r = 0;

  for v in arr {
   if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
    *v = left_half[l];
    l += 1;
   } else {
    *v = right_half[r];
    r += 1;
   }
  }
 }

 pub fn print(output_shown: bool, ind: usize, duration: std::time::Duration, nums: Vec<i32>) {
  if output_shown {
   println!("Input {} - Time: {:?} - Ouput: {:?}", ind, duration, nums);
  } else {
   println!("Input {} - Time: {:?}", ind, duration);
  }
 }
}
