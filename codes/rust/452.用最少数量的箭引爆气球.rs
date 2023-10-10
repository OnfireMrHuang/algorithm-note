/*
 * @lc app=leetcode.cn id=452 lang=rust
 *
 * [452] 用最少数量的箭引爆气球
 */

// @lc code=start
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut count = 0;
        let mut end = i64::MIN;
        for point in points {
            if point[0] as i64 > end {
                count += 1;
                end = point[1] as i64;
            }
        }
        count
    }
}
// @lc code=end
