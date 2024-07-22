//! Safe Rust bindings for the SoPlex linear programming solver.
//!
//! # Example
//! ```
//! use soplex_rs::*;
//!
//! let mut lp = Model::new();
//! let col1= lp.add_col(vec![], 1.0, 0.0, 5.0);
//! let col2 = lp.add_col(vec![], 1.0, 0.0, 5.0);
//! let row = lp.add_row(vec![1.0, 1.0], 1.0, 5.0);
//! assert_eq!(lp.num_cols(), 2);
//! assert_eq!(lp.num_rows(), 1);
//!
//! let lp = lp.optimize();
//! let result = lp.status();
//! assert_eq!(result, Status::Optimal);
//! assert!((lp.obj_val() - 5.0).abs() < 1e-6);
//!
//! let mut lp = Model::from(lp); // Convert the solved model back to a mutable one
//! lp.remove_row(row);
//! assert_eq!(lp.num_rows(), 0);
//! let lp = lp.optimize();
//! let new_result = lp.status();
//! assert_eq!(new_result, Status::Optimal);
//! assert!((lp.obj_val() - 10.0).abs() < 1e-6);
//!
//! let mut lp = Model::from(lp);
//! lp.remove_col(col1);
//! assert_eq!(lp.num_cols(), 1);
//! let lp = lp.optimize();
//! let new_result = lp.status();
//! assert_eq!(new_result, Status::Optimal);
//! assert!((lp.obj_val() - 5.0).abs() < 1e-6);
//! ```

#![deny(missing_docs)]

mod status;

pub use status::*;

/// Re-export of the raw FFI bindings.
pub mod ffi {
    pub use soplex_sys::*;
}

mod basis_status;
mod model;
mod param;
mod soplex_ptr;
pub use basis_status::*;

pub use param::*;

pub use model::*;
