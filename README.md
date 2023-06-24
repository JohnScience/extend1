# Stable `Extend::extend_one` alternative

[![crates.io](https://img.shields.io/crates/v/extend1.svg)][`extend1`]
[![crates.io](https://img.shields.io/crates/d/extend1.svg)][`extend1`]

Alternative to [`Extend::extend_one`] that is stable and does not require `#![feature(extend_one)]`.

[`Extend::extend_one`]: https://doc.rust-lang.org/core/iter/trait.Extend.html#method.extend_one
[`extend1`]: https://crates.io/crates/extend1

## Example

```rust
use proc_macro2 as pm2;
use extend1::Extend1;

let mut ts = pm2::TokenStream::new();
ts.extend1(pm2::Literal::string("Hello, world!"));
assert_eq!(ts.to_string(), "\"Hello, world!\"");
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
