class Solution:
    def calculate(self, s: str) -> int:
        def helper(s: List) -> int:
            stack = []
            sign = '+'
            num = 0

            while len(s) > 0:
                c = s.popleft()
                if c.isdigit():
                    num = 10 * num + int(c)
                # 遇到左括号开始递归计算 num
                if c == '(':
                    num = helper(s)

                if (not c.isdigit() and c != ' ') or len(s) == 0:
                    if sign == '+':
                        stack.append(num)
                    elif sign == '-':
                        stack.append(-num)
                    elif sign == '*':
                        stack[-1] = stack[-1] * num
                    elif sign == '/':
                        # python 除法向 0 取整的写法
                        stack[-1] = int(stack[-1] / float(num))       
                    num = 0
                    sign = c
                # 遇到右括号返回递归结果
                if c == ')': break
            return sum(stack)

        return helper(collections.deque(s))
