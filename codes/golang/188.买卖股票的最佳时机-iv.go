/*
 * @lc app=leetcode.cn id=188 lang=golang
 *
 * [188] 买卖股票的最佳时机 IV
 */

// @lc code=start
func maxProfit(k int, prices []int) int {
	
	n,result := len(prices),0
	if k >= n/2{
		for i:=1;i<n;i++ {
			if prices[i] > prices[i-1] {
				result+=(prices[i]-prices[i-1])
			}
		}
	} else {
		hold,cash := make([]int,k+1),make([]int,k+1)
		for i:=0;i <= k;i++ {
			hold[i],cash[i] = -999,0
		}
		for _,price := range prices {
			for i:=1;i<=k;i++{
				hold[i] = max(hold[i],cash[i-1]-price)
				cash[i] = max(cash[i],hold[i]+price)
			}
		}
		result = cash[k]
	}
	return result
}

func max(x,y int) int {
	if x > y {
		return x
	}
	return y
}

func min(x,y int) int {
	if x < y {
		return x
	}
	return y
}
// @lc code=end

