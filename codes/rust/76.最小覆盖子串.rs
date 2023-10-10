/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut need_map: HashMap<char, i32> = HashMap::new();
        let mut window_map: HashMap<char, i32> = HashMap::new();
        let mut min_window_size = s.len();
        let mut min_left: i32 = -1;
        let mut min_right: i32 = -1;
        let mut valid_num = 0;
        let (mut left, mut right) = (0, 0);
        // 初始化需要覆盖字符的信息
        for c in t.chars() {
            *need_map.entry(c).or_insert(0) += 1;
        }
        let s_chars = s.chars().collect::<Vec<char>>();
        // 移动右游标扩大窗口
        while right < s_chars.len() {
            let ch = s_chars[right];
            right += 1;

            *window_map.entry(ch).or_insert(0) += 1;
            if need_map.contains_key(&ch) && window_map[&ch] == need_map[&ch] {
                valid_num += 1;
            }

            // 还未覆盖，继续扩大右窗口
            if valid_num < need_map.len() {
                continue;
            }

            // 更新最小的覆盖子串长度大小和左、右游标
            if right - left <= min_window_size {
                min_window_size = right - left;
                min_left = left as i32;
                min_right = right as i32;
            }

            // 已经覆盖了，开始移动左游标缩短窗口
            while left < right && valid_num == need_map.len() {
                let ch = s_chars[left];
                left += 1;
                if need_map.contains_key(&ch) && window_map[&ch] == need_map[&ch] {
                    valid_num -= 1;
                }
                if window_map.contains_key(&ch) {
                    *window_map.get_mut(&ch).unwrap() -= 1;
                }

                // 继续判断是否完全覆盖
                if valid_num == need_map.len() && right - left <= min_window_size {
                    min_window_size = right - left;
                    min_left = left as i32;
                    min_right = right as i32;
                }
            }
        }
        if min_left != -1 && min_right != -1 {
            let result = s[min_left as usize..min_right as usize].to_string();
            return result;
        }
        "".to_string()
    }
}
// @lc code=end
