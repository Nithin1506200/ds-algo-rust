// Remove N-th node from the end of a Linked List
// https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/

use super::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut first: *const Box<ListNode> = &dummy;
    let mut second = &mut dummy;
    // Move first pointer n+1 steps ahead
    for _ in 0..=n {
        if let Some(node) = (unsafe { &*first }).next.as_ref() {
            // &* is needed
            first = node;
        } else {
            return dummy.next; // If n is greater than length, return original head
        }
    }
    // Move both pointers until first reaches the end
    while let Some(first_) = unsafe { (*first).next.as_ref() } {
        first = first_;
        second = second.next.as_mut().unwrap();
    }
    // Remove the Nth node
    second.next = second.next.as_mut().unwrap().next.take();
    dummy.next
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let list = ListNode::from_array(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(
            ListNode::to_array(&remove_nth_from_end(list, 2)),
            [1, 2, 3, 4, 5, 6, 8, 9]
        );
        let list = ListNode::from_array(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(
            ListNode::to_array(&remove_nth_from_end(list, 0)),
            [1, 2, 3, 4, 5, 6, 7, 8]
        );
        let list = ListNode::from_array(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(
            ListNode::to_array(&remove_nth_from_end(list, 1)),
            [1, 2, 3, 4, 5, 6, 7, 9]
        )
    }
}
