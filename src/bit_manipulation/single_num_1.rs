pub fn single_num_1(arr: Vec<i32>) -> i32 {
    let mut x = 0;
    for i in arr {
        x ^= i;
    }
    x
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(single_num_1(vec![1, 1, 2, 2, 4]), 4);
        assert_eq!(single_num_1(vec![1, 1, 2, 4, 4]), 2);
        assert_eq!(single_num_1(vec![1, 1, 2, 2, 41, 41, 89]), 89);
    }
}
