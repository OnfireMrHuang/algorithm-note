/*
 * @lc app=leetcode.cn id=704 lang=golang
 *
 * [704] 二分查找
 */

// @lc code=start
func search(nums []int, target int) int {
	left := 0
	right := len(nums) -1
	for left <= right {
		mid := (left + right) / 2
		if nums[mid] == target {
			return mid 
		} else if nums[mid] > target {
			right = mid -1 
		} else if nums[mid] < target {
			left = mid + 1
		}
	}
	return -1
}
// @lc code=end

