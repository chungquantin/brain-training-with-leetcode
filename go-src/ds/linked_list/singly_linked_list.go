package linkedlist

import "fmt"

type ListNode struct {
	Val  int32
	Next *ListNode
}

func (listNode *ListNode) Init(values []int32) {
	listNode.InsertHead(values[0])
	for _, value := range values[1:] {
		listNode.Insert(value)
	}
}

func (listNode *ListNode) InsertHead(value int32) {
	if listNode.Next != nil {
		panic("Head is added already")
	}
	listNode.Val = value
}

func (listNode *ListNode) Insert(value int32) {
	curNode := listNode

	for {
		if curNode.Next == nil {
			curNode.Next = &ListNode{
				Next: nil,
				Val:  value,
			}
			break
		}
		curNode = curNode.Next
	}
}

func (listNode *ListNode) Info() {
	curNode := listNode
	for {
		fmt.Printf("Linked list: %+v\n", curNode)
		if curNode.Next == nil {
			break
		}
		curNode = curNode.Next
	}
}

func RunDsLinkedListTest(isRunning bool) {
	if isRunning {
		linkedList := &ListNode{}

		linkedList.Init([]int32{1, 2, 3, 4, 5})
		linkedList.Info()
	}
}
