/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */

// @lc code=start
impl Solution {
    pub fn merge(vals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = vals.clone();
        Self::sort(&mut intervals);
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut temp_vec: Vec<i32> = Vec::new();
        for interval in intervals {
            // 如果temp是个空区间，则更新为当前区间
            if temp_vec.len() == 0 {
                temp_vec = interval;
                continue;
            }
            // 如果当前区间开始点大于上一个区间的结束点，说明两个区间没有重叠；将上一个区间作为新区间
            if interval.get(0) > temp_vec.get(1) {
                result.push(temp_vec.clone());
                temp_vec = interval;
                continue;
            }
            // 当前区间的开始点小于上一个区间的结束点，说明两个区间出现重叠； 将两个区间进行合并
            if interval.get(0) <= temp_vec.get(1) && interval.get(1) > temp_vec.get(1) {
                temp_vec[1] = interval[1];
                continue;
            }
        }
        // 如果temp_vec最后还有区间，则放入到结果中
        if temp_vec.len() > 0 {
            result.push(temp_vec);
        }
        result
    }

    // // 先以区间的开始节点进行排序: 使用计数排序法(提示的104限制没有生效)
    // fn sort(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //     // 首先创建一个104的动态数组桶
    //     let mut c = [0; 104];
    //     let mut max_stard_int = 0;
    //     for interval in &intervals {
    //         c[interval[0] as usize] += 1;
    //         if interval[0] > max_stard_int {
    //             max_stard_int = interval[0];
    //         }
    //     }
    //     // 依次累加计数
    //     for i in 1..=max_stard_int {
    //         c[i as usize] += c[i as usize - 1];
    //     }
    //     let mut r = vec![vec![0, 0]; intervals.len()];
    //     for interval in intervals.iter().rev() {
    //         let index = c[interval[0] as usize] - 1;
    //         r[index][0] = interval[0];
    //         r[index][1] = interval[1];
    //         c[interval[0] as usize] -= 1;
    //     }
    //     r
    // }

    // 还是使用插入排序吧
    fn sort(intervals: &mut Vec<Vec<i32>>) {
        if intervals.len() < 1 {
            return;
        }
        for i in 1..intervals.len() {
            let value = intervals[i].clone();
            let mut index = 0;
            for j in (0..i).rev() {
                index = j;
                if value[0] < intervals[j][0] {
                    intervals[j + 1] = intervals[j].clone();
                } else {
                    index += 1;
                    break;
                }
            }
            intervals[index] = value;
        }
    }
}
// @lc code=end
