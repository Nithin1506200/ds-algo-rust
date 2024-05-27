use std::cell::RefCell;
use std::rc::Rc;

use super::TreeNode;
use std::collections::VecDeque;
//https://leetcode.com/problems/maximum-width-of-binary-tree/

// has memory issues
pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let mut max: i32 = 0;
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(Some(node));
        while !queue.is_empty() {
            while let Some(None) = queue.front() {
                queue.pop_front();
            }
            while let Some(None) = queue.back() {
                queue.pop_back();
            }

            max = max.max(queue.len() as i32);

            let len = queue.len();

            for i in 0..len {
                let nodes = queue.pop_front().unwrap();
                let (left, right) = if let Some(node) = nodes {
                    let node_ref = node.borrow();
                    let left = match node_ref.left {
                        Some(ref e) => Some(e.clone()),
                        None => None,
                    };
                    let right = match node_ref.right {
                        Some(ref e) => Some(e.clone()),
                        None => None,
                    };
                    // queue.push_back(left); // doesnt work as it is borrowed
                    (left, right)
                } else {
                    (None, None)
                };
                queue.push_back(left);
                queue.push_back(right);
            }
        }
        max
    } else {
        return 0;
    }
}

pub fn width_of_binary_tree_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;

    if root.is_none() {
        return 0;
    }

    let mut deque = VecDeque::new();
    deque.push_back((root.unwrap(), 0));

    while !deque.is_empty() {
        let size = deque.len();
        let (mut l, mut r) = (i32::MAX, i32::MIN);

        for _i in 0..size {
            if let Some((node, count)) = deque.pop_front() {
                l = l.min(count);
                r = r.max(count);

                if let Some(left) = &node.borrow().left {
                    deque.push_back((Rc::clone(left), 2 * count + 1));
                }
                if let Some(right) = &node.borrow().right {
                    deque.push_back((Rc::clone(right), 2 * count + 2));
                }
            }
        }

        res = res.max(r - l + 1);
    }

    res
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::trees::TreeNode;

    use super::{width_of_binary_tree, width_of_binary_tree_2};

    #[test]
    fn test() {
        let mut root = TreeNode::new(1);
        let mut two = TreeNode::new(2);
        let mut three = TreeNode::new(3);
        let three2 = TreeNode::new(3);
        let five = TreeNode::new(5);
        let nine = TreeNode::new(9);
        two.right = Some(Rc::new(RefCell::new(nine)));
        three.left = Some(Rc::new(RefCell::new(five)));
        three.right = Some(Rc::new(RefCell::new(three2)));
        root.left = Some(Rc::new(RefCell::new(three)));
        root.right = Some(Rc::new(RefCell::new(two)));
        let x = Rc::new(RefCell::new(root));
        assert!(width_of_binary_tree(Some(Rc::clone(&x))) == 4);

        assert!(width_of_binary_tree_2(Some(Rc::clone(&x))) == 4);
    }
}
