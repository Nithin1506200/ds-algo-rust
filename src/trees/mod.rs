mod delete_leaves_with_given_value;
mod distribute_coins;
mod is_same_tree;
mod max_depth;
mod maximum_difference_between_node_and_ancestor;
mod maximum_path_sum;
mod maximum_width_of_binary_tree;
mod path_sum_2;
mod sum_root_to_leaf;

use std::{cell::RefCell, collections::VecDeque, rc::Rc};
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

impl TreeNode {
    fn pretty_print_helper(&self, prefix: String, is_left: bool) {
        let connector = if is_left { "├── " } else { "└── " };
        println!("{}{}{}", prefix, connector, self.clone().val);

        let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });

        if let Some(ref left) = self.left.clone() {
            left.borrow().pretty_print_helper(new_prefix.clone(), true);
        }
        if let Some(ref right) = self.right {
            right.borrow().pretty_print_helper(new_prefix, false);
        }
    }
    pub fn pretty_print(node: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            node.borrow().pretty_print_helper(String::from(""), false)
        }
    }
    fn build_tree_from_preorder(
        preorder: &[Option<i32>],
        index: &mut usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if *index >= preorder.len() || preorder[*index].is_none() {
            *index += 1; // Move to next index for `None`
            return None;
        }

        let root_value = preorder[*index].unwrap();
        *index += 1;

        let root = Rc::new(RefCell::new(TreeNode::new(root_value)));
        root.borrow_mut().left = Self::build_tree_from_preorder(preorder, index);
        root.borrow_mut().right = Self::build_tree_from_preorder(preorder, index);

        Some(root)
    }

    pub fn build_tree_pre_order(preorder: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut index = 0;
        Self::build_tree_from_preorder(preorder, &mut index)
    }
    // Function to construct a binary tree from a level-order array
    pub fn construct_tree(arr: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.is_empty() || arr[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(arr[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while i < arr.len() {
            if let Some(node) = queue.pop_front() {
                // Process left child
                if i < arr.len() {
                    if let Some(val) = arr[i] {
                        let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                        node.borrow_mut().left = Some(left_node.clone());
                        queue.push_back(left_node);
                    }
                    i += 1;
                }

                // Process right child
                if i < arr.len() {
                    if let Some(val) = arr[i] {
                        let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                        node.borrow_mut().right = Some(right_node.clone());
                        queue.push_back(right_node);
                    }
                    i += 1;
                }
            }
        }

        Some(root)
    }
}

#[macro_use]
#[macro_export]
macro_rules! tree {
    ($($val:expr),*) => {{
        use super::*;
        let vec:Vec<Option<i32>> = vec![$(stringify!($val)),*].into_iter().map(|e| {
            if e==r#"null"# {
                None
            }
            else {
               match e.parse::<i32>() {
                Ok(e)=>Some(e),
                Err(_)=>panic!("not an number or null")
               }
            }
        }).collect();
        TreeNode::construct_tree(&vec)

    }};
}

// #[macro_use]
// #[macro_export]
// macro_rules! tree {
//     ($($val:expr),*) => {{
//         use super::*;
//         let mut iter = vec![$(stringify!($val)),*].into_iter();
//         fn build_tree<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<Rc<RefCell<TreeNode>>>
//          {
//             match iter.next() {
//                 Some(r#"null"#) => {
//                     None
//                 },
//                 Some(val) if val.parse::<i32>().is_ok() => {
//                     let parsed_val = val.parse::<i32>().unwrap();
//                     let node = Rc::new(RefCell::new(TreeNode::new(parsed_val)));
//                     node.borrow_mut().left=build_tree(iter);
//                     node.borrow_mut().right=build_tree(&mut iter.next());
//                     Some(node)
//                 },
//                 _ => None,
//             }
//         }

//         build_tree(&mut iter)
//     }};
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let x = tree![1, 2, 3, 8, null, 4, 9];
        TreeNode::pretty_print(&x);
    }
}
