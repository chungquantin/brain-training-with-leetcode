import TreeNode from "./ds/TreeNode";

function preorderTraversalRecursion(root: TreeNode | null): number[] {
  // Left - root -right
  if (root == null) {
    return [];
  }

  return [
    root.val,
    ...preorderTraversalRecursion(root.left),
    ...preorderTraversalRecursion(root.right),
  ];
}

function preorderTraversalIterative(root: TreeNode | null): number[] {
  let val = [];
  if (root == null) {
    return val;
  }

  let stack = [];
  stack.push(root);
  while (root !== null || stack.length !== 0) {
    root = stack.pop();
    if (root !== null) {
      val.push(root.val);
      stack.push(root.left);
      stack.push(root.right);
    }
  }
  return val;
}
