/*
 * @lc app=leetcode.cn id=435 lang=rust
 *
 * [435] 无重叠区间
 */

// @lc code=start
impl Solution {
    /*
    和预定会议室类似的题目类似，这里是求出最小移除的区间数能让所有区间不重叠，那么反过来则是求最大不重叠的区间数(会议室预定时间)
    在会议室场景中，肯定是会议结束时间越早，我们能预定到的会议室越多。
    同理，我们对区间的右端点进行排序，那么第一个就是右端点最小的，并且此时右端点按照升序排序，每个区间按时间段划分是最紧靠的，
    此时我们只需要顺序遍历找出没有重叠的区间个数就是最大不重叠区间个数
     */
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        let mut intervals = intervals;
        let size = intervals.len();
        intervals.sort_by(|a, b| a[1].to_owned().cmp(&(b[1].to_owned())));
        let mut right = intervals[0][1];
        let mut ans = 1;
        for i in 1..size {
            if intervals[i][0] >= right {
                ans += 1;
                right = intervals[i][1];
            }
        }
        (size - ans) as i32
    }
}
// @lc code=end
