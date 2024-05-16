use super::LinkedList;

//https://leetcode.com/problems/remove-nodes-from-linked-list/?envType=daily-question&envId=2024-05-06

// fn helper(
//     node: Option<Box<LinkedList>>,
//     mut prev: Option<Box<LinkedList>>,
//     max: &mut i32,
// ) -> Option<Box<LinkedList>> {
//     if let Some(mut n) = node {
//         let next = n.next.take();
//         let next_prev = Some(n);
//         helper(next, next_prev, max);
//         *max = *max.max(&mut n.val);
//         if n.val < *max {
//             if let Some(prev_node) = prev {
//                 prev_node.next = n.next;
//             } else {
//                 return n.next;
//             }
//         }
//         Some(n)
//     } else {
//         None
//     }
// }
// pub fn remove_ele_if_greater_right_exist(
//     mut head: Option<Box<LinkedList>>,
// ) -> Option<Box<LinkedList>> {
//     let mut max = 0;
//     helper(&mut max, head, None).into()
// }
#[allow(dead_code)]

pub fn solution(node: Option<Box<LinkedList>>) -> Option<Box<LinkedList>> {
    fn solve(node: Option<Box<LinkedList>>) -> Option<Box<LinkedList>> {
        if let Some(mut node) = node {
            node.next = solve(node.next);
            let r: Option<Box<LinkedList>> = match node.next {
                Some(ref next) => {
                    if next.val > node.val {
                        node.next
                    } else {
                        Some(node)
                    }
                }
                _ => Some(node),
            };
            return r;
        } else {
            None
        }
    }
    solve(node)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ll = LinkedList::from_array(&[1, 2, 5, 4, 3]);
        println!("Solution {:?}", solution(ll));
    }
}
