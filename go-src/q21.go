package leetcode

import (
	"fmt"
	linkedlist "go-algo/go-src/ds/linked_list"
)

type ListNode = linkedlist.ListNode

// Recursive
func RecursiveMergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	if list1 == nil {
		return list2
	}
	if list2 == nil {
		return list1
	}
	if list1.Val < list2.Val {
		list1.Next = RecursiveMergeTwoLists(list1.Next, list2)
		return list1
	} else {
		list2.Next = RecursiveMergeTwoLists(list1, list2.Next)
		return list2
	}
}

func IterativeMergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	dummy := &ListNode{}
	p := dummy
	for list1 != nil && list2 != nil {
		if list1.Val > list2.Val {
			p.Next = list2
			list2 = list2.Next
		} else {
			p.Next = list1
			list1 = list1.Next
		}
		p = p.Next
	}
	if list1 != nil {
		p.Next = list1
	} else {
		p.Next = list2
	}
	return dummy.Next
}

func RunRecursiveTest(list1 *ListNode, list2 *ListNode) {
	recur_merge_list := RecursiveMergeTwoLists(list1, list2)
	recur_merge_list.Info()
}

func RunIterativeTest(list1 *ListNode, list2 *ListNode) {
	iter_merge_list := RecursiveMergeTwoLists(list1, list2)
	iter_merge_list.Info()
}

func Run21Test(isRunning bool) {
	fmt.Println("Q21. Merge two linked list")
	if isRunning {
		list1, list2 := &ListNode{}, &ListNode{}
		list1.Init([]int32{1, 2, 3, 4, 5, 6})
		list2.Init([]int32{1, 3, 5, 6})
		// RunRecursiveTest(list1, list2)
		RunIterativeTest(list1, list2)
	}
}
