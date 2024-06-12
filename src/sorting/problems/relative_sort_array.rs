use std::collections::HashMap;

pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut hp: HashMap<i32, i32> = HashMap::new();

    for i in &arr1 {
        hp.entry(*i).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut result = vec![];

    for value in arr2 {
        while *hp.get(&value).unwrap_or(&0) > 0 {
            result.push(value);
            hp.entry(value).and_modify(|x| *x -= 1).or_insert(1);
        }
    }
    let mut remaining = vec![];
    for num in &arr1 {
        while *hp.get(num).unwrap_or(&0) > 0 {
            remaining.push(*num);
            hp.entry(*num).and_modify(|x| *x -= 1).or_insert(1);
        }
    }
    remaining.sort();

    result.append(&mut remaining);
    result
}

#[test]
fn test() {
    assert_eq!(
        relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        ),
        vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
    );
    assert_eq!(
        relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
        vec![22, 28, 8, 6, 17, 44]
    )
}
