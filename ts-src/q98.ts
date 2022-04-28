import TreeNode from "./ds/TreeNode";

function isValidBST(root: TreeNode | null): boolean {
  if (root == null) {
    return true;
  }

  console.log(inorderTraversalIterative(root));

  return true;
}

function inorderTraversalIterative(root: TreeNode | null): number[] {
  let val = [];
  if (root == null) {
    return val;
  }

  let stack = [];
  while (root !== null || stack.length !== 0) {
    while (root !== null) {
      root = root.left;
      stack.push(root);
    }
    root = stack.pop();
    val.push(root.val);
    root = root.right;
  }

  return val;
}
