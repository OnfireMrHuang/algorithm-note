/*
 * @lc app=leetcode.cn id=70 lang=golang
 *
 * [70] 爬楼梯
 */

// @lc code=start
func climbStairs(n int) int {

	fn := make([]int,n+1)
	fn[0] = 1
	fn[1] = 1

	for i:=2;i<=n;i++{
		fn[i] = fn[i-1] + fn[i-2]
	}
	return fn[n]
}
// @lc code=end

