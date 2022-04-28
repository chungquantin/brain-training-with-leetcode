import TreeNode from "./ds/TreeNode";

function isValidBST(root: TreeNode | null): boolean {
  if (root == null) {
    return true;
  }

  const values = [
    ...inorderTraversal(root.left),
    root.val,
    ...inorderTraversal(root.right),
  ];

  let cur = null;
  for (let value of values) {
    if ((cur && value <= cur) || value < 0) {
      return false;
    }
    cur = value;
  }
  return true;
}

function inorderTraversal(root: TreeNode | null): number[] {
  // Left - root -right
  if (root == null) {
    return [];
  }

  return [
    ...inorderTraversal(root.left),
    root.val,
    ...inorderTraversal(root.right),
  ];
}
