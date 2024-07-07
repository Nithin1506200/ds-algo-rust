mod arrays;
mod basic;
mod binary_search;
mod bit_manipulation;
mod dp;
mod greedy;
mod hash_table;
mod linked_list;
mod misc;
mod sliding_window;
mod sorting;
mod trees;
mod trie;
mod two_pointer;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
