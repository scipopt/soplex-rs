# soplex-rs

Safe rust bindings for the SoPlex linear programming library.

## Usage

```rust
use soplex_rs::{SoPlex, Status};

let mut lp = SoPlex::new();
lp.add_col([], 1.0, 0.0, 5.0);
lp.add_col([], 1.0, 0.0, 5.0);
lp.add_row([1.0, 1.0], 1.0, 5.0);
let result = lp.optimize();
assert_eq!(result, Status::Optimal);
```