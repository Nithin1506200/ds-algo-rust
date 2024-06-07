use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

// https://leetcode.com/problems/same-tree/
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p), Some(q)) => {
            let p_ = p.borrow(); // think... q.clone().borrow() is not needed
            let q_ = q.borrow();
            if p_.val == q_.val {
                is_same_tree(p_.right.clone(), q_.right.clone())
                    && is_same_tree(p_.left.clone(), q_.left.clone())
            } else {
                false
            }
        }
        (None, None) => true,
        (_, _) => false,
    }
}
#[cfg(test)]
mod test {
    use crate::trees;

    use super::*;
    #[test]
    fn test() {
        // assert_eq!(is_same_tree(tree![1, 2, 3], tree![1, 2, 3]), true);
    }
}
