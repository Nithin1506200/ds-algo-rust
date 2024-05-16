use super::LinkedList;
#[allow(dead_code)]
pub fn find_length(head: Option<Box<LinkedList>>) -> i32 {
    let mut ptr: &Option<Box<LinkedList>> = &head;
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
        let ll = LinkedList::from_array(&[1, 2, 3, 4, 5, 6]);
        println!("{:?}", ll);
        assert_eq!(find_length(ll), 6);
    }
}
