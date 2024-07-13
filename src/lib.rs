//! Safe Rust bindings for the SoPlex linear programming solver.
//!
//! # Example
//! ```
//! use soplex_rs::*;
//!
//! let mut lp = Model::new();
//!  lp.add_col([], 1.0, 0.0, 5.0);
//!  lp.add_col([], 1.0, 0.0, 5.0);
//!  lp.add_row([1.0, 1.0], 1.0, 5.0);
//!  let result = lp.optimize();
//!  assert_eq!(result, Status::Optimal);
//!  assert!((lp.obj_val() - 5.0).abs() < 1e-6);
//! ```

#![deny(missing_docs)]

mod status;

pub use status::*;

/// Re-export of the raw FFI bindings.
pub mod ffi {
    pub use soplex_sys::*;
}

mod model;

pub use model::*;