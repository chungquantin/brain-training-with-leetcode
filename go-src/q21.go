package leetcode

import (
	linkedlist "go-algo/go-src/ds/linked_list"
)

type ListNode = linkedlist.ListNode

func MergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	if list1 == nil {
		return list2
	}
	if list2 == nil {
		return list1
	}
	if list1.Val < list2.Val {
		list1.Next = MergeTwoLists(list1.Next, list2)
		return list1
	} else {
		list2.Next = MergeTwoLists(list1, list2.Next)
		return list2
	}
}

func Run21Test(isRunning bool) {
	if isRunning {
		list1, list2 := &ListNode{}, &ListNode{}
		list1.Init([]int32{1, 2, 3, 4, 5, 6})
		list2.Init([]int32{1, 3, 5, 6})
		merge_list := MergeTwoLists(list1, list2)
		merge_list.Info()
	}
}
