package main

import (
	"fmt"
	"sync"
)

type Queue struct {
	m         *sync.Mutex
	maxLength int32
	s         []interface{}
	pos       int32
	order     int32
}

// FIFO
func NewQueue(maxLength_ int32) *Queue {
	return &Queue{
		m:         new(sync.Mutex),
		maxLength: maxLength_,
		s:         make([]interface{}, 0),
		pos:       0,
	}
}

// Add items into the stack
// OVERFLOW: if the stack is full, panic overflow
// TIME: O(1)
func (queue *Queue) Enqueue(v interface{}) {
	queue.m.Lock()
	defer queue.m.Unlock()
	if v == nil {
		panic("ERROR: Can't push nil into stack")
	}
	queue.s = append(queue.s, v)
	queue.pos++
}

// Remove items from the stack. The items are popped in reserved order
// UNDERFLOW: if the stack is empty, panic underflow
// TIME: O(1)
func (queue *Queue) Dequeue() interface{} {
	queue.m.Lock()
	defer queue.m.Unlock()
	if len(queue.s) == 0 {
		panic("ERROR: Queue underflow. Can't pop stack")
	}

	var rv interface{}
	rv = queue.s[queue.pos-1]
	queue.s = queue.s[:queue.pos-1]
	queue.pos--
	return rv
}

// Returns true if the stack is empty
// TIME: O(1)
func (queue *Queue) isEmpty() bool {
	return len(queue.s) == 0
}

func (queue *Queue) Front() interface{} {
	return nil
}

func (queue *Queue) Rear() interface{} {
	return nil
}

func main() {
	fmt.Println("FILO Stack")
	queue := NewQueue(10)
	queue.Enqueue(1)
	fmt.Println(queue.s...)
	queue.Enqueue("Hi")
	fmt.Println(queue.s...)
	queue.Enqueue(2)
	fmt.Println(queue.s...)

	dequeuedValue := queue.Dequeue()
	fmt.Println("Popped value: ", dequeuedValue)
	fmt.Println(queue.s...)
}
