class Solution {
	// 存储 inorder 中值到索引的映射
	HashMap<Integer, Integer> valToIndex = new HashMap<>();

	public TreeNode buildTree(int[] preorder, int[] inorder) {
		for (int i = 0; i < inorder.length; i++) {
			valToIndex.put(inorder[i], i);
		}
		return build(preorder, 0, preorder.length - 1,
				inorder, 0, inorder.length - 1);
	}

	/*
	 * 定义：前序遍历数组为 preorder[preStart..preEnd]，
	 * 中序遍历数组为 inorder[inStart..inEnd]，
	 * 构造这个二叉树并返回该二叉树的根节点
	 */
	TreeNode build(int[] preorder, int preStart, int preEnd,
			int[] inorder, int inStart, int inEnd) {
		if (preStart > preEnd) {
			return null;
		}

		// root 节点对应的值就是前序遍历数组的第一个元素
		int rootVal = preorder[preStart];
		// rootVal 在中序遍历数组中的索引
		int index = valToIndex.get(rootVal);

		int leftSize = index - inStart;

		// 先构造出当前根节点
		TreeNode root = new TreeNode(rootVal);
		// 递归构造左右子树
		root.left = build(preorder, preStart + 1, preStart + leftSize,
				inorder, inStart, index - 1);

		root.right = build(preorder, preStart + leftSize + 1, preEnd,
				inorder, index + 1, inEnd);
		return root;
	}
}