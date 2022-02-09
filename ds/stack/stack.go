package main

import (
	"fmt"
	"sync"
)

type Stack struct {
	m         *sync.Mutex
	maxLength int32
	s         []interface{}
	pos       int32
	order     int32
}

// FILO: 0 - LIFO: 1
func NewStack(maxLength_ int32, order_ int32) *Stack {
	if (order_ != 0) && (order_ != 1) {
		panic("ERROR: Invalid stack order")
	}
	return &Stack{
		m:         new(sync.Mutex),
		maxLength: maxLength_,
		s:         make([]interface{}, 0),
		pos:       0,
		order:     order_,
	}
}

// Add items into the stack
// OVERFLOW: if the stack is full, panic overflow
// TIME: O(1)
func (stack *Stack) Push(v interface{}) {
	stack.m.Lock()
	defer stack.m.Unlock()
	if v == nil {
		panic("ERROR: Can't push nil into stack")
	}
	if stack.order == 0 {
		stack.s = append(stack.s, v)
	} else {
		tempStack := make([]interface{}, 0)
		tempStack = append(tempStack, v)
		stack.s = append(tempStack, stack.s...)
	}
	stack.pos++
}

// Remove items from the stack. The items are popped in reserved order
// UNDERFLOW: if the stack is empty, panic underflow
// TIME: O(1)
func (stack *Stack) Pop() interface{} {
	stack.m.Lock()
	defer stack.m.Unlock()
	if len(stack.s) == 0 {
		panic("ERROR: Stack underflow. Can't pop stack")
	}

	var rv interface{}
	if stack.order == 0 {
		rv = stack.s[stack.pos-1]
		stack.s = stack.s[:stack.pos-1]
	} else {
		rv = stack.s[0]
		stack.s = stack.s[1:]
	}
	stack.pos--
	return rv
}

// Returns the top element in stack
// TIME: O(1)
func (stack *Stack) Peek() interface{} {
	stack.m.Lock()
	defer stack.m.Unlock()
	return stack.s[int32(len(stack.s))-stack.pos]

}

// Returns true if the stack is empty
// TIME: O(1)
func (stack *Stack) isEmpty() bool {
	return len(stack.s) == 0
}

func main() {
	fmt.Println("FILO Stack")
	stack := NewStack(10, 0)
	stack.Push(1)
	fmt.Println(stack.s...)
	stack.Push("Hi")
	fmt.Println(stack.s...)
	stack.Push(2)
	fmt.Println(stack.s...)
	stack.Push(5)
	fmt.Println(stack.s...)

	poppedValue := stack.Pop()
	fmt.Println("Popped value: ", poppedValue)
	fmt.Println(stack.s...)

	topValue := stack.Peek()
	fmt.Println("Top value: ", topValue)

	fmt.Println("LIFO Stack")
	stack1 := NewStack(10, 1)
	stack1.Push(1)
	fmt.Println(stack1.s...)
	stack1.Push("Hi")
	fmt.Println(stack1.s...)
	stack1.Push(2)
	fmt.Println(stack1.s...)
	stack1.Push(5)
	fmt.Println(stack1.s...)

	poppedValue1 := stack1.Pop()
	fmt.Println("Popped value: ", poppedValue1)
	fmt.Println(stack1.s...)

	topValue1 := stack.Peek()
	fmt.Println("Top value: ", topValue1)
}
