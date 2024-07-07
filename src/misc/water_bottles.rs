// https://leetcode.com/problems/water-bottles/description/?envType=daily-question&envId=2024-07-07

fn helper(num_bottles: i32, num_exchange: i32, num_empty: i32) -> i32 {
    let d = num_bottles / num_exchange;
    let r = num_bottles % num_exchange + num_empty;
    num_bottles
        + if d + r < num_exchange {
            d
        } else {
            helper(d + r / num_exchange, num_exchange, r % num_exchange)
        }
}
pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    helper(num_bottles, num_exchange, 0)
}
