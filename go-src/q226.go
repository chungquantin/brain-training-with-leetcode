package leetcode

import binary_tree "go-algo/go-src/ds/tree"

type TreeNode = binary_tree.TreeNode

// 226. Invert a binary tree
// Leetcode: Runtime: 0ms - Memory: 2.2MB
func InvertTree(root *TreeNode) *TreeNode {
	if root != nil {
		l, r := root.Left, root.Right
		root.Left, root.Right = r, l
		InvertTree(root.Left)
		InvertTree(root.Right)
	}
	return root
}
