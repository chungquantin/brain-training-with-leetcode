import TreeNode from "./ds/TreeNode";

function isValidBST(root: TreeNode | null): boolean {
  return isValidBSTHelper(root, null, 0);
}

function isValidBSTHelper(
  root: TreeNode | null,
  memoRootVal: number | null,
  align: number
) {
  if (!root) return true;
  let left_ok = false;
  let right_ok = false;
  let left = root.left;
  let right = root.right;
  let is_root = memoRootVal == null;

  if (
    left == null ||
    (left.val < root.val &&
      (align <= 0 || (align == 1 && left.val > memoRootVal)))
  ) {
    left_ok = isValidBSTHelper(
      left,
      is_root ? root.val : memoRootVal,
      is_root ? -1 : align
    );
  }

  if (
    right == null ||
    (right.val > root.val &&
      (align >= 0 || (align == -1 && right.val < memoRootVal)))
  ) {
    right_ok = isValidBSTHelper(
      right,
      is_root ? root.val : memoRootVal,
      is_root ? 1 : align
    );
  }

  return left_ok && right_ok;
}
