use std::collections::HashMap;

// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/?envType=daily-question&envId=2024-05-27
pub fn special_array(nums: Vec<i32>) -> i32 {
    let mut max = -1;
    for x in 1..nums.len() + 1 {
        let mut n: i32 = 0;
        for i in &nums {
            if *i >= x as i32 {
                n += 1
            }
        }
        max = if n == x as i32 { n.max(max) } else { max };
    }
    max
}
pub fn special_array_2(nums: Vec<i32>) -> i32 {
    let len = nums.len() as i32;
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for i in nums {
        freq.insert(i.min(len), *freq.get(&i.min(len)).get_or_insert(&0) + 1);
    }
    let mut count = 0;
    for i in (1..len + 1).rev() {
        let ele = **freq.get(&i).get_or_insert(&0);
        count += ele;
        if count == i {
            return count;
        }
    }

    return -1;
}
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test() {
        println!("{:?}", special_array_2(vec![3, 5]));
    }
}
