package golang

import "sort"

func merge(intervals [][]int) [][]int {
	res := [][]int{}
	// 按区间的 start 升序排列
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})

	res = append(res, intervals[0])
	for i := 1; i < len(intervals); i++ {
		curr := intervals[i]
		// res 中最后一个元素的引用
		last := res[len(res)-1]
		if curr[0] <= last[1] {
			last[1] = max(last[1], curr[1])
		} else {
			// 处理下一个待合并区间
			res = append(res, curr)
		}
	}
	return res
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}
