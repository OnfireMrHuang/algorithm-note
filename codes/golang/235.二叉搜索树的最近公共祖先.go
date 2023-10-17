/*
 * @lc app=leetcode.cn id=235 lang=golang
 *
 * [235] 二叉搜索树的最近公共祖先
 */

// @lc code=start
/**
 * Definition for TreeNode.
 * type TreeNode struct {
 *     Val int
 *     Left *ListNode
 *     Right *ListNode
 * }
 */
 func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	if root == nil || root == p || root == q {
		return root
	}
	if p.Val > root.Val && q.Val > root.Val {
		return lowestCommonAncestor(root.Right,p,q)
	}
	if p.Val < root.Val && q.Val < root.Val  {
		return lowestCommonAncestor(root.Left,p,q)
	}
	return root
}
// @lc code=end

