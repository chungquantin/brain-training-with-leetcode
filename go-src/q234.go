package leetcode

import (
	"fmt"
	linkedlist "go-algo/go-src/ds/linked_list"
	"go-algo/go-src/ds/stack"
)

// Using stack
// TIME: O(2n) - SPACE: O(n)
// Leetcode: Runtime: 274ms - Memory: 15MB

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func IsPalindrome(head *linkedlist.ListNode) bool {
	curHead, newHead := head, head
	s := stack.NewStack(int32(1<<31 - 1))
	for {
		s.Push(curHead.Val)
		if curHead.Next == nil {
			break
		}
		curHead = curHead.Next
	}
	for {
		p := s.Pop()
		if newHead.Next == nil {
			break
		}
		if p != newHead.Val {
			return false
		}
		newHead = newHead.Next
	}
	return true
}

func Run234Test(isRunning bool) {
	if isRunning {
		ll := &linkedlist.ListNode{}
		ll.Init([]int32{1})
		fmt.Println(IsPalindrome(ll))
	}
}
