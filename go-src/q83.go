package leetcode

import linkedlist "go-algo/go-src/ds/linked_list"

// 82. Remove Duplicates from Sorted List
// Leetcode: Runtime: 4ms - Memory: 3.1MB
func DeleteDuplicates(head *linkedlist.ListNode) *linkedlist.ListNode {
	if head == nil {
		return nil
	}
	head.Next = DeleteDuplicates(head.Next)
	if head.Next != nil && head.Val == head.Next.Val {
		return head.Next
	}
	return head
}
