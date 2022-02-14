package leetcode

import "sort"

// 217. Contains Duplicates

// TIME: O(n)
// LeetCode: Runtime: 91ms - Memory: 10.1MB
func ContainsDuplicate(nums []int) bool {
	numDuplicateCheck := make(map[int]bool, 0)
	for _, num := range nums {
		if numDuplicateCheck[num] {
			return true
		}
		numDuplicateCheck[num] = true
	}
	return false
}

// TIME: O(n*logn)
// SPACE: O(1) - We don't need another ds to store the value (Set)
func ContainsDuplicateV2(nums []int) bool {
	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})
	for i := 0; i < len(nums); i++ {
		if i+1 < len(nums) && nums[i] == nums[i+1] {
			return true
		}
	}
	return false
}
