/*
 * @lc app=leetcode.cn id=126 lang=rust
 *
 * [126] 单词接龙 II
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        // step1: 对单词列表构建为哈希集合
        let mut word_set: HashSet<String> = HashSet::new();
        for word in word_list {
            word_set.insert(word);
        }
        Self::bfs(begin_word, end_word, &word_set, &mut result);
        result
    }

    fn bfs(
        begin_word: String,
        end_word: String,
        word_set: &HashSet<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        // 首先判断结束单词是否存在列表中，如果不在则直接返回
        if !word_set.contains(&end_word) {
            return;
        }

        let mut queue: Vec<String> = Vec::new(); // 声明一个队列，用于存储当前层的单词
        queue.push(begin_word.clone());
        let mut visited: HashSet<String> = HashSet::new(); // 声明一个哈希集合，用于存储已经访问过的单词
        visited.insert(begin_word.clone());
        let mut path: LinkedList<String> = LinkedList::new(); // 声明一个栈，用于存储路径中的单词并且用于回溯
        let mut path_node_childrens: HashMap<String, usize> = HashMap::new(); // 声明一个哈希表，用于存储每个节点的子节点
        let mut node_parent_map: HashMap<String, String> = HashMap::new(); // 声明一个哈希表，用于存储每个节点的父节点
        let mut min_path_len = 0; // 最小路径的长度

        while let Some(cur_word) = queue.pop() {
            // 将访问的当前单词加入到已访问的单词列表中
            path.push_back(cur_word.clone());

            // 反向找到弹出节点的父节点，减少其父单词的子单词个数
            if let Some(parent) = node_parent_map.get(&cur_word) {
                if let Some(count) = path_node_childrens.get_mut(parent) {
                    if *count > 0 {
                        *count -= 1;
                    }
                }
            }

            let mut valid_neighbors: Vec<String> = Vec::new(); // 存储有效的邻居单词，作为下一层会访问的单词
            let neighbors = Self::get_neighbors(cur_word.clone(), word_set);
            let mut is_continue_find = true; // 是否继续查找下一层
            for neighbor_word in neighbors {
                //  判断邻居单词是否在往上的层次中访问过，如果访问过则没有必要继续访问，选择跳过
                if visited.contains(&neighbor_word) {
                    continue;
                }
                // 判断邻居单词的层级是不是超过了最小路径长度，如果超过了则也不用继续了
                if path.len() + 1 > min_path_len && min_path_len != 0 {
                    is_continue_find = false;
                    break;
                }

                // 邻居单词中找到了结束单词
                if neighbor_word.eq(&end_word) {
                    is_continue_find = false; // 不需要继续查找下一层
                    path.push_back(neighbor_word.clone());
                    if min_path_len == 0 || path.len() <= min_path_len {
                        if path.len() < min_path_len {
                            result.clear();
                        }
                        min_path_len = path.len();
                        let mut list: Vec<String> = Vec::new();
                        for word in path.iter() {
                            list.push(word.clone());
                        }
                        result.push(list);
                    }
                    path.pop_back(); // 回溯路径到上一层单词处
                    break;
                }
                valid_neighbors.push(neighbor_word.clone());
            }
            // 如果没有被阻止，那么就继续查找下一层
            let neighbors_len = valid_neighbors.len();
            if is_continue_find && neighbors_len > 0 {
                for neighbor in valid_neighbors.iter() {
                    node_parent_map.insert(neighbor.clone(), cur_word.clone());
                    visited.insert(neighbor.clone());
                }
                queue.append(&mut valid_neighbors);
                path_node_childrens.insert(cur_word.clone(), neighbors_len);
            } else {
                Self::recursive_back_path(&mut path, &path_node_childrens, &mut visited);
            }
        }
    }

    fn get_back_path(
        parent_word_map: &HashMap<String, String>,
        root_word: &String,
        word: &String,
    ) -> (Vec<String>, i32) {
        let mut list: Vec<String> = Vec::new();
        let mut len = 0;
        let mut tmp_word = word.clone();
        list.push(tmp_word.clone());
        len += 1;
        loop {
            let parent_word = parent_word_map.get(&tmp_word);
            if parent_word.is_none() {
                list = vec![];
                len = 0;
                break;
            }
            let p_word = parent_word.unwrap();
            list.push(p_word.clone());
            len += 1;
            if p_word.eq(root_word) {
                break;
            }
            tmp_word = parent_word.unwrap().clone();
        }
        list.reverse(); // 调换一下方向
        (list, len)
    }

    fn get_neighbors(word: String, word_set: &HashSet<String>) -> Vec<String> {
        let mut neighbors: Vec<String> = Vec::new();
        let mut word_bytes = word.as_bytes().to_vec();
        for i in 0..word_bytes.len() {
            let old = word_bytes[i] as u8;
            for j in 0..26 {
                word_bytes[i] = (j + 97) as u8;
                let new_word = String::from_utf8(word_bytes.clone()).unwrap();
                if word_set.contains(&new_word) && new_word != word {
                    neighbors.push(new_word);
                }
            }
            word_bytes[i] = old
        }
        neighbors
    }

    fn recursive_back_path(
        path: &mut LinkedList<String>,
        path_node_childrens: &HashMap<String, usize>,
        visited: &mut HashSet<String>,
    ) {
        if path.len() == 0 {
            return;
        }
        let cur_word = path.pop_back().unwrap();
        let childrens = path_node_childrens.get(&cur_word);
        // 如果当前单词还有子节点，那么就不需要回溯了
        if let Some(childrens_count) = childrens {
            if *childrens_count > 0 {
                path.push_back(cur_word);
                return;
            }
        }
        visited.remove(&cur_word); // 从已访问的单词中移除
        Self::recursive_back_path(path, path_node_childrens, visited);
    }
}
// @lc code=end
