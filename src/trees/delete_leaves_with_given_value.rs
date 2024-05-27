/**
 *
https://leetcode.com/problems/delete-leaves-with-a-given-value/description/?envType=daily-question&envId=2024-05-17

 Given a binary tree root and an integer target, delete all the leaf nodes with value target.

 Note that once you delete a leaf node with value target, if its parent node becomes a leaf node and has the value target, it should also be deleted (you need to continue doing that until you cannot).
 */
use std::cell::RefCell;
use std::rc::Rc;

use super::TreeNode;
#[allow(dead_code)]
pub fn remove_leaf_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(root) => {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            root.borrow_mut().left = remove_leaf_nodes(left, target);
            root.borrow_mut().right = remove_leaf_nodes(right, target);

            if root.borrow().right.is_none()
                && root.borrow().left.is_none()
                && root.borrow().val == target
            {
                None
            } else {
                Some(root)
            }
        }
        None => None,
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn test1() {}
}
