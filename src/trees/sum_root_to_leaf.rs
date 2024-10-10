// https://leetcode.com/problems/sum-root-to-leaf-numbers/submissions/1415592952/

use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, path_sum: i32, sum: &mut i32) -> i32 {
        if let Some(root) = root {
            let left = helper(&root.borrow().left, root.borrow().val + path_sum * 10, sum);
            let right = helper(&root.borrow().right, root.borrow().val + path_sum * 10, sum);
            let sum_ = match (&root.borrow().left, &root.borrow().right) {
                (Some(_), Some(_)) => 2 * path_sum + left + right,
                (None, Some(_)) => path_sum + right,
                (Some(_), None) => path_sum + left,
                (None, None) => {
                    let sum_ = path_sum * 10 + root.borrow().val;
                    *sum = *sum + sum_;
                    sum_
                }
            };
            sum_
        } else {
            0
        }
    }
    let mut sum: i32 = 0;
    helper(&root, 0, &mut sum);
    sum
}
