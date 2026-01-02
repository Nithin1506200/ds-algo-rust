# hashmap

A HashMap is a collection that stores key-value pairs. It provides O(1) average time complexity for insertions, lookups, and deletions.

## Quick Reference

| Operation | Method | Description |
|-----------|--------|-------------|
| [Create](#initialization) | `HashMap::new()` | Create empty HashMap |
| [Create from array](#creating-from-an-arrayiterator) | `HashMap::from([("a", 1)])` | Create from array |
| [Insert](#insert) | `map.insert(key, value)` | Insert/overwrite key-value pair, returns old value |
| [Get](#get) | `map.get(&key)` | Get reference to value, returns `Option<&V>` |
| [Get mutable](#get-mutable-reference) | `map.get_mut(&key)` | Get mutable reference, returns `Option<&mut V>` |
| [Entry](#entry-api) | `map.entry(key).or_insert(default)` | Get or insert default value |
| [Entry update](#updating-values-with-entry) | `*map.entry(key).or_insert(0) += 1` | Increment counter pattern |
| [Contains](#check-if-key-exists) | `map.contains_key(&key)` | Check if key exists |
| [Remove](#remove-values) | `map.remove(&key)` | Remove key-value pair, returns `Option<V>` |
| [Length](#length-and-clearing) | `map.len()` | Get number of key-value pairs |
| [Clear](#length-and-clearing) | `map.clear()` | Remove all entries |
| [Iterate](#iterate-over-hashmap) | `for (k, v) in &map { }` | Iterate over key-value pairs |

---

## Initialization

> **API Reference**: [HashMap::new](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.new) | [HashMap::from](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.from) | [HashMap::with_capacity](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.with_capacity)

### Creating an empty HashMap

```rust
use std::collections::HashMap;

// Create an empty HashMap
let mut map: HashMap<String, i32> = HashMap::new();

// Create with type inference
let mut scores = HashMap::new();
scores.insert("Blue", 10);  // Type inferred as HashMap<&str, i32>
```

### Creating from an array/iterator

```rust
use std::collections::HashMap;

// From array of tuples
let map: HashMap<&str, i32> = HashMap::from([
    ("apple", 3),
    ("banana", 2),
    ("orange", 5),
]);

// Using collect from iterator
let teams = vec!["Blue", "Yellow"];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

### Creating with capacity

```rust
use std::collections::HashMap;

// Pre-allocate capacity for better performance
let mut map: HashMap<String, i32> = HashMap::with_capacity(100);
```

---

## Insert

> **API Reference**: [HashMap::insert](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert)

### Basic insert

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// Insert returns Option<V> - the old value if key existed
map.insert("apple", 3);      // Returns None (key didn't exist)
map.insert("banana", 2);     // Returns None
let old = map.insert("apple", 5);  // Returns Some(3) (overwrites old value)

println!("{:?}", old);  // Some(3)
println!("{:?}", map);  // {"apple": 5, "banana": 2}
```

---

## Get

> **API Reference**: [HashMap::get](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get) | [HashMap::get_mut](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_mut)

### Get with Option

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);
map.insert("banana", 2);

// get returns Option<&V>
match map.get("apple") {
    Some(value) => println!("Apple count: {}", value),  // Apple count: 3
    None => println!("Not found"),
}

// Using if let
if let Some(count) = map.get("banana") {
    println!("Banana count: {}", count);  // Banana count: 2
}

// Direct unwrap (use carefully!)
let apple_count = map.get("apple").unwrap();  // 3
```

### Get with default value

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);

// get_or_insert returns a mutable reference
let count = map.get("orange").unwrap_or(&0);
println!("{}", count);  // 0 (returns default, doesn't insert)

// copied() to get the value instead of reference
let value = map.get("apple").copied().unwrap_or(0);
println!("{}", value);  // 3
```

### Get mutable reference

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);

// get_mut returns Option<&mut V>
if let Some(count) = map.get_mut("apple") {
    *count += 10;  // Modify the value in place
}

println!("{:?}", map);  // {"apple": 13}
```

---

## Entry API

> **API Reference**: [HashMap::entry](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) | [Entry::or_insert](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert) | [Entry::or_insert_with](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert_with) | [Entry::and_modify](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.and_modify)

The Entry API is the most efficient way to work with HashMap when you need to insert or update values.

### or_insert - Insert if key doesn't exist

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// Insert only if key doesn't exist
map.entry("apple").or_insert(5);
map.entry("apple").or_insert(10);  // Does nothing, key exists

println!("{:?}", map);  // {"apple": 5}
```

### or_insert_with - Lazy initialization

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// Only calls the closure if key doesn't exist
map.entry("expensive").or_insert_with(|| {
    println!("Computing expensive value...");
    42
});

map.entry("expensive").or_insert_with(|| {
    println!("This won't print");
    100
});
```

### Updating values with entry

```rust
use std::collections::HashMap;

let text = "hello world hello";
let mut word_count = HashMap::new();

// Count word frequencies
for word in text.split_whitespace() {
    *word_count.entry(word).or_insert(0) += 1;
}

println!("{:?}", word_count);  // {"hello": 2, "world": 1}
```

### and_modify - Modify existing entry

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);

// Modify if exists, insert if not
map.entry("apple")
    .and_modify(|count| *count += 1)
    .or_insert(1);

map.entry("banana")
    .and_modify(|count| *count += 1)
    .or_insert(1);

println!("{:?}", map);  // {"apple": 4, "banana": 1}
```

---

## Common Patterns

### Comparing insert vs entry (from original example)

```rust
use std::collections::HashMap;

let mut freq = HashMap::new();
let nums = vec![1, 2, 1, 3, 2, 1];

// Method 1: Using get and insert (verbose)
for &num in &nums {
    freq.insert(num, *freq.get(&num).unwrap_or(&0) + 1);
}

// Method 2: Using entry API (preferred)
freq.clear();
for &num in &nums {
    *freq.entry(num).or_insert(0) += 1;
}

println!("{:?}", freq);  // {1: 3, 2: 2, 3: 1}
```

### Check if key exists

> **API Reference**: [HashMap::contains_key](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.contains_key)

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);

// Using contains_key
if map.contains_key("apple") {
    println!("Found apple!");
}

// Using get with is_some()
if map.get("banana").is_some() {
    println!("Found banana!");
} else {
    println!("Banana not found");
}
```

### Remove values

> **API Reference**: [HashMap::remove](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.remove)

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);
map.insert("banana", 2);

// remove returns Option<V>
let removed = map.remove("apple");
println!("{:?}", removed);  // Some(3)

// Remove non-existent key
let removed = map.remove("orange");
println!("{:?}", removed);  // None

println!("{:?}", map);  // {"banana": 2}
```

### Iterate over HashMap

> **API Reference**: [HashMap::keys](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.keys) | [HashMap::values](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.values) | [HashMap::iter](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter) | [HashMap::iter_mut](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter_mut)

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);
map.insert("banana", 2);
map.insert("orange", 5);

// Iterate over key-value pairs
for (key, value) in &map {
    println!("{}: {}", key, value);
}

// Iterate over keys only
for key in map.keys() {
    println!("Key: {}", key);
}

// Iterate over values only
for value in map.values() {
    println!("Value: {}", value);
}

// Mutable iteration
for (key, value) in &mut map {
    *value *= 2;  // Double all values
}
println!("{:?}", map);
```

### Length and clearing

> **API Reference**: [HashMap::len](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.len) | [HashMap::is_empty](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.is_empty) | [HashMap::clear](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.clear)

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);
map.insert("banana", 2);

println!("Length: {}", map.len());  // 2
println!("Is empty: {}", map.is_empty());  // false

map.clear();
println!("After clear - Length: {}", map.len());  // 0
println!("Is empty: {}", map.is_empty());  // true
```

---
