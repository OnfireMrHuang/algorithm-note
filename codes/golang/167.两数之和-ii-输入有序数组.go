
func twoSum(numbers []int, target int) []int {
	result := []int{0, 0}
	i := 0
	j := len(numbers) - 1
	for i < j {
		if numbers[i]+numbers[j] == target {
			result[0] = i + 1
			result[1] = j + 1
			break
		}
		if numbers[i]+numbers[j] > target {
			j--
		}
		if numbers[i]+numbers[j] < target {
			i++
		}
	}
	return result
}