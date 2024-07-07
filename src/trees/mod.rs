mod delete_leaves_with_given_value;
mod distribute_coins;
mod is_same_tree;
mod maximum_difference_between_node_and_ancestor;
mod maximum_path_sum;
mod maximum_width_of_binary_tree;
mod path_sum_2;

use std::{cell::RefCell, rc::Rc};
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
#[macro_use]
#[macro_export]
macro_rules! tree {
    ($($val:expr),*) => {{
        use super::*;
        let mut iter = vec![$(stringify!($val)),*].into_iter();
        fn build_tree<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<Rc<RefCell<TreeNode>>>
         {
            match iter.next() {
                Some(r#""null""#) => None,
                Some(val) if val.parse::<i32>().is_ok() => {
                    let parsed_val = val.parse::<i32>().unwrap();
                    let node = Rc::new(RefCell::new(TreeNode::new(parsed_val)));
                    node.borrow_mut().left=build_tree(iter);
                    node.borrow_mut().right=build_tree(iter);
                    Some(node)
                },
                _ => None,
            }
        }

        build_tree(&mut iter)
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let x = tree![1, 2, 3, "null"];
    }
}
