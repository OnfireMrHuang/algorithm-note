

func merge(nums1 []int, m int, nums2 []int, n int) {
	i, j, p := m-1, n-1, len(nums1)-1 // 初始化指针
	for i >= 0 && j >= 0 {            // 两个数组都未遍历完时进行比较
		if nums1[i] > nums2[j] { // 挑选大的元素放入 nums1 的末位
			nums1[p] = nums1[i]
			i--
		} else {
			nums1[p] = nums2[j]
			j--
		}
		p-- // 从后往前生成结果
	}
	for j >= 0 { // nums2 剩余元素放入 nums1
		nums1[p] = nums2[j]
		j--
		p--
	}
}