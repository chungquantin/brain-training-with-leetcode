package main

import (
	"fmt"
	"go-algo/ds/queue"
	"go-algo/ds/stack"
)

func main() {
	fmt.Println("Golang Algorithms and Data Structure")
	fmt.Println("----- Data Structure Test Suite -----")

	queue.RunDsQueueTest()

	stack.RunDsStackTest()
}
