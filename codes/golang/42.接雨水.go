

func trap(height []int) int {
	if len(height) == 0 {
		return 0
	}
	n := len(height)
	res := 0
	// 数组充当备忘录
	l_max := make([]int, n)
	r_max := make([]int, n)
	// 初始化 base case
	l_max[0] = height[0]
	r_max[n-1] = height[n-1]
	// 从左向右计算 l_max
	for i := 1; i < n; i++ {
		l_max[i] = max(height[i], l_max[i-1])
	}
	// 从右向左计算 r_max
	for i := n - 2; i >= 0; i-- {
		r_max[i] = max(height[i], r_max[i+1])
	}
	// 计算答案
	for i := 1; i < n-1; i++ {
		res += min(l_max[i], r_max[i]) - height[i]
	}
	return res
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}