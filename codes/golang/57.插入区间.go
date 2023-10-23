

func insert(intervals [][]int, newInterval []int) [][]int {
	startInt := newInterval[0]
	endInt := newInterval[1]
	var result [][]int
	placed := false
	for _, interval := range intervals {
		if interval[0] > endInt {
			if !placed {
				result = append(result, []int{startInt, endInt})
				placed = true
			}
			result = append(result, interval)
		} else if interval[1] < startInt {
			result = append(result, interval)
		} else {
			startInt = min(startInt, interval[0])
			endInt = max(endInt, interval[1])
		}
	}
	if !placed {
		result = append(result, []int{startInt, endInt})
	}
	return result
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}
