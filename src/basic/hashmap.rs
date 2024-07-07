use std::collections::{HashMap, HashSet};

#[test]
fn init_hashmap() {
    let hp: HashMap<i32, i32> = HashMap::new();
}

#[test]
fn hash_set() {
    let mut hs: HashSet<i32> = HashSet::new();
    hs.insert(3);
}
