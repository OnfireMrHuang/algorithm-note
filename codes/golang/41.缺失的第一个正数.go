

func firstMissingPositive(nums []int) int {
	size := len(nums)
	for i := 0; i < size; i++ {
		for nums[i] > 0 && nums[i] <= size && nums[i] != nums[nums[i]-1] {
			index := nums[i] - 1
			nums[i], nums[index] = nums[index], nums[i]
		}
	}
	for i := 0; i < size; i++ {
		if nums[i] != i+1 {
			return i + 1
		}
	}

	return size + 1
}
