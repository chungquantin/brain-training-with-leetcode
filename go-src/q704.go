package leetcode

import (
	"fmt"
)

// 704. Binary Search
// Leetcode: Runtime: 31ms - Memory: 7.5MB
func BinarySearch(nums []int, target int) int {
	s, e := len(nums), 0
	if nums[0] == target {
		return 0
	}
	for s != e {
		m := int((s + e) / 2)
		if m == e || m == s {
			break
		}
		if nums[m] > target {
			s = m
		} else if nums[m] < target {
			e = m
		} else {
			return m
		}
	}
	return -1
}

func Run704Test(isRunning bool) {
	if isRunning {
		fmt.Println("Q704. Binary Search")
		fmt.Println("Result: ", BinarySearch([]int{-1, 0, 3, 5, 9, 12, 16}, 12))
		fmt.Println("Result: ", BinarySearch([]int{-3, -2, -1, 0}, -4))
	}
}
