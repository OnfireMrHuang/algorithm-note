/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] x 的平方根
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let (mut l, mut r, mut ans) = (0, x, -1);
        while (l <= r) {
            let mid = l + (r - l) / 2;
            if ((mid as i64 * mid as i64) <= x as i64) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        ans
    }
}
// @lc code=end
