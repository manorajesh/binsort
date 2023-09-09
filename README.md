# SearchSort

A Rust trait implementing a [Binary Search]() and the [Quick Sort]() algorithm. Currently, they are implemented for
`Vec<T>` types but can be expanded to other collection types.

## Usage
```shell
cargo add searchsort
```

## Examples

```rust
use searchsort::SearchSort;

let arr = vec![4, 82, 4, 32, 3, 20, 3, 2, 2, 9, 8, 7, 5, 0];
let find = 5;

assert_eq!(arr.find_me(find, 0, arr.len()-1), Some(12));

let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
arr.quicksort();
assert_eq!(arr, [1, 1, 2, 3, 4, 5, 5, 6, 9]);
```

## Why
This was my first time implementing the Binary Search and Quick Sort algorithms. I have a feeling this may not be the
last time.