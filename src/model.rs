use std::ffi::c_void;
use crate::ffi;
use crate::status::Status;

/// A linear programming model.
pub struct Model {
    inner: *mut c_void,
}

pub struct RowId(usize);
pub struct ColId(usize);

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
    pub fn add_col<const N: usize>(&mut self, mut colentries: [f64; N], objval: f64, lb: f64, ub: f64) -> ColId {
        let nnonzeros = colentries.iter().filter(|&&x| x != 0.0).count();
        let colsize = colentries.len();

        unsafe {
            ffi::SoPlex_addColReal(self.inner,
                                   colentries.as_mut_ptr(),
                                   colsize as i32,
                                   nnonzeros as i32,
                                   objval, lb, ub);
        }

        ColId(self.num_cols() - 1)
    }

    /// Adds a row to the model.
    ///
    /// # Arguments
    ///
    /// * `rowentries` - An array of f64 representing the row entries.
    /// * `lhs` - The left-hand side of the row.
    /// * `rhs` - The right-hand side of the row.
    pub fn add_row<const N: usize>(&mut self, mut rowentries: [f64; N], lhs: f64, rhs: f64) -> RowId {
        let nnonzeros = rowentries.iter().filter(|&&x| x != 0.0).count();
        let rowsize = rowentries.len();

        unsafe {
            ffi::SoPlex_addRowReal(self.inner,
                                   rowentries.as_mut_ptr(),
                                   rowsize as i32,
                                   nnonzeros as i32,
                                   lhs, rhs);
        }

        RowId(self.num_rows() - 1)
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

    /// Returns the number of columns in the model.
    pub fn num_cols(&self) -> usize {
        unsafe { ffi::SoPlex_numCols(self.inner) as usize}
    }

    /// Returns the number of rows in the model.
    pub fn num_rows(&self) -> usize {
        unsafe { ffi::SoPlex_numRows(self.inner) as usize }
    }

    /// Remove a column from the model.
    pub fn remove_col(&mut self, col_id: ColId) {
        unsafe { ffi::SoPlex_removeColReal(self.inner, col_id.0 as i32) };
    }


    /// Remove a row from the model.
    pub fn remove_row(&mut self, row_id: RowId) {
        unsafe { ffi::SoPlex_removeRowReal(self.inner, row_id.0 as i32) };
    }

    /// Returns the solving time of the model in seconds.
    pub fn solving_time(&self) -> f64 {
        unsafe { ffi::SoPlex_getSolvingTime(self.inner) }
    }

    /// Returns the primal solution of the model.
    pub fn primal_solution(&self) -> Vec<f64> {
        let mut primal = vec![0.0; self.num_cols()];
        unsafe {
            ffi::SoPlex_getPrimalReal(self.inner, primal.as_mut_ptr(), self.num_cols() as i32);
        }
        primal
    }

    /// Returns the dual solution of the model.
    pub fn dual_solution(&self) -> Vec<f64> {
        let mut dual = vec![0.0; self.num_rows()];
        unsafe {
            ffi::SoPlex_getDualReal(self.inner, dual.as_mut_ptr(), self.num_rows() as i32);
        }
        dual
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
        let col1= lp.add_col([], 1.0, 0.0, 5.0);
        let _col2 = lp.add_col([], 1.0, 0.0, 10.0);
        let row = lp.add_row([1.0, 1.0], 1.0, 5.0);
        assert_eq!(lp.num_cols(), 2);
        assert_eq!(lp.num_rows(), 1);

        let result = lp.optimize();
        assert_eq!(result, Status::Optimal);
        assert!((lp.obj_val() - 5.0).abs() < 1e-6);
        let dual_sol = lp.dual_solution();
        assert_eq!(dual_sol.len(), 1);
        assert!((dual_sol[0] - 1.0).abs() < 1e-6);

        lp.remove_row(row);
        assert_eq!(lp.num_rows(), 0);
        let new_result = lp.optimize();
        assert_eq!(new_result, Status::Optimal);
        assert!((lp.obj_val() - 15.0).abs() < 1e-6);
        let primal_sol = lp.primal_solution();
        assert_eq!(primal_sol.len(), 2);
        assert!((primal_sol[0] - 5.0).abs() < 1e-6);
        assert!((primal_sol[1] - 10.0).abs() < 1e-6);

        lp.remove_col(col1);
        assert_eq!(lp.num_cols(), 1);
        let new_result = lp.optimize();
        assert_eq!(new_result, Status::Optimal);
        assert!((lp.obj_val() - 10.0).abs() < 1e-6);

        assert!(lp.solving_time() >= 0.0);
    }
}
