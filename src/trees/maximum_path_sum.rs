use std::cell::RefCell;
use std::rc::Rc;
// https://leetcode.com/problems/binary-tree-maximum-path-sum/submissions/1286171348/
use super::TreeNode;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn findMax(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> i32 {
        if let Some(node) = node {
            let left_max = 0.max(findMax(&node.borrow().left, sum));
            let right_max = 0.max(findMax(&node.borrow().right, sum));
            *sum = (*sum).max(node.borrow().val + left_max + right_max);
            left_max.max(right_max) + node.borrow().val
        } else {
            0
        }
    }
    let mut sum = i32::MIN;
    findMax(&root, &mut sum);
    sum
}
