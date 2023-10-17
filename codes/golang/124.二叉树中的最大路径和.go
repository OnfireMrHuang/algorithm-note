/*
 * @lc app=leetcode.cn id=124 lang=golang
 *
 * [124] 二叉树中的最大路径和
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

import "math"

func maxPathSum(root *TreeNode) int {
	if root == nil {
		return 0
	}

	max := math.MinInt32
	getPathSum(root, &max)
	return max
}

func getPathSum(root *TreeNode, maxSum *int) int {
	if root == nil {
		return math.MinInt32
	}
	left := getPathSum(root.Left, maxSum)
	right := getPathSum(root.Right, maxSum)

	currMax := max(max(left+root.Val, right+root.Val), root.Val)
	*maxSum = max(*maxSum, max(currMax, left+right+root.Val))
	return currMax
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// @lc code=end

