# soplex-rs
[![][img_crates]][crates] [![][img_doc]][doc] 

[img_crates]: https://img.shields.io/crates/v/soplex-rs.svg
[crates]: https://crates.io/crates/soplex-rs
[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg
[doc]: https://docs.rs/soplex-rs/

Safe rust bindings for the SoPlex linear programming library.

## Usage

```rust
use soplex_rs::*;

let mut lp = Model::new();
let col1= lp.add_col([], 1.0, 0.0, 5.0);
let col2 = lp.add_col([], 1.0, 0.0, 5.0);
let row = lp.add_row([1.0, 1.0], 1.0, 5.0);
assert_eq!(lp.num_cols(), 2);
assert_eq!(lp.num_rows(), 1);

let lp = lp.optimize();
let result = lp.status();
assert_eq!(result, Status::Optimal);
assert!((lp.obj_val() - 5.0).abs() < 1e-6);

let mut lp = Model::from(lp); // Convert the solved model back to a mutable one
lp.remove_row(row);
assert_eq!(lp.num_rows(), 0);
let lp = lp.optimize();
let new_result = lp.status();
assert_eq!(new_result, Status::Optimal);
assert!((lp.obj_val() - 10.0).abs() < 1e-6);

let mut lp = Model::from(lp);
lp.remove_col(col1);
assert_eq!(lp.num_cols(), 1);
let lp = lp.optimize();
let new_result = lp.status();
assert_eq!(new_result, Status::Optimal);
assert!((lp.obj_val() - 5.0).abs() < 1e-6);
```
