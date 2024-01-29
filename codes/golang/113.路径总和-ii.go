/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func pathSum(root *TreeNode, targetSum int) [][]int {
	var res [][]int
	var path []int
	dfs(root, targetSum, &res, &path)
	return res
}

func dfs(root *TreeNode, targetSum int, res *[][]int, path *[]int) {
	if root == nil {
		return
	}
	*path = append(*path, root.Val)
	if root.Left == nil && root.Right == nil && root.Val == targetSum {
		tmp := make([]int, len(*path))
		copy(tmp, *path)
		*res = append(*res, tmp)
	}
	dfs(root.Left, targetSum-root.Val, res, path)
	dfs(root.Right, targetSum-root.Val, res, path)
	*path = (*path)[:len(*path)-1]
}