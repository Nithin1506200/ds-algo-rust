use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let left_max = dfs(&root.borrow().left);
                let right_max = dfs(&root.borrow().right);

                1 + left_max.max(right_max)
            }
            None => 0,
        }
    }
    dfs(&root)
}
