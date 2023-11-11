
func maxSlidingWindow(nums []int, k int) []int {
	length := len(nums)
	result := make([]int, length-k+1)
	deque := make([]int, 0)
	for i := 0; i < length; i++ {
		if i < k-1 {
			for len(deque) > 0 && deque[len(deque)-1] < nums[i] {
				deque = deque[:len(deque)-1]
			}
			deque = append(deque, nums[i])
			continue
		}
		for len(deque) > 0 && deque[len(deque)-1] < nums[i] {
			deque = deque[:len(deque)-1]
		}
		deque = append(deque, nums[i])
		result[i-k+1] = deque[0]
		if deque[0] == nums[i-k+1] {
			deque = deque[1:]
		}
	}
	return result
}