This repository will be mine place where i try to write a cool, (really cool ) math parser,
with `functions` and `x` 

How'd I see this work

```rust
assert_eq!(f!("x + 2", 1), 3);
assert_eq!(f!("(2 + 2) * 2"), 8);
assert_eq!(f!("(2 + x) * 2" => 2), 8);
```

so here we can see how'd I see this work
