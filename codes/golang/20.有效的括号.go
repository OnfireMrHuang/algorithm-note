
// 包名为 main
// 定义一个字节数组栈类型
type Stack []byte

// 入栈操作
func (s *Stack) push(str byte) {
	*s = append(*s, str)
}

// 出栈操作
func (s *Stack) pop() byte {
	if len(*s) == 0 {
		return 0
	}
	res := (*s)[len(*s)-1]
	*s = (*s)[:len(*s)-1]
	return res
}

// 判断给定字符串是否是合法的括号序列
func isValid(str string) bool {
	// 定义一个栈 left 保存左括号
	var left Stack
	// 遍历字符
	for i := range str {
		c := str[i]
		// 当 c 是左括号时，入栈 left
		if c == '(' || c == '[' || c == '{' {
			left.push(c)
		} else { // 当 c 是右括号时
			// 如果栈 left 非空，且栈顶的左括号和当前右括号匹配，则弹出栈顶元素
			if len(left) != 0 && leftOf(c) == left.pop() {
				continue
			} else { // 当前左括号和最近的左括号不匹配
				return false
			}
		}
	}
	// 是否所有的左括号都被匹配了
	return len(left) == 0
}

// 返回左括号
func leftOf(c byte) byte {
	if c == '}' {
		return '{'
	} else if c == ')' {
		return '('
	} else {
		return '['
	}
}