#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn distribute_coins(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(current: &mut Option<Box<TreeNode>>, parent: Option<&mut Box<TreeNode>>) -> i32 {
        if let Some(current_node) = current {
            let mut left = current_node.left.take();
            let mut right = current_node.right.take();

            let left_moves = dfs(&mut left, Some(current_node));
            let right_moves = dfs(&mut right, Some(current_node));

            current_node.left = left;
            current_node.right = right;

            let total_moves = left_moves + right_moves;
            let x = current_node.val - 1;

            // Update the parent node's value
            if let Some(parent_node) = parent {
                parent_node.val += x;
            }

            total_moves + x.abs()
        } else {
            0
        }
    }

    let mut root = root;
    dfs(&mut root, None)
}

#[cfg(test)]
mod test {
    use super::{distribute_coins, TreeNode};

    #[test]
    fn test() {
        let mut example = Box::new(TreeNode::new(0));
        example.left = Some(Box::new(TreeNode::new(3)));
        example.right = Some(Box::new(TreeNode::new(0)));
        assert_eq!(distribute_coins(Some(example)), 3);
    }
    struct X {}
    fn test2() {
        let mut x: X = X {};
        fn doSomething(x: &mut X) {}
        fn doSomethingOwner(x: X) {}
        doSomething(&mut x);
        doSomething(&mut x);
        // doSomethingOwner(x);
        doSomething(&mut x)
    }
}
