

func moveZeroes(nums []int) {
	// 去除 nums 中的所有 0
	// 返回去除 0 之后的数组长度
	p := removeElement(nums, 0)
	// 将 p 之后的所有元素赋值为 0
	for ; p < len(nums); p++ {
		nums[p] = 0
	}
}

// 双指针技巧，复用 [27. 移除元素] 的解法。
func removeElement(nums []int, val int) int {
	fast := 0
	slow := 0
	for fast < len(nums) {
		if nums[fast] != val {
			nums[slow] = nums[fast]
			slow++
		}
		fast++
	}
	return slow
}