use std::collections::HashMap;
// https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/editorial/?envType=daily-question&envId=2024-05-30
pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let mut ans = 0;

    for i in 0..arr.len() {
        let mut hp: HashMap<i32, i32> = HashMap::new();
        {
            let mut prev = 0;
            for i_ in (0..i + 1).rev() {
                let ele = arr[i_];
                let curr = ele ^ prev;
                prev = curr;
                hp.insert(curr, **hp.get(&curr).get_or_insert(&0) + 1);
            }
        }
        let mut prev = 0;
        for j in i + 1..arr.len() {
            let ele = arr[j];
            let curr = prev ^ ele;
            prev = curr;
            ans = ans + **hp.get(&(curr)).get_or_insert(&0);
        }
    }
    ans
}
pub fn count_triplets2(arr: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    let mut xor_arr: Vec<i32> = vec![0];
    xor_arr.append(&mut arr.clone());
    let size = xor_arr.len();
    for i in 1..size {
        xor_arr[i] ^= xor_arr[i - 1];
    }
    for i in 0..size {
        for j in i + 1..size {
            if xor_arr[i] == xor_arr[j] {
                count += (j as i32) - (i as i32) - 1;
            }
        }
    }
    count
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert!(count_triplets(vec![]) == 0);
        assert!(count_triplets(vec![2, 3, 1, 6, 7]) == 4);
        assert!(count_triplets(vec![1, 1, 1, 1, 1]) == 10);
        assert!(count_triplets2(vec![]) == 0);
        assert!(count_triplets2(vec![2, 3, 1, 6, 7]) == 4);
        assert!(count_triplets2(vec![1, 1, 1, 1, 1]) == 10);
    }
}
