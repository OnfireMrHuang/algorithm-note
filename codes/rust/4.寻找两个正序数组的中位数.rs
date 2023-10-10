/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(m: Vec<i32>, n: Vec<i32>) -> f64 {
        let total_len = m.len() + n.len();
        let mid_num: f64;
        if total_len % 2 != 0 {
            // 总长度为奇数
            let k = total_len / 2 + 1;
            mid_num = Self::get_kth_element(&m, &n, k) as f64;
        } else {
            // 总长度为偶数
            let k_left = total_len / 2;
            let mid_left_num = Self::get_kth_element(&m, &n, k_left) as f64;
            let k_right = total_len / 2 + 1;
            let mid_right_num = Self::get_kth_element(&m, &n, k_right) as f64;
            mid_num = (mid_left_num + mid_right_num) / 2.0;
        }
        mid_num
    }

    // 这里的k是第几位数的意思
    fn get_kth_element(m: &Vec<i32>, n: &Vec<i32>, mut k: usize) -> i32 {
        let mut m_index = 0;
        let mut n_index = 0;
        let k_num: i32;
        loop {
            // 如果m的索引等于了动态数组长度，那么说明m数组的数值都被pass了，这时候再取n数组的k值就好了
            if m_index == m.len() {
                k_num = n[n_index + k - 1];
                break;
            }
            if n_index == n.len() {
                k_num = m[m_index + k - 1];
                break;
            }
            if k <= 1 {
                k_num = Self::min(m[m_index], n[n_index]);
                break;
            }

            let half = k / 2 - 1;
            let new_m_index = Self::min((m_index + half) as i32, (m.len() - 1) as i32) as usize;
            let new_n_index = Self::min((n_index + half) as i32, (n.len() - 1) as i32) as usize;
            if m[new_m_index] <= n[new_n_index] {
                k = k - (new_m_index - m_index) - 1;
                m_index = new_m_index + 1;
            } else {
                k = k - (new_n_index - n_index) - 1;
                n_index = new_n_index + 1;
            }
        }
        k_num
    }

    fn min(x: i32, y: i32) -> i32 {
        if x < y {
            return x;
        }
        y
    }
}
// @lc code=end
