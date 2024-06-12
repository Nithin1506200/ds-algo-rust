// https://leetcode.com/problems/odd-even-linked-list/?envType=study-plan-v2&envId=leetcode-75

use super::ListNode;

/**
 * Given the head of a singly linked list, group all the nodes with odd indices together followed by the nodes with even indices, and return the reordered list.

The first node is considered odd, and the second node is even, and so on.

Note that the relative order inside both the even and odd groups should remain as it was in the input.

You must solve the problem in O(1) extra space complexity and O(n) time complexity.
 */
// pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     if head.as_ref().is_none() || head.as_ref().unwrap().next.is_none() {
//         return head;
//     }

//     let mut odd = head.as_mut().unwrap();
//     let mut even = odd.next.as_mut().unwrap();
//     let mut even_head = odd.next.take();

//     while even.next.is_some() {
//         odd.next = even.as_mut().next.take();
//         odd = odd.next.as_mut().unwrap();

//         if odd.next.is_some() {
//             even.next = odd.next.take();
//             even = even.next.as_mut().unwrap();
//         }
//     }

//     odd.next = even_head;

//     head
// }

pub fn odd_even_list_(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    } else if head.as_ref().unwrap().next.is_none() {
        return head;
    }

    let mut odd = head;
    let mut even = odd.as_ref().unwrap().next.to_owned();

    let mut odd_tail = odd.as_mut();
    let mut even_tail = even.as_mut();

    while even_tail.is_some() {
        let new_odd = even_tail.as_ref().unwrap().next.to_owned();
        let new_even = match new_odd.as_ref() {
            Some(node) => node.next.to_owned(),
            None => None,
        };

        odd_tail.as_mut().unwrap().next = new_odd;
        even_tail.as_mut().unwrap().next = new_even;

        if odd_tail.as_ref().unwrap().next.is_some() {
            odd_tail = odd_tail.unwrap().next.as_mut();
            even_tail = even_tail.unwrap().next.as_mut();
        } else {
            break;
        }
    }

    odd_tail.unwrap().next = even;

    odd
}
