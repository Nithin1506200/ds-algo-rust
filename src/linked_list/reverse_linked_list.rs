use super::ListNode;
#[allow(dead_code)]
pub fn reverse_linked_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    while let Some(mut node) = head {
        head = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            reverse_linked_list(ListNode::from_array(&[1, 2, 5, 4, 3])),
            ListNode::from_array(&[3, 4, 5, 2, 1])
        )
    }
}
