# soplex-rs
[![][img_crates]][crates] [![][img_doc]][doc] 

[img_crates]: https://img.shields.io/crates/v/soplex-rs.svg
[crates]: https://crates.io/crates/soplex-rs
[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg
[doc]: https://docs.rs/soplex-rs/

Safe rust bindings for the SoPlex linear programming library.

## Usage

```rust
use soplex_rs::{Model, Status};

let mut lp = Model::new();
lp.add_col([], 1.0, 0.0, 5.0);
lp.add_col([], 1.0, 0.0, 5.0);
lp.add_row([1.0, 1.0], 1.0, 5.0);
let result = lp.optimize();
assert_eq!(result, Status::Optimal);
```
