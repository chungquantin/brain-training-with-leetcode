package leetcode

import (
	"fmt"
	linkedlist "go-algo/go-src/ds/linked_list"
)

// 206. Reverse Linked List
// Leetcode: Runtime: 0ms - Memory: 2.6MB
func ReverseList(head *linkedlist.ListNode) *linkedlist.ListNode {
	var l, f1, f2 *linkedlist.ListNode
	f0 := head

	if f0 != nil && f0.Next != nil {
		for {
			l, f1 = head, f0.Next
			cond := f1.Next == nil
			if f2 = f1.Next; cond {
				f2 = nil
			}
			f1.Next = l
			if f0.Next = f2; cond {
				f0.Next = nil
			}
			if cond {
				head = f1
				break
			}
			head = f1
		}
	}
	return head
}

func Run206Test(isRunning bool) {
	if isRunning {
		ll := &linkedlist.ListNode{}
		ll.Init([]int32{1})
		ll.Info()
		fmt.Println("After reversing")
		ReverseList(ll).Info()
	}
}
