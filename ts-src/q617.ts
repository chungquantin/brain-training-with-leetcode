import TreeNode from "./ds/TreeNode";

function mergeTrees(
  root1: TreeNode | null,
  root2: TreeNode | null
): TreeNode | null {
  if (root1 === null && root2 === null) {
    return null;
  }

  const root = new TreeNode();
  root.val = (root1?.val || 0) + (root2?.val || 0);
  if (root1?.left || root2?.left) {
   root.left = mergeTrees(root1?.left, root2?.left);
  }
  if (root1?.right || root2?.right) {
   root.right = mergeTrees(root1?.right, root2?.right);
  }

  return root;
}
