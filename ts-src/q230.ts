import TreeNode from "./ds/TreeNode";

function kthSmallest(root: TreeNode | null, k: number): number {
  return inorderTraversal(root)[k - 1];
}

function inorderTraversal(root: TreeNode | null): number[] {
  // Left - root -right
  if (root == null) {
    return [];
  }

  return [
    root.val,
    ...inorderTraversal(root.left),
    ...inorderTraversal(root.right),
  ];
}
