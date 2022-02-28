package main

import (
	"fmt"
	leetcode "go-algo/go-src"
	linkedlist "go-algo/go-src/ds/linked_list"
	"go-algo/go-src/ds/queue"
	"go-algo/go-src/ds/stack"
)

func main() {
	fmt.Println("Golang Algorithms and Data Structure")
	fmt.Println("----- Data Structure Test Suite -----")

	queue.RunDsQueueTest(false)

	stack.RunDsStackTest(false)

	linkedlist.RunDsLinkedListTest(false)

	// Leetcode testing
	leetcode.Run21Test(false)

	leetcode.Run206Test(false)

	leetcode.Run234Test(false)

	leetcode.Run704Test(true)
}

func mockMain() {
	// Following book: Cracking the coding interview

	// Amortized array (ArrayList)

	// When the size of the array is reach, it automatically expand with the size of the array

	// Cost to appends m elements is O(m). However, it is O(2m). Because:

	// m + m/2 + m/4 + m/8 + ... + 1 would be 2m not m
}
