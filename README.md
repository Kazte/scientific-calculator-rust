# Scientific Calculator

Scientific calculator made in Rust.

## Regex

Addition regex:

```rust
Regex::new(r"(\d+)\s?\+\s?(\d+)")
```

- Add Regex -> (\d+) \s? \+ \s? (\d+)
  - (\d+) -> One or more digits
  - \s? -> Space (optional)
  - \+ -> add symbol

## Todo

- [x] Basic operations
- [ ] Abstract operations to method
- [ ] Add parenthesis
- [ ] Add more functionality:
  - [ ] Exponential
  - [ ] Logarithms
  - [ ] Sin, Cos, Tg

<hr/>

Created for learning purpose.
