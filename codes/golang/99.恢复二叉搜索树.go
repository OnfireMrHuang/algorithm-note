/*
 * @lc app=leetcode.cn id=99 lang=golang
 *
 * [99] 恢复二叉搜索树
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
func recoverTree(root *TreeNode) {
	// 首先中序遍历，得到一个有序数组
	nums := inorder(root)
	// 找到两个被交换的节点
	x, y := findTwoSwapped(nums)
	// 恢复二叉搜索树
	recover(root, 2, x, y)
}

func inorder(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}
	res := inorder(root.Left)
	res = append(res, root.Val)
	res = append(res, inorder(root.Right)...)
	return res
}

func findTwoSwapped(nums []int) (int, int) {
	x, y := -1, -1
	for i := 0; i < len(nums)-1; i++ {
		if nums[i+1] < nums[i] {
			y = i + 1
			if x == -1 {
				x = i
			} else {
				break
			}
		}
	}
	return nums[x], nums[y]
}

func recover(root *TreeNode, count int, x int, y int) {
	if root != nil {
		if root.Val == x || root.Val == y {
			root.Val = root.Val ^ x ^ y
			count--
			if count == 0 {
				return
			}
		}
		recover(root.Left, count, x, y)
		recover(root.Right, count, x, y)
	}
}

// @lc code=end

