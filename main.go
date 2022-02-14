package main

import (
	"fmt"
	linkedlist "go-algo/go-src/ds/linked_list"
	"go-algo/go-src/ds/queue"
	"go-algo/go-src/ds/stack"
)

func main() {
	fmt.Println("Golang Algorithms and Data Structure")
	fmt.Println("----- Data Structure Test Suite -----")

	queue.RunDsQueueTest(false)

	stack.RunDsStackTest(false)

	linkedlist.RunDsLinkedListTest(true)
}
