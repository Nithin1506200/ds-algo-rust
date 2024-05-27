struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn do_magic(mut node: Node) {}

fn do_magic2(node: &mut Node) {}
fn do_magic_ref(node: &Node) {}

fn magical_function(mut node: Node) -> i32 {
    let current_borrow = &node;

    // do_magic(node);
    do_magic_ref(&node);
    // do_magic2(&mut node);

    let new_val = current_borrow.val + 1;

    new_val
}
