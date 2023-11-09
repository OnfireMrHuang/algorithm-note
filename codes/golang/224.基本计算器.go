
func calculate(s string) int {
	var helper func(s []rune) int
	helper = func(s []rune) int {
		stack := []int{}
		sign := '+'
		num := 0

		for len(s) > 0 {
			c := s[0]
			s = s[1:]
			if unicode.IsDigit(c) {
				num = 10*num + int(c-'0')
			}
			// 遇到左括号开始递归计算 num
			if c == '(' {
				num = helper(s)
			}

			if (!unicode.IsDigit(c) && c != ' ') || len(s) == 0 {
				if sign == '+' {
					stack = append(stack, num)
				} else if sign == '-' {
					stack = append(stack, -num)
				} else if sign == '*' {
					stack[len(stack)-1] = stack[len(stack)-1] * num
				} else if sign == '/' {
					stack[len(stack)-1] = int(stack[len(stack)-1] / float64(num))
				}
				num = 0
				sign = c
			}
			// 遇到右括号返回递归结果
			if c == ')' {
				break
			}
		}
		sum := 0
		for _, val := range stack {
			sum += val
		}
		return sum
	}

	return helper([]rune(s))
}
