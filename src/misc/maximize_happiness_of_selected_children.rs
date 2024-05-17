#![feature(custom_test_frameworks)]
pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
    let mut queue: Vec<i32> = vec![];
    for i in happiness.iter() {
        if queue.len() < k.try_into().unwrap() {
            queue.push(*i);
        } else {
            for q in queue.iter_mut() {
                if *q < *i {
                    *q = *i;
                }
            }
        }
    }
    let mut ans: i64 = 0;
    queue.sort();
    for (i, q) in queue.iter().enumerate() {
        ans += *q as i64 - (if i > 0 && *q > 1 { 1 } else { 0 });
        if i > 0 {}
    }

    ans
}
#[cfg(test)]
mod test {
    use super::maximum_happiness_sum;

    #[test]
    fn test() {
        maximum_happiness_sum(vec![1, 2, 3], 2);
    }
}
