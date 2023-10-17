/*
 * @lc app=leetcode.cn id=46 lang=golang
 *
 * [46] 全排列
 */

import "fmt"

// @lc code=start
func permute(nums []int) [][]int {
	var res [][]int
	var trace []int
	backTrace(nums, &trace, &res)
	return res
}

func backTrace(nums []int, trace *[]int, res *[][]int) {

	if len(*trace) == len(nums) {
		temp := make([]int, len(*trace))
		copy(temp, *trace)
		*res = append(*res, temp)
		return
	}
	for i := 0; i < len(nums); i++ {
		if IsExistItem(nums[i], *trace) {
			continue
		}
		*trace = append(*trace, nums[i])
		backTrace(nums, trace, res)
		*trace = (*trace)[:len(*trace)-1]
	}
}

func IsExistItem(value int, array []int) bool {
	for _, v := range array {
		if v == value {
			return true
		}
	}
	return false
}

// @lc code=end

