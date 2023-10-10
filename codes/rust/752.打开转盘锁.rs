/*
 * @lc app=leetcode.cn id=752 lang=rust
 *
 * [752] 打开转盘锁
 */

// @lc code=start
use std::collections::HashSet;
use std::collections::LinkedList;
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut deadends_set = std::collections::HashSet::new();
        for deadend in deadends {
            deadends_set.insert(deadend);
        }
        let mut visited: HashSet<String> = std::collections::HashSet::new();
        let mut step = 0;
        let mut q: LinkedList<String> = std::collections::LinkedList::new();
        q.push_back("0000".to_string());
        visited.insert("0000".to_string());
        while !q.is_empty() {
            let cur_q_size = q.len();
            for _ in 0..cur_q_size {
                let cur = q.pop_front().unwrap();
                if deadends_set.contains(&cur) {
                    continue;
                }
                if cur == target {
                    return step;
                }
                for i in 0..4 {
                    let up = Self::plus_one(cur.to_owned(), i);
                    if !visited.contains(&up) {
                        q.push_back(up.to_owned());
                        visited.insert(up.to_owned());
                    }
                    let down = Self::minus_one(cur.to_owned(), i);
                    if !visited.contains(&down) {
                        q.push_back(down.to_owned());
                        visited.insert(down.to_owned());
                    }
                }
            }
            step += 1;
        }
        -1
    }

    fn plus_one(s: String, i: usize) -> String {
        let mut ch = s.chars().collect::<Vec<char>>();
        if ch[i] == '9' {
            ch[i] = '0';
        } else {
            ch[i] = (ch[i] as u8 + 1) as char;
        }
        ch.iter().collect::<String>()
    }

    fn minus_one(s: String, i: usize) -> String {
        let mut ch = s.chars().collect::<Vec<char>>();
        if ch[i] == '0' {
            ch[i] = '9';
        } else {
            ch[i] = (ch[i] as u8 - 1) as char;
        }
        ch.iter().collect::<String>()
    }
}
// @lc code=end
