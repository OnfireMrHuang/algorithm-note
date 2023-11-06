
class Solution:
    def isValid(self, s: str) -> bool:
        left = []  # 使用栈结构，存储所有待匹配的左括号
        for c in s:
            if c == '(' or c == '{' or c == '[':
                left.append(c)  # 如果字符 c 是左括号，则将其加入左括号栈 left 中
            else:
                if left and self.leftOf(c) == left[-1]:  # 如果字符 c 是右括号，则比较它与最近一次加入栈 left 中的左括号是否匹配
                    left.pop()  # 如果匹配，则将最近的左括号出栈，否则返回 False
                else:
                    return False
        return not left  # 最后判断栈是否为空，如果是则说明所有的左括号都被匹配了，返回 True，否则返回 False

    def leftOf(self, c: str) -> str:
        if c == '}':
            return '{'
        elif c == ')':
            return '('
        else:
            return '['