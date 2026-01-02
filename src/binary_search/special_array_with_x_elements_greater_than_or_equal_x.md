# special array with x elements greater than or equal to x

Leetcode : <https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/?envType=daily-question&envId=2024-05-27>

---

## description

You are given an array nums of non-negative integers. nums is considered special if there exists a number x such that there are exactly x numbers in nums that are greater than or equal to x.

Notice that x does not have to be an element in nums.

Return x if the array is special, otherwise, return -1. It can be proven that if nums is special, the value for x is unique

---

| Code                                                                     | Explanation                                                                       |
| ------------------------------------------------------------------------ | --------------------------------------------------------------------------------- |
| `pub fn special_array_2(nums: Vec<i32>) -> i32 {`                        | Defines a function that returns the special value `x` or `-1` if it doesn’t exist |
| `let len = nums.len() as i32;`                                           | Stores the length of the array; possible values of `x` range from `0` to `len`    |
| `let mut freq: HashMap<i32, i32> = HashMap::new();`                      | Creates a HashMap to count occurrences of each number                             |
| `for i in nums {`                                                        | Iterates through each element in the array                                        |
| `i.min(len)`                                                             | Caps values larger than `len` since numbers greater than `len` behave the same    |
| `freq.insert(i.min(len), *freq.get(&i.min(len)).get_or_insert(&0) + 1);` | Increments the frequency count for the capped value                               |
| `let mut count = 0;`                                                     | Tracks how many elements are greater than or equal to the current value           |
| `for i in (1..len + 1).rev() {`                                          | Iterates from `len` down to `1`                                                   |
| `let ele = **freq.get(&i).get_or_insert(&0);`                            | Retrieves how many elements are exactly equal to `i`                              |
| `count += ele;`                                                          | Accumulates the number of elements ≥ `i`                                          |
| `if count == i {`                                                        | Checks whether exactly `i` elements are ≥ `i`                                     |
| `return count;`                                                          | Returns the special value `x` when found                                          |
| `-1`                                                                     | Returned when no valid special value exists                                       |

---

## Examples Walkthrough

### Example 1: Simple Special Array

**Input**: `nums = [3, 5]`

**Step 1: Build Frequency Map**
- Array length: `len = 2`
- Process each element:
  - `3.min(2) = 2` → freq[2] = 1
  - `5.min(2) = 2` → freq[2] = 2

**Frequency Map**: `{2: 2}`

**Step 2: Iterate Backwards from len to 1**

| i | freq[i] | count (cumulative) | count == i? | Action |
|---|---------|-------------------|-------------|--------|
| 2 | 2       | 0 + 2 = 2         | 2 == 2 ✓    | Return 2 |

**Result**: `2` (exactly 2 elements are ≥ 2)

---

### Example 2: No Special Value

**Input**: `nums = [0, 0]`

**Step 1: Build Frequency Map**
- Array length: `len = 2`
- Process each element:
  - `0.min(2) = 0` → freq[0] = 1
  - `0.min(2) = 0` → freq[0] = 2

**Frequency Map**: `{0: 2}`

**Step 2: Iterate Backwards from len to 1**

| i | freq[i] | count (cumulative) | count == i? | Action |
|---|---------|-------------------|-------------|--------|
| 2 | 0       | 0 + 0 = 0         | 0 != 2 ✗    | Continue |
| 1 | 0       | 0 + 0 = 0         | 0 != 1 ✗    | Continue |

**Result**: `-1` (no valid special value exists)

---

### Example 3: Complex Array

**Input**: `nums = [0, 4, 3, 0, 4]`

**Step 1: Build Frequency Map**
- Array length: `len = 5`
- Process each element:
  - `0.min(5) = 0` → freq[0] = 1
  - `4.min(5) = 4` → freq[4] = 1
  - `3.min(5) = 3` → freq[3] = 1
  - `0.min(5) = 0` → freq[0] = 2
  - `4.min(5) = 4` → freq[4] = 2

**Frequency Map**: `{0: 2, 3: 1, 4: 2}`

**Step 2: Iterate Backwards from len to 1**

| i | freq[i] | count (cumulative) | count == i? | Action |
|---|---------|-------------------|-------------|--------|
| 5 | 0       | 0 + 0 = 0         | 0 != 5 ✗    | Continue |
| 4 | 2       | 0 + 2 = 2         | 2 != 4 ✗    | Continue |
| 3 | 1       | 2 + 1 = 3         | 3 == 3 ✓    | Return 3 |

**Result**: `3` (exactly 3 elements are ≥ 3: the values [4, 3, 4])

---

### Example 4: Array with Large Values

**Input**: `nums = [3, 6, 7, 7, 0]`

**Step 1: Build Frequency Map**
- Array length: `len = 5`
- Process each element (values > len are capped):
  - `3.min(5) = 3` → freq[3] = 1
  - `6.min(5) = 5` → freq[5] = 1
  - `7.min(5) = 5` → freq[5] = 2
  - `7.min(5) = 5` → freq[5] = 3
  - `0.min(5) = 0` → freq[0] = 1

**Frequency Map**: `{0: 1, 3: 1, 5: 3}`

**Step 2: Iterate Backwards from len to 1**

| i | freq[i] | count (cumulative) | count == i? | Action |
|---|---------|-------------------|-------------|--------|
| 5 | 3       | 0 + 3 = 3         | 3 != 5 ✗    | Continue |
| 4 | 0       | 3 + 0 = 3         | 3 != 4 ✗    | Continue |
| 3 | 1       | 3 + 1 = 4         | 4 != 3 ✗    | Continue |
| 2 | 0       | 4 + 0 = 4         | 4 != 2 ✗    | Continue |
| 1 | 0       | 4 + 0 = 4         | 4 != 1 ✗    | Continue |

**Result**: `-1` (no valid special value exists)

---

### Key Insights

1. **Capping at len**: Values greater than the array length are capped because if we have a number larger than `len`, it behaves the same as `len` for our counting purposes

2. **Reverse iteration**: We iterate backwards to accumulate counts efficiently. As we move from `len` down to `1`, we're counting all elements ≥ current value

3. **Unique solution**: If a special value exists, it's guaranteed to be unique. The algorithm finds it in O(n) time and O(n) space

4. **Early termination**: The algorithm returns immediately when it finds `count == i`, avoiding unnecessary iterations
