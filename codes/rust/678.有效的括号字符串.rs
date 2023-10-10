/*
 * @lc app=leetcode.cn id=678 lang=rust
 *
 * [678] 有效的括号字符串
 */

// @lc code=start
impl Solution {
    /*
    按照只有(和)的字符判断是否为合法字符串的思路，使用栈来解决，当是(时入栈，是)时弹出栈进行比对，
    如果对比不匹配则返回false,否则一直执行完，检查堆栈中是否还有剩余字符，没有才算合法。

    该题目多了一个*，*可以表示一个空字符串、(字符或)字符，于是‘(’和'*'不能放到一个栈，比如栈里面是(*的时候
    遇到)字符，这时候就不知道*是代表(还是空字符了，所以需要两个栈，一个栈存放(的位置，一个栈存放*的位置，
    当遇到)时，优先匹配(的栈，没有再匹配*的栈。这样匹配到最后，两个栈还有'('和'*'，这里是有先后顺序的，
    假设'('和'*'的数目相等，但是*在前面，这就不算合法，只有‘(’在前面才算合法，因为两个栈分别代表了'('
    和'*'，所以我们可以使用下标来入栈，这样就可以确定顺序了.
     */
    pub fn check_valid_string(s: String) -> bool {
        let mut left_stack = vec![];
        let mut star_stack = vec![];
        for (i, c) in s.chars().into_iter().enumerate() {
            match c {
                '(' => {
                    left_stack.push(i);
                }
                '*' => {
                    star_stack.push(i);
                }
                _ => {
                    // 否则就是)字符了
                    if left_stack.len() > 0 {
                        left_stack.pop();
                    } else if star_stack.len() > 0 {
                        star_stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }
        while !left_stack.is_empty() && !star_stack.is_empty() {
            if left_stack.pop().unwrap() > star_stack.pop().unwrap() {
                return false;
            }
        }
        return left_stack.len() == 0;
    }
}
// @lc code=end
