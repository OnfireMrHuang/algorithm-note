/*
 * @lc app=leetcode.cn id=121 lang=golang
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
func maxProfit(prices []int) int {
	if len(prices) <= 0 {
		return 0
	}
	res := 0
	min := prices[0]
	for i:=1;i<len(prices);i++ {
		if prices[i] < min {
			min = prices[i]
		}
		if prices[i] - min > res {
			res = prices[i] - min
		} 
	}
	return res
}
// @lc code=end

