/*
 * @lc app=leetcode.cn id=341 lang=rust
 *
 * [341] 扁平化嵌套列表迭代器
 */

// @lc code=start
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }

impl NestedInteger {
    pub fn is_integer(&self) -> bool {
        match self {
            NestedInteger::Int(_) => true,
            NestedInteger::List(_) => false,
        }
    }

    pub fn get_integer(&self) -> i32 {
        match self {
            NestedInteger::Int(x) => *x,
            NestedInteger::List(_) => panic!("not an integer"),
        }
    }

    pub fn get_list(&self) -> &Vec<NestedInteger> {
        match self {
            NestedInteger::Int(_) => panic!("not a list"),
            NestedInteger::List(x) => x,
        }
    }
}
struct NestedIterator {
    nested_list: Vec<NestedInteger>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self { nested_list }
    }

    fn next(&mut self) -> i32 {
        let res = self.nested_list.get(0).unwrap().get_integer();
        self.nested_list.remove(0);
        res
    }

    // 因为在调用next之前一定会调用has_next，所以考虑在这里把下一个元素先展开预热
    fn has_next(&mut self) -> bool {
        loop {
            if self.nested_list.is_empty() {
                return false;
            }
            let first = self.nested_list.get(0).unwrap();
            // 如果是数值，则直接返回
            if first.is_integer() {
                return true;
            }
            // 否则是嵌套列表，则将嵌套列表展开为数值列表
            let nums = Self::flatten(first.get_list());
            self.nested_list.remove(0);
            if nums.is_empty() {
                continue;
            }
            for num in nums.iter().rev() {
                self.nested_list
                    .insert(0, NestedInteger::Int(num.to_owned()));
            }
        }
    }

    fn flatten(nested_list: &Vec<NestedInteger>) -> Vec<i32> {
        let mut res = vec![];
        for item in nested_list {
            if item.is_integer() {
                res.push(item.get_integer());
            } else {
                res.append(&mut Self::flatten(item.get_list()));
            }
        }
        res
    }
}


/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// @lc code=end

