/*
 * @lc app=leetcode.cn id=105 lang=golang
 *
 * [105] 从前序与中序遍历序列构造二叉树
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func buildTree(preorder []int, inorder []int) *TreeNode {

	inPos := make(map[int]int)
	for i := 0; i < len(inorder); i++ {
		inPos[inorder[i]] = i
	}
	return buildPreIn2TreeDFS(preorder, 0, len(preorder)-1, inorder, 0, len(inorder)-1, inPos)
}

func buildPreIn2TreeDFS(pre []int, preStart, preEnd int, in []int, inStart, inEnd int, inMap map[int]int) *TreeNode {

	if preStart > preEnd || inStart > inEnd {
		return nil
	}

	// 前序遍历
	root := &TreeNode{
		Val: pre[preStart],
	}

	inRoot := inMap[root.Val]
	numsLeft := inRoot - inStart

	root.Left = buildPreIn2TreeDFS(pre, preStart+1, preStart+numsLeft, in, inStart, inRoot-1, inMap)
	root.Right = buildPreIn2TreeDFS(pre, preStart+numsLeft+1, preEnd, in, inRoot+1, inEnd, inMap)

	return root
}

// @lc code=end

