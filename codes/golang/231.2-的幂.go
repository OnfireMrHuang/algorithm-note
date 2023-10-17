/*
 * @lc app=leetcode.cn id=231 lang=golang
 *
 * [231] 2的幂
 */

// @lc code=start
func isPowerOfTwo(n int) bool {
	if n == 0 {
		return false
	}
	n &= n-1
	return n==0
}
// @lc code=end

