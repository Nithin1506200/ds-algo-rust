// Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.

// You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.
// https://leetcode.com/problems/single-number-iii/description/?envType=daily-question&envId=2024-05-31

pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let x_or = nums.iter().fold(0, |acc, &e| (acc ^ e));
    let lowest_set_bit = x_or & (-x_or);
    let mut vec = vec![0, 0];
    for i in nums {
        if lowest_set_bit & i == 0 {
            vec[0] ^= i;
        } else {
            vec[1] ^= i
        }
    }
    vec
}
/**
 The expression lowestBit = xor2no & -xor2no is a bitwise operation often used in algorithms to isolate the lowest set bit (i.e., the least significant bit that is 1) of an integer.
Here's an explanation of how this works:

  * Bitwise AND (&): This operation takes two binary representations of equal length and performs the logical AND operation on each pair of corresponding bits.

  * Negation (-xor2no): In computer systems, negation of a number (two's complement) is achieved by inverting all the bits (one's complement) and adding 1.

  ```yaml
  0000 1100 -> 12
------------------
  1111 0011
  +       1
  1111 0100 -> -12
-------------------
   0000 1100
   1111 0100
 & 0000 0100 -> 4

  ```
*/
fn isolate_lowest_set_bit(x: i32) -> i32 {
    x & (-x)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_right_most_bit() {
        println!("{:?}", 3 & (-3));
    }

    #[test]
    fn single_number_3() {
        assert_eq!(single_number(vec![1, 1, 2, 2, 3, 5]), vec![5, 3]);
        assert_eq!(single_number(vec![1, 1, 4, 4, 46, 6]), vec![6, 46]);
    }
}
