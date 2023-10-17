/*
 * @lc app=leetcode.cn id=102 lang=golang
 *
 * [102] 二叉树的层次遍历
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

 // 使用bfs-广度优先
 // 算法解析： 从二叉树的根开始，
 func levelOrder(root *TreeNode) [][]int {
	if root == nil {
		return [][]int{}
	}
	queue := []*TreeNode{}
	curNum,nextLevelNum,tmp,res :=1,0,[]int{},[][]int{}
	queue = append(queue,root)

	for len(queue) != 0 {
		if curNum >0 {
			node := queue[0]
			if node.Left != nil {
				queue = append(queue,node.Left)
				nextLevelNum++
			}
			if node.Right != nil {
				queue = append(queue,node.Right)
				nextLevelNum++
			}
			tmp = append(tmp,node.Val)
			curNum--
			queue = queue[1:]
		}
		if curNum == 0 {
			curNum = nextLevelNum
			nextLevelNum = 0
			res = append(res,tmp)
			tmp = []int{}
		}
	}
	return res
}

 // 使用dfs-深度优先
func levelOrder1(root *TreeNode) [][]int {
	level := -1
	res := [][]int{}
	dfsLevel(root,level,&res)
	return res
}

func dfsLevel(node *TreeNode,level int,res *[][]int) {
	if node == nil {
		return
	}
	currLevel := level +1
	for len(*res) <= currLevel {
		*res = append(*res,[]int{})
	}
	(*res)[currLevel] = append((*res)[currLevel],node.Val)
	dfsLevel(node.Left,currLevel,res)
	dfsLevel(node.Right,currLevel,res)
}
// @lc code=end

