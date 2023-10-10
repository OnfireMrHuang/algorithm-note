/*
 * @lc app=leetcode.cn id=621 lang=rust
 *
 * [621] 任务调度器
 */

// @lc code=start
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if tasks.is_empty() {
            return 0;
        }
        let size = tasks.len();
        let mut word_cnt: Vec<usize> = vec![0; 26];
        for task in tasks {
            let index = task as usize - 'A' as usize;
            word_cnt[index] += 1;
        }
        let mut max_cnt = 1;
        // 求出其中的最大值
        for cnt in word_cnt.iter() {
            max_cnt = std::cmp::max(max_cnt, cnt.to_owned());
        }
        let mut tot = 0;
        // 求出最大值有多少个
        for cnt in word_cnt {
            if cnt == max_cnt {
                tot += 1;
            }
        }
        std::cmp::max(size as i32, (max_cnt as i32 - 1) * (n + 1) + tot)
    }
}
// @lc code=end
