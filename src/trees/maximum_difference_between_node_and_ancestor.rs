use std::cell::RefCell;
use std::rc::Rc;

use super::TreeNode;
#[allow(dead_code)]
fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: i32, min: i32) -> i32 {
    match node {
        Some(node) => {
            let current_max = max.max(node.borrow().val);
            let current_min = min.min(node.borrow().val);
            let left = dfs(&node.borrow().left, current_max, current_min);
            let right = dfs(&node.borrow().right, current_max, current_min);
            left.max(right)
        }
        None => i32::abs(max - min),
    }
}
#[allow(dead_code)]
pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(ref node) => dfs(&root, node.borrow().val, node.borrow().val),
        None => 0,
    }
}
