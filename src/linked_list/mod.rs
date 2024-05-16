mod find_length;
mod remove_ele_if_greater_right_exist;
mod reverse_linked_list;
#[allow(dead_code)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LinkedList {
    pub val: i32,
    pub next: Option<Box<LinkedList>>,
}
#[allow(dead_code)]
impl LinkedList {
    #[inline]
    fn new(val: i32) -> Self {
        LinkedList { next: None, val }
    }
    fn append(&mut self, data: i32) {
        let new_node = Box::new(LinkedList::new(data));
        let mut current = self;
        loop {
            if let Some(ref mut node) = current.next {
                current = node
            } else {
                current.next = Some(new_node);
                break;
            }
        }
    }

    pub fn from_array(arr: &[i32]) -> Option<Box<LinkedList>> {
        // Initialize the head of the linked list
        let mut head = None;
        let mut current = &mut head;

        // Iterate over each element in the array
        for &val in arr {
            // Create a new node with the current value
            let new_node = Box::new(LinkedList { val, next: None });

            // Set the next field of the current node to the new node
            *current = Some(new_node);

            // Move to the next node
            if let Some(ref mut node) = current {
                current = &mut node.next;
            }
        }

        // Return the head of the linked list
        head
    }
}
