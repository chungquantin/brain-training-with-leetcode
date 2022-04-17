package leetcode

import (
	"math"
)

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isBalanced(root *TreeNode) bool {
	if root == nil {
					return true
	}
	left, right := root.Left, root.Right
	left_level := traverse_level(left)
	right_level := traverse_level(right)
	b_index := math.Abs(left_level - right_level);
	return b_index <= 1 && isBalanced(left) && isBalanced(right)
}

func traverse_level(root *TreeNode) float64 {
	if root == nil {
					return 0
	}
	left, right := root.Left, root.Right
	left_level, right_level := 0.0,0.0
	if left != nil {
		left_level = traverse_level(left)
	}
	if right != nil {
		right_level = traverse_level(right)
	}
	return 1.0 + math.Max(float64(left_level),float64(right_level))
}