/*
 * @lc app=leetcode.cn id=354 lang=golang
 *
 * [354] 俄罗斯套娃信封问题
 */

/* 算法思路
先对信封的宽度进行升序排序，然后对宽度相同的信封进行高度降序
1、先排序
2、在对高度一栏进行最长子序列计算
*/
// @lc code=start

import (
	"sort"
)

type Envelops [][]int

func (e Envelops) Len() int {
	return len(e)
}

func (e Envelops) Swap(i,j int)  {
	e[i],e[j] = e[j],e[i]
}

func (e Envelops) Less(i, j int) bool {
	if e[i][0] == e[j][0] {
		return e[i][1] > e[j][1]
	}
	return e[i][0] < e[j][0]
}

func lengthOfLIS(nums []int) int {
	dp := make([]int,len(nums))
	// dp数组全部初始化为1，因为每个元素至少包括它自身这么一个序列
	for i:=0; i < len(nums); i++ {
		dp[i] = 1
	}
	// 遍历数组
	for i := 0; i < len(nums); i++ {
		for j :=0; j < i; j++ {
			if nums[i] > nums[j] {
				dp[i] = max(dp[i],dp[j]+1)
			}
		}
	}
	var res  = 0
	for i:=0; i < len(dp); i++ {
		res = max(res,dp[i])
	}
	return res
}

func max(a,b int) int {
	if a > b {
		return a
	}
	return b
}

func maxEnvelopes(envelopes [][]int) int {
	// 排序
	n := len(envelopes)
	sort.Sort(Envelops(envelopes))
	// 计算最长子序列
	height := make([]int,n)
	for i:=0; i< n; i++ {
		height[i] = envelopes[i][1]
	}
	return lengthOfLIS(height)
}
// @lc code=end

