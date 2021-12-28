# any-terminal-size

[Documention](https://docs.rs/crate/any_terminal_size)

Rust library to getting the size of a terminal of another process.

```rust
use any_terminal_size::{any_terminal_size, Height, Width};

let size = any_terminal_size();
if let Some((Width(w), Height(h))) = size {
    println!("The terminal size of your process or [transitive] parent process is {} cols wide and {} lines tall.", w, h);
} else {
    println!("Unable to get terminal the size.");
}
```

## Minimum Rust Version

This crate requires a minimum rust version of 1.31.0 (2018-12-06)

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
