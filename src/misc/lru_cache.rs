//https://www.youtube.com/watch?v=k0cL6K28SL0
//https://www.youtube.com/watch?v=EfjN80PyMoM
use crate::linked_list::double_ll::{self, DoubleLinkedList};
use std::{
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    rc::{Rc, Weak},
};
// LEAST RECENTLY USED

pub struct LRU<K: Copy + Eq + Hash, T: Copy> {
    list: double_ll::DoubleLinkedList<T>,
    map: HashMap<K, Weak<RefCell<double_ll::Node<T>>>>,
    capacity: usize,
}

impl<K: Copy + Eq + Hash + std::fmt::Debug, T: Copy> LRU<K, T> {
    pub fn new(capacity: usize) -> Self {
        LRU {
            list: DoubleLinkedList::new(),
            map: HashMap::new(),
            capacity,
        }
    }
    pub fn get(&mut self, k: K) -> Option<T> {
        let node_ptr = self.map.get(&k)?;
        let node = node_ptr.upgrade()?;
        let val = node.borrow().val;
        self.list.move_node_back(node.clone());
        match self.list.peep_back() {
            //very important
            Some(node) => self.map.insert(k, Rc::downgrade(&node)),
            None => self.map.remove(&k),
        };
        Some(val)
    }
    pub fn put(&mut self, k: K, v: T) {
        match self.map.get(&k) {
            Some(node) => {
                node.upgrade().map(|node| {
                    self.list.remove_node(node);
                });
            }
            None => (),
        }
        self.list.push_back(v);
        let last = self.list.peep_back();
        match last {
            Some(last) => {
                let weak = Rc::downgrade(&last);
                self.map.insert(k, weak);
            }
            None => (),
        }
        if self.list.len() > self.capacity {
            self.list.pop_front();
        }
    }
}

#[cfg(test)]
mod test {
    use super::LRU;

    #[test]
    fn test() {
        let mut lru: LRU<i32, i32> = LRU::new(3);
        lru.put(1, 1);
        assert_eq!(lru.get(1), Some(1));
        assert_eq!(lru.get(1), Some(1));
        assert_eq!(lru.list.into_vec(), vec![1]);
        lru.put(2, 2);
        assert_eq!(lru.list.into_vec(), vec![1, 2]);
        lru.put(3, 3);
        assert_eq!(lru.list.into_vec(), vec![1, 2, 3]);
        assert_eq!(lru.get(3), Some(3));
        assert_eq!(lru.list.into_vec(), vec![1, 2, 3]);
        lru.put(1, 11);
        assert_eq!(lru.list.into_vec(), vec![2, 3, 11]);
        lru.put(8, 8);
        assert_eq!(lru.list.into_vec(), vec![3, 11, 8]);
    }
}
