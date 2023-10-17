/*
 * @lc app=leetcode.cn id=50 lang=golang
 *
 * [50] Pow(x, n)
 */

// @lc code=start
func myPow(x float64, n int) float64 {
	if n == 0 {
		return 1.0
	}
	if n < 0 {
		x = 1 / x
		n = -n
	}
	var pow float64 = 1
	for n > 0 {
		// 是奇数就多乘以一个x
		if n % 2 != 0 {
			pow *= x
		}
		x *=x
		n /= 2
	}
	return pow
}
// @lc code=end

