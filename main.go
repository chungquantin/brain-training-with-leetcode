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
	leetcode.Run21Test(true)

	leetcode.Run206Test(false)

	leetcode.Run234Test(false)

}
