

func removeDuplicates(nums []int) int {
	// 如果数组为空，直接返回 0
	if len(nums) == 0 {
		return 0
	}
	// 定义快慢指针，初始化都指向数组头部
	slow, fast := 0, 0
	// 快指针向后遍历数组，直到末尾
	for fast < len(nums) {
		// 如果两个指针指向的元素不相同
		if nums[fast] > nums[slow] {
			// 慢指针向后移动，并且将慢指针位置上的值设为快指针位置上的值
			slow++
			nums[slow] = nums[fast]
		}
		// 快指针继续向后移动
		fast++
	}
	// slow 指向数组的最后一个不重复元素的位置
	// 数组长度为索引 + 1
	return slow + 1
}