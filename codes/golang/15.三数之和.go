/*
 * @lc app=leetcode.cn id=15 lang=golang
 *
 * [15] 三数之和
 */

// @lc code=start
import (
	"sort"
)

func threeSum(nums []int) [][]int {
	res := [][]int{}
	counter := map[int]int{}

	for _, value := range nums {
		counter[value]++
	}
	// 去重
	uniqNums := []int{}
	for key := range counter {
		uniqNums = append(uniqNums, key)
	}

	// 排序
	sort.Ints(uniqNums)

	for i := 0; i < len(uniqNums); i++ {
		if uniqNums[i]*3 == 0 && counter[uniqNums[i]] >= 3 {
			res = append(res, []int{uniqNums[i], uniqNums[i], uniqNums[i]})
		}
		for j := i + 1; j < len(uniqNums); j++ {
			if uniqNums[i]*2+uniqNums[j] == 0 && counter[uniqNums[i]] > 1 {
				res = append(res, []int{uniqNums[i], uniqNums[i], uniqNums[j]})
			}
			if uniqNums[j]*2+uniqNums[i] == 0 && counter[uniqNums[j]] > 1 {
				res = append(res, []int{uniqNums[i], uniqNums[j], uniqNums[j]})
			}
			c := 0 - uniqNums[i] - uniqNums[j]
			if c > uniqNums[j] && counter[c] > 0 {
				res = append(res, []int{uniqNums[i], uniqNums[j], c})
			}
		}
	}
	return res
}

// @lc code=end



