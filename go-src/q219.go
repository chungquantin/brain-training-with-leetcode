package leetcode

// 219. Contains Duplicate II

// TIME: O(n)
// LeetCode: Runtime: 146 ms - Memory: 17.7 MB
func containsNearbyDuplicate(nums []int, k int) bool {
	occurrence := make(map[int]int, 0)
	exist := make(map[int]bool, 0)
	for i, num := range nums {
		if exist[num] && (i-occurrence[num] <= k) {
			return true
		}
		occurrence[num] = i
		exist[num] = true
	}
	return false
}
