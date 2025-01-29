// https://leetcode.com/problems/maximum-subarray/

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    let mut temp_max = 0;
    for val in nums {
        println!(" before {} {} {}", val, temp_max, max);
        if temp_max < 0 {
            temp_max = val
        } else {
            temp_max = temp_max + val
        }

        max = max.max(temp_max);
        println!(" after    {} {}", temp_max, max);
    }
    max
}
pub fn max_sub_array_2(nums: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    let mut temp_max = 0;
    for val in nums {
        println!(" before {} {} {}", val, temp_max, max);
        temp_max = val.max(val + temp_max);

        max = max.max(temp_max);
        println!(" after    {} {}", temp_max, max);
    }
    max
}

#[cfg(test)]
mod test {
    use super::max_sub_array;

    #[test]
    fn test() {
        println!("{}", max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
        // assert!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6);
    }
}
