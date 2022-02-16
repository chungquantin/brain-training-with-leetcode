package search_algo

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
