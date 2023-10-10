/*
 * @lc app=leetcode.cn id=231 lang=rust
 *
 * [231] 2 的幂
 */

// @lc code=start
impl Solution {
    /*
    该题目可以使用位运算来解决，i32是32位的，所以最多只有31个1，如果是2的幂，那么二进制表示中只有一个1，其余都是0
    */
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut count = 0;
        let mut n = n;
        while n > 0 {
            if n & 1 == 1 {
                count += 1;
            }
            n >>= 1;
        }
        count == 1
    }
}
// @lc code=end
