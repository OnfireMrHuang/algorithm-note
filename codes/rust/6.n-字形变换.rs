/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut arr: Vec<Vec<u8>> = Vec::with_capacity(num_rows as usize);
        // 初始化
        for _ in 0..num_rows {
            arr.push(Vec::new())
        }
        let mut flag: i32 = -1;
        let mut i: i32 = 0;
        for c in s.bytes() {
            arr[i as usize].push(c);
            if i == 0 || i == num_rows - 1 {
                flag = -1 * flag;
            }
            i += flag;
            if i >= num_rows {
                i = num_rows - 1;
            }
            if i < 0 {
                i = 0;
            }
        }
        let mut result: Vec<u8> = Vec::new();
        // 最后遍历动态数组重组结果
        for arr_item in &mut arr {
            result.append(arr_item);
        }
        String::from_utf8(result).unwrap()
    }
}
// @lc code=end
