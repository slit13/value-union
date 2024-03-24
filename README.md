# Value Union

A proof-of-concept for my idea of a "value union", where a single variable can hold multiple values
of the same type.

It's distinct from a list in that the value of `a: list<int>` is not the integers themselves but
the list holding the integers. On the other hand, `a: value union<int>` does hold the integer values
directly and acts as if it were an integer, not a list.

See [](./value_union.md) for a more professional explanation.

## Proof-of-concept

A Rust proof-of-concept is used.

The primary elements are:
1. `ValueUnion`: the type itself, a container for an underlying type
2. `unite!`: a macro that simplifies the creation of a value union

Example:
```rust
// create a value union from the values 1, 2, 3
// they're passed as a slice, the slice itself isn't used but its elements are
// this is simply a limitation of how many From traits are implemented
let a = ValueUnion::from([1, 2, 3]);

// create a value union using the unite! macro (doesn't require a slice)
let b = unite!(1, 2, 3);

// equality between value unions
// todo: strict vs loose equality differentiation
//       strict equality would require an exact match of values
//       loose equality would only require one shared value
assert_eq!(a, b); // => true

// equality between value union and singular values
assert_eq!(a, 1); // => true
assert_eq!(a, 2); // => true
assert_eq!(a, 3); // => true
assert_eq!(a, 4); // => false
```
