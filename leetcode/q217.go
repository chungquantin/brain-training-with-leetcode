package leetcode

// 217. Contains Duplicates

// TIME: O(n)
// LeetCode: Runtime: 91ms - Memory: 10.1MB
func containsDuplicate(nums []int) bool {
	numDuplicateCheck := make(map[int]bool, 0)
	for _, num := range nums {
		if numDuplicateCheck[num] {
			return true
		}
		numDuplicateCheck[num] = true
	}
	return false
}
