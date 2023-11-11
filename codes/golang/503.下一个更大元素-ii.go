
func nextGreaterElements(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	for i := 0; i < n; i++ {
		res[i] = -1
	}
	stack := make([]int, 0)
	for i := 0; i < n*2; i++ {
		for len(stack) > 0 && nums[stack[len(stack)-1]] < nums[i%n] {
			lastIdx := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			res[lastIdx] = nums[i%n]
		}
		stack = append(stack, i%n)
	}
	return res
}