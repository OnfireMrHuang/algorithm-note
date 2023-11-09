class Solution {
    public int calculate(String s) {
        Queue<Character> queueS = new LinkedList<Character>();
        for (char c : s.toCharArray()) {
            queueS.add(c);
        }

        return helper(queueS);
    }

    private int helper(Queue<Character> s) {
        Stack<Integer> stack = new Stack<Integer>();
        char sign = '+';
        int num = 0;

        while (!s.isEmpty()) {
            char c = s.poll();
            if (Character.isDigit(c)) {
                num = 10 * num + Character.getNumericValue(c);
            }
            // 遇到左括号开始递归计算 num
            if (c == '(') {
                num = helper(s);
            }

            if ((!Character.isDigit(c) && c != ' ') || s.isEmpty()) {
                if (sign == '+') {
                    stack.push(num);
                } else if (sign == '-') {
                    stack.push(-num);
                } else if (sign == '*') {
                    stack.push(stack.pop() * num);
                } else if (sign == '/') {
                    stack.push(stack.pop() / num);
                }
                num = 0;
                sign = c;
            }
            // 遇到右括号返回递归结果
            if (c == ')') {
                break;
            }
        }
        int res = 0;
        for (int i : stack) {
            res += i;
        }
        return res;
    }

}