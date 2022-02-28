package binary_tree

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type BinaryTree struct {
	root *TreeNode
}

func (tree *BinaryTree) Insert(value int) *BinaryTree {
	if tree.root == nil {
		tree.root = &TreeNode{
			Val:   value,
			Left:  nil,
			Right: nil,
		}
	} else {
		tree.root.Insert(value)
	}

	return tree
}

func (node *TreeNode) Insert(value int) {
	if node == nil {
		node.Val = value
	} else if value <= node.Val {
		if node.Left == nil {
			node.Left = &TreeNode{
				Val:   value,
				Left:  nil,
				Right: nil,
			}
		} else {
			node.Left.Insert(value)
		}
	} else {
		if node.Right == nil {
			node.Right = &TreeNode{
				Val:   value,
				Left:  nil,
				Right: nil,
			}
		} else {
			node.Right.Insert(value)
		}
	}
}
