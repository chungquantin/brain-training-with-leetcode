import TreeNode from "./ds/TreeNode";

function levelOrder(root: TreeNode | null): number[][] {
  const result: number[][] = [];
  if (root == null) return result;
  result[0] = [root.val];
  return traverseLevel(result, root, 1);
}

function traverseLevel(result: number[][], root: TreeNode, level: number) {
  let { left, right } = root;
  if (left !== null || right !== null || !result[level]) {
    result[level] = [];
  }
  if (left !== null) {
    result[level].push(left.val);
    traverseLevel(result, left, level + 1);
  }
  if (right !== null) {
    result[level].push(right.val);
    traverseLevel(result, right, level + 1);
  }
  return result;
}
