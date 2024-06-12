use super::ListNode;
#[allow(dead_code)]
pub fn find_length(head: Option<Box<ListNode>>) -> i32 {
    let mut ptr: &Option<Box<ListNode>> = &head;
    let mut len = 0;
    while ptr.is_some() {
        match ptr {
            Some(ll) => {
                len += 1;
                ptr = &ll.next;
            }
            None => (),
        }
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ll = ListNode::from_array(&[1, 2, 3, 4, 5, 6]);
        println!("{:?}", ll);
        assert_eq!(find_length(ll), 6);
    }
}
