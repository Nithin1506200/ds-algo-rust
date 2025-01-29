use std::ptr::eq;

pub mod double_ll;
mod find_length;
mod odd_even_linked_list;

mod remove_greater_nodes;
mod remove_nth_from_last;
mod reverse_linked_list;
#[allow(dead_code)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[allow(dead_code)]
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn append(&mut self, data: i32) {
        let new_node = Box::new(ListNode::new(data));
        let mut current = self;
        loop {
            if let Some(ref mut node) = current.next {
                current = node
            } else {
                current.next = Some(new_node);
                break;
            }
        }
    }
    pub fn to_array(a: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        let mut ptr = a.as_ref();
        while let Some(node) = ptr {
            v.push(node.val);
            ptr = node.next.as_ref();
        }
        v
    }

    pub fn from_array(arr: &[i32]) -> Option<Box<ListNode>> {
        // Initialize the head of the linked list
        let mut head = None;
        let mut current = &mut head;

        // Iterate over each element in the array
        for &val in arr {
            // Create a new node with the current value
            let new_node = Box::new(ListNode { val, next: None });

            // Set the next field of the current node to the new node
            *current = Some(new_node);

            // Move to the next node
            if let Some(ref mut node) = current {
                current = &mut node.next;
            }
        }

        // Return the head of the linked list
        head
    }
}

// impl PartialEq for ListNode {
//     fn eq(&self, other: &Self) -> bool {
//         if self.val == other.val {
//             match (&self.next, &other.next) {
//                 (Some(a), Some(b)) => eq(a, b),
//                 (None, None) => true,
//                 _ => false,
//             }
//         } else {
//             false
//         }
//     }
// }
