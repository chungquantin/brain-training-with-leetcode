/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

import TreeNode from "./ds/TreeNode";

type OptionalTreeNode = TreeNode | null;

function lowestCommonAncestor(
  root: OptionalTreeNode,
  p: OptionalTreeNode,
  q: OptionalTreeNode
): OptionalTreeNode {
  if (!root || root == q || root == p) {
    return root;
  }
  let left = lowestCommonAncestor(root.left, p, q);
  let right = lowestCommonAncestor(root.right, p, q);
  if (left == null) {
    return right;
  } else if (right == null) {
    return left;
  } else {
    return root;
  }
}
