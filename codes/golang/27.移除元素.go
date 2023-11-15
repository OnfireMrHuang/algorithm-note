

// 双指针法
func removeElement(nums []int, val int) int {
	var fast, slow int
	for fast < len(nums) {
		if nums[fast] != val {
			nums[slow] = nums[fast]
			slow++
		}
		fast++
	}
	return slow
}