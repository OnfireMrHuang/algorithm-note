

func subarraySum(nums []int, k int) int {
	ans := 0
	preSumCntMap := make(map[int]int) // 前缀和 -> 前缀和出现的次数
	preSum := 0                       // 上一个数的前缀和
	preSumCntMap[0] = 1               // 默认空数组的前缀和为0，出现次数为1
	for i := 0; i < len(nums); i++ {
		curSum := preSum + nums[i]
		target := curSum - k
		// 检查target是否存在映射中
		if _, ok := preSumCntMap[target]; ok {
			ans += preSumCntMap[target]
		}
		// 更新preSumCntMap
		if _, ok := preSumCntMap[curSum]; ok {
			preSumCntMap[curSum] += 1
		} else {
			preSumCntMap[curSum] = 1
		}
		preSum = curSum
	}
	return ans
}

