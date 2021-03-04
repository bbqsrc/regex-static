# regex_static

Compile-time validation of [`regex::Regex`](https://github.com/rust-lang/regex).

## Examples

### Lazy regex

Uses `once_cell` to lazily create the regex.

```rust
static RE: Lazy<Regex> = regex_static::lazy_regex!("^yesss$");
```

### Static regex

Also uses `once_cell`, but works inline (will therefore reuse the same instance of the regex each function call).

```rust
let some_regex = regex_static::static_regex!("^yesss$");
```

### Ordinary regex

Will create an owned `Regex`, just like calling `Regex::new(...)` but with compile-time validation.

```rust
let ordinary_regex = regex_static::regex!("^yesss$");
```
