use std::ffi::c_void;
use crate::ffi;
use crate::status::Status;

/// A linear programming model.
pub struct Model {
    inner: *mut c_void,
}

impl Model {
    /// Creates a new linear programming model.
    pub fn new() -> Self {
        let inner = unsafe { ffi::SoPlex_create() };
        assert!(!inner.is_null());
        Self { inner }
    }

    /// Adds a column to the model.
    ///
    /// # Arguments
    ///
    /// * `colentries` - An array of f64 representing the column entries.
    /// * `objval` - The objective value of the column.
    /// * `lb` - The lower bound of the column.
    /// * `ub` - The upper bound of the column.
    pub fn add_col<const N: usize>(&mut self, mut colentries: [f64; N], objval: f64, lb: f64, ub: f64) {
        let nnonzeros = colentries.iter().filter(|&&x| x != 0.0).count();
        let colsize = colentries.len();

        unsafe {
            ffi::SoPlex_addColReal(self.inner,
                                   colentries.as_mut_ptr(),
                                   colsize as i32,
                                   nnonzeros as i32,
                                   objval, lb, ub);
        }
    }

    /// Adds a row to the model.
    ///
    /// # Arguments
    ///
    /// * `rowentries` - An array of f64 representing the row entries.
    /// * `lhs` - The left-hand side of the row.
    /// * `rhs` - The right-hand side of the row.
    pub fn add_row<const N: usize>(&mut self, mut rowentries: [f64; N], lhs: f64, rhs: f64) {
        let nnonzeros = rowentries.iter().filter(|&&x| x != 0.0).count();
        let rowsize = rowentries.len();

        unsafe {
            ffi::SoPlex_addRowReal(self.inner,
                                   rowentries.as_mut_ptr(),
                                   rowsize as i32,
                                   nnonzeros as i32,
                                   lhs, rhs);
        }
    }

    /// Optimizes the model and returns the `Status`.
    pub fn optimize(&mut self) -> Status {
        unsafe { ffi::SoPlex_optimize(self.inner) }.into()
    }

    /// Returns the `Status` of the model.
    pub fn status(&self) -> Status {
        unsafe { ffi::SoPlex_getStatus(self.inner) }.into()
    }

    /// Returns the objective value of the model.
    pub fn obj_val(&self) -> f64 {
        unsafe { ffi::SoPlex_objValueReal(self.inner) }
    }
}

impl Drop for Model {
    /// Frees the memory allocated for the model.
    fn drop(&mut self) {
        unsafe { ffi::SoPlex_free(self.inner) };
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_problem() {
        let mut lp = Model::new();
        lp.add_col([], 1.0, 0.0, 5.0);
        lp.add_col([], 1.0, 0.0, 5.0);
        lp.add_row([1.0, 1.0], 1.0, 5.0);
        let result = lp.optimize();
        assert_eq!(result, Status::Optimal);
        assert!((lp.obj_val() - 5.0).abs() < 1e-6);
    }
}
