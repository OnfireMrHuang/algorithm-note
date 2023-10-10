/*
 * @lc app=leetcode.cn id=326 lang=rust
 *
 * [326] 3 的幂
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // int范围内的最大3次幂是多少，约为 1162261467
        // 如果n为3的幂的话，那么必然满足 n * 3k=1162261467
        // 因此，我们只需要判断 n 是否为 1162261467的约数即可。
        n > 0 && 1162261467 % n == 0
        // let mut n = n;
        // if n <= 0 {
        //     return false;
        // }
        // while n % 3 == 0 {
        //     n /= 3;
        // }
        // n == 1
    }
}
// @lc code=end
