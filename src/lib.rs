mod basic;
mod binary_search;
mod bit_manipulation;
mod dp;
mod linked_list;
mod misc;
mod trees;
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
