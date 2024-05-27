//https://leetcode.com/problems/distribute-coins-in-binary-tree/?envType=daily-question&envId=2024-05-18

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::TreeNode;

// You have to release the borrow of node_ while you operate on its children because it will be a parent at some point and thus be mutably borrowed as well. You can reacquire the immutable borrow on node_ after you work on its children. (I cannot speak to the correctness of the algorithm in general; once the panic is fixed, the test still fails.) You should change the Some branch to the following:
pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(current: Option<Rc<RefCell<TreeNode>>>, parent: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match current {
            Some(node_) => {
                let node: Ref<TreeNode> = node_.borrow();
                let left = node.left.clone();
                let right = node.right.clone();
                drop(node);
                let left_moves = dfs(left, Some(Rc::clone(&node_)));
                let right_moves = dfs(right, Some(Rc::clone(&node_)));
                let moves = left_moves + right_moves;
                let node: Ref<TreeNode> = node_.borrow();
                let x = node.val - 1;

                if let Some(ref parent) = parent {
                    parent.borrow_mut().val += x
                }
                moves + x.abs()
            }
            None => 0,
        }
    }
    dfs(root, None)
}

pub fn distribute_coins2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(n: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> Option<i32> {
        let n = n.as_ref()?;
        let n = n.borrow();
        let flow =
            dfs(n.left.clone(), res).unwrap_or(0) + dfs(n.right.clone(), res).unwrap_or(0) + n.val
                - 1;
        *res += flow.abs();
        Some(flow)
    }
    let mut res = 0;
    dfs(root, &mut res);
    res
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::trees::{distribute_coins::distribute_coins, TreeNode};

    #[test]
    fn test() {
        let mut example = TreeNode::new(0);
        let left = TreeNode::new(3);
        let right = TreeNode::new(0);
        example.left = Some(Rc::new(RefCell::new(left)));
        example.right = Some(Rc::new(RefCell::new(right)));
        assert!(distribute_coins(Some(Rc::new(RefCell::new(example)))) == 3);
    }
}
