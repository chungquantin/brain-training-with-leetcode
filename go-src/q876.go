package leetcode

import linkedlist "go-algo/go-src/ds/linked_list"

// 876. Middle of the linked list
// Leetcode: Runtime: 0ms - Memory: 1/9MB

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func MiddleNode(head *linkedlist.ListNode) *linkedlist.ListNode {
	one_step_pointer := head
	double_step_pointer := head
	for {
		if double_step_pointer.Next == nil {
			break
		}
		if double_step_pointer.Next.Next == nil {
			one_step_pointer = one_step_pointer.Next
			break
		}
		one_step_pointer = one_step_pointer.Next
		double_step_pointer = double_step_pointer.Next.Next
	}
	return one_step_pointer
}
