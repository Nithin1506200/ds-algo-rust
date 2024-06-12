use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut hs = HashSet::new();
    let mut ans = 0;
    for i in nums {
        hs.insert(i);
    }
    for i in &hs {
        if !hs.contains(&(*i - 1)) {
            let mut j = *i + 1;
            while hs.contains(&j) {
                j += 1;
            }
            ans = ans.max(j - i);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
