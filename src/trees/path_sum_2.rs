//https://leetcode.com/problems/path-sum-ii/description/

//Given the root of a binary tree and an integer targetSum, return all root-to-leaf paths where the sum of the node values in the path equals targetSum. Each path should be returned as a list of the node values, not node references.
// A root-to-leaf path is a path starting from the root and ending at any leaf node. A leaf is a node with no children.

use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

pub fn path_sum2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    if let Some(node) = root {
        let val = node.borrow().val;
        match (node.borrow().left.clone(), node.borrow().right.clone()) {
            (None, None) => {
                if target_sum == val {
                    ans.push(vec![val])
                }
            }
            (_, _) => (),
        }
        let mut left: Vec<Vec<i32>> = if let Some(left) = node.borrow().left.clone() {
            let mut vec = path_sum2(Some(left), target_sum - val);
            for each_vec in &mut vec {
                each_vec.insert(0, val);
            }
            vec
        } else {
            vec![]
        };
        let mut right: Vec<Vec<i32>> = if let Some(right) = node.borrow().right.clone() {
            let mut vec = path_sum2(Some(right), target_sum - val);
            for each_vec in &mut vec {
                each_vec.insert(0, val);
            }
            vec
        } else {
            vec![]
        };
        if left.len() > 0 {
            ans.append(&mut left);
        }
        if right.len() > 0 {
            ans.append(&mut right);
        }
    }
    ans
}
