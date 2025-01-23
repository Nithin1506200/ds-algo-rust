//https://www.youtube.com/watch?v=k0cL6K28SL0
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node<T: Copy> {
    pub val: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

pub struct DoubleLinkedList<T: Copy> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T: Copy> From<&Vec<T>> for DoubleLinkedList<T> {
    fn from(value: &Vec<T>) -> Self {
        let mut list = DoubleLinkedList::new();
        for i in value {
            list.push_back(*i);
        }
        list
    }
}
impl<T: Copy> Into<Vec<T>> for DoubleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut arr: Vec<T> = Vec::new();
        let mut ptr = self.head; // Start from the head of the list
        while let Some(node) = ptr.clone() {
            // Borrow the value of the current node
            let val = node.borrow_mut().val;
            arr.push(val);
            ptr = node.borrow_mut().next.clone();
        }
        arr
    }
}

impl<T: Copy> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }
    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);
        match self.tail.take() {
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(&current_tail));
                self.tail = Some(Rc::new(RefCell::new(node)));
                current_tail.borrow_mut().next = self.tail.clone();
            }
            None => {
                self.head = Some(Rc::new(RefCell::new(node)));
                self.tail = self.head.clone();
            }
        }
        self.len += 1;
    }
    pub fn push_front(&mut self, val: T) {
        let mut node = Node::new(val);
        match self.head.take() {
            Some(ptr) => {
                node.next = Some(ptr.clone());
                let n = Rc::new(RefCell::new(node));
                self.head = Some(n.clone());
                ptr.borrow_mut().prev = Some(Rc::downgrade(&n))
            }
            None => {
                self.head = Some(Rc::new(RefCell::new(node)));
                self.tail = self.head.clone();
            }
        }
        self.len += 1;
    }
    #[allow(dead_code)]
    pub fn pop_back(&mut self) -> Option<T> {
        self.len = 0.max(self.len - 1);
        match &self.tail.take() {
            Some(node) => {
                if let Some(prev) = &node.borrow_mut().prev {
                    if let Some(prev) = prev.upgrade() {
                        self.tail = Some(prev.clone());
                    } else {
                        self.head = None
                    }
                } else {
                    self.head = None
                }
                Some(node.borrow_mut().val)
            }
            None => None,
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.len = 0.max(self.len - 1);
        match &self.head.take() {
            Some(node) => {
                self.head = node.borrow_mut().next.clone();
                if let None = self.head {
                    self.tail = None
                }
                Some(node.borrow_mut().val)
            }
            None => None,
        }
    }
    pub fn remove_node(&mut self, node: Rc<RefCell<Node<T>>>) {
        let node = node.borrow_mut();
        self.len = 0.max(self.len - 1);
        match (&node.prev, &node.next) {
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            (None, Some(next)) => {
                next.clone().borrow_mut().prev = None;
                self.head = Some(next.clone());
            }
            (Some(prev), None) => {
                let prev = prev.clone().upgrade().map(move |node| {
                    node.borrow_mut().next = None;
                    node
                });
                self.tail = prev;
            }
            (Some(prev), Some(next)) => match prev.upgrade() {
                Some(prev) => prev.borrow_mut().next = Some(next.clone()),
                None => {
                    next.clone().borrow_mut().prev = None;
                    self.head = Some(next.clone())
                }
            },
        }
    }
    pub fn move_node_back(&mut self, node: Rc<RefCell<Node<T>>>) {
        self.remove_node(node.clone());
        self.push_back(node.borrow_mut().val);
    }
    pub fn peep_front(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.head.clone()
    }
    pub fn peep_back(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.tail.clone()
    }
    pub fn into_vec(&self) -> Vec<T> {
        let mut vec = vec![];
        let mut ptr = self.head.clone();
        while let Some(ref ptr_) = ptr.clone() {
            vec.push(ptr_.borrow_mut().val);
            ptr = ptr_.borrow_mut().next.clone();
        }
        vec
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use super::DoubleLinkedList;

    #[test]
    fn list_from_array() {
        let arr = vec![1, 43, 5, 63, 75, 7];
        let double_ll = DoubleLinkedList::from(&arr);
        let res: Vec<i32> = double_ll.into();
        assert_eq!(arr, res);
    }
    #[test]
    fn push_back() {
        let mut ll = DoubleLinkedList::new();
        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(9);
        assert_eq!(vec![1, 2, 9], Into::<Vec<i32>>::into(ll));
    }
    #[test]
    fn push_front() {
        let mut ll = DoubleLinkedList::new();
        ll.push_front(9);
        ll.push_front(2);
        ll.push_front(1);
        assert_eq!(vec![1, 2, 9], Into::<Vec<i32>>::into(ll));
    }
    #[test]
    fn push() {
        let mut ll = DoubleLinkedList::new();
        ll.push_front(9);
        ll.push_back(12);
        ll.push_back(29);
        ll.push_front(2);
        ll.push_back(2);
        ll.push_front(12);
        assert_eq!(vec![12, 2, 9, 12, 29, 2], Into::<Vec<i32>>::into(ll));
    }
    #[test]
    fn pop_front() {
        let mut ll = DoubleLinkedList::from(&vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(ll.pop_front(), Some(1));
        assert_eq!(ll.pop_front(), Some(2));
        assert_eq!(ll.pop_front(), Some(3));
        assert_eq!(ll.pop_front(), Some(4));
        assert_eq!(ll.pop_front(), Some(5));
        assert_eq!(ll.pop_front(), Some(6));
        assert_eq!(ll.pop_front(), Some(7));
        assert_eq!(ll.pop_front(), Some(8));
        assert_eq!(ll.head.is_none(), true);
        assert_eq!(ll.tail.is_none(), true);
        assert_eq!(ll.pop_front(), None);
        assert_eq!(ll.pop_front(), None);
        assert_eq!(ll.head.is_none(), true);
        assert_eq!(ll.tail.is_none(), true);
    }
    #[test]
    fn pop_back() {
        let mut ll = DoubleLinkedList::from(&vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(ll.pop_back(), Some(8));
        assert_eq!(ll.pop_back(), Some(7));
        assert_eq!(ll.pop_back(), Some(6));
        assert_eq!(ll.pop_back(), Some(5));
        assert_eq!(ll.pop_back(), Some(4));
        assert_eq!(ll.pop_back(), Some(3));
        assert_eq!(ll.pop_back(), Some(2));
        assert_eq!(ll.pop_back(), Some(1));
        assert_eq!(ll.head.is_none(), true);
        assert_eq!(ll.tail.is_none(), true);
        assert_eq!(ll.pop_back(), None);
        assert_eq!(ll.pop_back(), None);
        assert_eq!(ll.head.is_none(), true);
        assert_eq!(ll.tail.is_none(), true);
    }
}
