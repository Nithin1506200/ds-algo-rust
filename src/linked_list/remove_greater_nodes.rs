// Remove Nodes From Linked List
// https://leetcode.com/problems/remove-nodes-from-linked-list/description/

// Input: head = [5,2,13,3,8]
// Output: [13,8]
// Explanation: The nodes that should be removed are 5, 2 and 3.
// - Node 13 is to the right of node 5.
// - Node 13 is to the right of node 2.
// - Node 8 is to the right of node 3.

// Example 2:

// Input: head = [1,1,1,1]
// Output: [1,1,1,1]
// Explanation: Every node has value 1, so no nodes are removed.

use super::ListNode;

fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn solve(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut val) => {
                val.next = solve(val.next);
                match val.next {
                    Some(ref next_node) => {
                        if next_node.val > val.val {
                            val.next
                        } else {
                            Some(val)
                        }
                    }
                    None => Some(val),
                }
            }
            None => None,
        }
    }

    solve(head)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::linked_list::ListNode;
    #[test]
    fn test() {
        assert_eq!(
            remove_nodes(ListNode::from_array(&vec![5, 2, 13, 8])),
            ListNode::from_array(&vec![13, 8])
        );
    }
}
