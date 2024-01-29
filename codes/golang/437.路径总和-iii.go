/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func pathSum(root *TreeNode, targetSum int) int {
	path := make([]int, 0)
	return dfs(root, targetSum, path)
}

func dfs(root *TreeNode, targetSum int, path []int) int {
	if root == nil {
		return 0
	}

	path = append(path, root.Val)
	var count, sum int
	for i := len(path) - 1; i >= 0; i-- {
		sum += path[i]
		if sum == targetSum {
			count++
		}
	}

	count += dfs(root.Left, targetSum, path)
	count += dfs(root.Right, targetSum, path)

	path = path[:len(path)-1]
	return count
}