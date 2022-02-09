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
}

// LIFO: 1
func NewStack(maxLength_ int32) *Stack {
	return &Stack{
		m:         new(sync.Mutex),
		maxLength: maxLength_,
		s:         make([]interface{}, 0),
		pos:       0,
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
	tempStack := make([]interface{}, 0)
	tempStack = append(tempStack, v)
	stack.s = append(tempStack, stack.s...)
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

	rv = stack.s[0]
	stack.s = stack.s[1:]
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
	fmt.Println("LIFO Stack")
	stack := NewStack(10)
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
}
