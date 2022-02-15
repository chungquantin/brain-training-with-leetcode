package leetcode

import linked_list "go-algo/go-src/ds/linked_list"

func RemoveElements(head *linked_list.ListNode, val int32) *linked_list.ListNode {
	if head == nil {
		return nil
	}
	head.Next = RemoveElements(head.Next, val)
	if head.Val == val {
		return head.Next
	}
	return head
}
