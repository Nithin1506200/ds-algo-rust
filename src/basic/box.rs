//A pointer type that uniquely owns a heap allocation of type T.

use crate::linked_list::ListNode;

#[test]
fn to_owned_test() {
    //creation of box
    let x = Box::new(1);
    let mut y = x.to_owned();
    *y.as_mut() = 9;

    assert_eq!(*x.as_ref(), 1);
    assert_eq!(*y.as_ref(), 9);
}

#[test]
fn linked_list_test() {
    let mut linked_list = ListNode::from_array(&[1, 2]);
    let mut tail = linked_list.as_ref().unwrap().next.to_owned();
    // match tail.as_mut() {
    //     Some(mut tail) => tail.as_mut().val = 9,
    //     None => (),
    // }

    // println!("{:?} {:?}", &linked_list, &tail)
}
