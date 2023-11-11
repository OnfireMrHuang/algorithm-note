
func leastInterval(tasks []byte, n int) int {
	if len(tasks) == 0 {
		return 0
	}
	size := len(tasks)
	wordCnt := make([]int, 26)
	for _, task := range tasks {
		index := task - 'A'
		wordCnt[index] += 1
	}
	maxCnt := 1
	for _, cnt := range wordCnt {
		if cnt > maxCnt {
			maxCnt = cnt
		}
	}
	tot := 0
	for _, cnt := range wordCnt {
		if cnt == maxCnt {
			tot += 1
		}
	}
	return max(size, (maxCnt-1)*(n+1)+tot)
}