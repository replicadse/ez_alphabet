# EZ Alphabet

An implementation of an easy to work with alphabet.

## Examples

```rust
// Following example creates 4 strings, encoding the numbers 4 to 8 (start: 4, count: 5) in the given alphabet.
assert_eq!(Alphabet::from("abcdef").unwrap().generate(4, 5), vec!["e", "f", "aa", "ab", "ac"]);
```
