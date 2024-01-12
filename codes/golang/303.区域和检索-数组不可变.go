

type NumArray struct {
	preSumMap map[int]int
}

func Constructor(nums []int) NumArray {
	preSumMap := make(map[int]int)
	preSum := 0
	for i := 0; i < len(nums); i++ {
		preSum += nums[i]
		preSumMap[i] = preSum
	}
	return NumArray{preSumMap: preSumMap}
}

func (this *NumArray) SumRange(left int, right int) int {
	return this.preSumMap[right] - this.preSumMap[left-1]
}

/**
 * Your NumArray object will be instantiated and called as such:
 * obj := Constructor(nums);
 * param_1 := obj.SumRange(left,right);
 */
