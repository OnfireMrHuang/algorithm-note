/*
 * @lc app=leetcode.cn id=407 lang=rust
 *
 * [407] 接雨水 II
 */

// @lc code=start
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Cell {
    v: i32,
    x: i32,
    y: i32,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.v.cmp(&other.v)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.v.cmp(&other.v))
    }
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        if m <= 2 || n <= 2 {
            return 0;
        }
        // 保存访问路径
        let mut visit = vec![vec![false; n]; m];
        let mut pq = BinaryHeap::new();
        // 首先将最外层的高度加入到最小堆中, 找到最短的那截板
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == (m - 1) || j == 0 || j == (n - 1) {
                    pq.push(Reverse(Cell {
                        v: height_map[i][j],
                        x: i as i32,
                        y: j as i32,
                    }));
                    visit[i][j] = true;
                }
            }
        }
        // 接着从最外层的短板开始，向内层开始找更低的高度.
        let mut result = 0;
        let dirs: Vec<i32> = vec![-1, 0, 1, 0, -1];
        while !pq.is_empty() {
            let curr_cell = pq.pop().unwrap();
            // 开始从当前单元处，向四周探索
            for k in 0..4 {
                let nx = curr_cell.0.x + dirs[k];
                let ny = curr_cell.0.y + dirs[k + 1];
                // 边界判断: 出界或访问过就跳过
                if nx < 0
                    || nx >= m as i32
                    || ny < 0
                    || ny >= n as i32
                    || visit[nx as usize][ny as usize]
                {
                    continue;
                }
                // 范围内：开始处理
                if height_map[nx as usize][ny as usize] < curr_cell.0.v {
                    result += curr_cell.0.v - height_map[nx as usize][ny as usize];
                }
                visit[nx as usize][ny as usize] = true;
                pq.push(Reverse(Cell {
                    v: height_map[nx as usize][ny as usize].max(curr_cell.0.v),
                    x: nx,
                    y: ny,
                }))
            }
        }
        result
    }
}
// @lc code=end
