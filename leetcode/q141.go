package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

// Runtime: 12ms - Memory: 4.4MB
func hasCycle(head *ListNode) bool {
	if head == nil {
		return false
	}
	walker, runner := head, head
	for {
		if runner.Next == nil || runner.Next.Next == nil {
			break
		}
		walker, runner = walker.Next, runner.Next.Next
		if walker == runner {
			return true
		}
	}
	return false
}
