package leetcode

import linkedlist "go-algo/go-src/ds/linked_list"

/**
 * Definition for singly-linked list.
 */
func isPalindrome(head *linkedlist.ListNode) bool {
	// Scan the linked list to get the length
	curHead, size := head, 0
	for {
		if curHead.Next == nil {
			break
		}
		curHead = curHead.Next
		size++
	}
	if size%2 == 1 {
		return false
	}
	pointer_front, pointer_mid := head, head
	s, pc := 0, 0
	// Palindrome Counter
	for i := 0; i < size; i++ {
		if s >= size/2 {
			if pointer_front == pointer_mid {
				pc += 1
			}
			pointer_front = pointer_front.Next
		}
		pointer_mid = pointer_mid.Next
		s++
	}
	return pc == size/2
}
