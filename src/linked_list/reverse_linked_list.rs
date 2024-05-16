use super::LinkedList;
#[allow(dead_code)]
pub fn reverse_linked_list(mut head: Option<Box<LinkedList>>) -> Option<Box<LinkedList>> {
    let mut reversed: Option<Box<LinkedList>> = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = reversed;
        reversed = Some(node);
    }
    reversed
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ll = reverse_linked_list(LinkedList::from_array(&[1, 2, 5, 4, 3]));
        println!("Solution {:?}", ll);
    }
}
