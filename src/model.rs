use crate::{BoolParam, ColBasisStatus, ffi, IntParam, ObjSense, RealParam, RowBasisStatus};
use crate::param::{ALGORITHM_PARAM_ID, OBJSENSE_PARAM_ID, REPR_PARAM_ID};
use crate::soplex_ptr::SoplexPtr;
use crate::status::Status;

/// A linear programming model.
pub struct Model {
    inner: SoplexPtr,
}

/// Id of a row in the model.
pub struct RowId(usize);

/// Id of a column in the model.
pub struct ColId(usize);

impl Model {
    /// Creates a new linear programming model.
    pub fn new() -> Self {
        Self { inner: SoplexPtr::new() }
    }

    /// Adds a column to the model.
    ///
    /// # Arguments
    ///
    /// * `colentries` - An array of f64 representing the column entries.
    /// * `objval` - The objective value of the column.
    /// * `lb` - The lower bound of the column.
    /// * `ub` - The upper bound of the column.
    ///
    /// # Returns
    ///
    /// The `ColId` of the added column.
    pub fn add_col<const N: usize>(&mut self, mut colentries: [f64; N], objval: f64, lb: f64, ub: f64) -> ColId {
        let nnonzeros = colentries.iter().filter(|&&x| x != 0.0).count();
        let colsize = colentries.len();

        unsafe {
            ffi::SoPlex_addColReal(*self.inner,
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
    ///
    /// # Returns
    ///
    /// The `RowId` of the added row.
    pub fn add_row<const N: usize>(&mut self, mut rowentries: [f64; N], lhs: f64, rhs: f64) -> RowId {
        let nnonzeros = rowentries.iter().filter(|&&x| x != 0.0).count();
        let rowsize = rowentries.len();

        unsafe {
            ffi::SoPlex_addRowReal(*self.inner,
                                   rowentries.as_mut_ptr(),
                                   rowsize as i32,
                                   nnonzeros as i32,
                                   lhs, rhs);
        }

        RowId(self.num_rows() - 1)
    }

    /// Optimizes the model and returns the solved model.
    pub fn optimize(self) -> SolvedModel {
        unsafe { ffi::SoPlex_optimize(*self.inner) };
        SolvedModel { inner: self.inner }
    }

    /// Returns the number of columns in the model.
    pub fn num_cols(&self) -> usize {
        unsafe { ffi::SoPlex_numCols(*self.inner) as usize }
    }

    /// Returns the number of rows in the model.
    pub fn num_rows(&self) -> usize {
        unsafe { ffi::SoPlex_numRows(*self.inner) as usize }
    }

    /// Remove a column from the model.
    pub fn remove_col(&mut self, col_id: ColId) {
        unsafe { ffi::SoPlex_removeColReal(*self.inner, col_id.0 as i32) };
    }


    /// Remove a row from the model.
    pub fn remove_row(&mut self, row_id: RowId) {
        unsafe { ffi::SoPlex_removeRowReal(*self.inner, row_id.0 as i32) };
    }

    /// Read instance from lp/mps file.
    ///
    /// # Arguments
    /// * `filename` - The name of the lp/mps file to read from.
    ///
    /// # Panics
    /// if the file does not exist or the file is not in the correct format.
    pub fn read_file(&mut self, filename: &str) {
        if !std::path::Path::new(filename).exists() {
            panic!("File does not exist");
        }

        if !filename.ends_with(".lp") && !filename.ends_with(".mps") {
            panic!("File is not in the correct format, must be .lp or .mps");
        }

        let c_filename = std::ffi::CString::new(filename).unwrap();
        unsafe {
            ffi::SoPlex_readInstanceFile(*self.inner, c_filename.as_ptr());
        }
    }

    /// Sets boolean parameter.
    ///
    /// # Arguments
    /// * `param` - which `BoolParam` to set.
    /// * `value` - The value of the parameter.
    pub fn set_bool_param(&mut self, param: BoolParam, value: bool) {
        unsafe {
            ffi::SoPlex_setBoolParam(*self.inner, param.into(), value as i32);
        }
    }

    /// Sets integer parameter.
    ///
    /// # Arguments
    /// * `param` - which `IntParam` to set.
    /// * `value` - The value of the parameter.
    pub fn set_int_param(&mut self, param: IntParam, value: i32) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, param.into(), value);
        }
    }

    /// Sets real parameter.
    ///
    /// # Arguments
    /// * `param` - which `RealParam` to set.
    /// * `value` - The value of the parameter.
    pub fn set_real_param(&mut self, param: RealParam, value: f64) {
        unsafe {
            ffi::SoPlex_setRealParam(*self.inner, param.into(), value);
        }
    }


    /// Change the bounds of a column.
    ///
    /// # Arguments
    /// * `col_id` - The `ColId` of the column to change.
    /// * `lb` - The new lower bound of the column.
    /// * `ub` - The new upper bound of the column.
    pub fn change_col_bounds(&mut self, col_id: ColId, lb: f64, ub: f64) {
        unsafe {
            ffi::SoPlex_changeVarBoundsReal(*self.inner, col_id.0 as i32, lb, ub);
        }
    }

    /// Change the range (bounds) of a row.
    ///
    /// # Arguments
    /// * `row_id` - The `RowId` of the row to change.
    /// * `lhs` - The new left-hand side of the row.
    /// * `rhs` - The new right-hand side of the row.
    pub fn change_row_range(&mut self, row_id: RowId, lhs: f64, rhs: f64) {
        unsafe {
            ffi::SoPlex_changeRowRangeReal(*self.inner, row_id.0 as i32, lhs, rhs);
        }
    }

    /// Sets the objective sense of the model.
    ///
    /// # Arguments
    /// * `sense` - The objective sense of the model.
    pub fn set_obj_sense(&mut self, sense: ObjSense) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, OBJSENSE_PARAM_ID, sense.into());
        }
    }

    /// Sets the algorithm to use.
    ///
    /// # Arguments
    /// * `algorithm` - The `Algorithm` to use.
    pub fn set_algorithm(&mut self, algorithm: crate::Algorithm) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, ALGORITHM_PARAM_ID, algorithm.into());
        }
    }

    /// Sets the representation of the model.
    ///
    /// # Arguments
    /// * `representation` - The `Representation` of the model.
    pub fn set_representation(&mut self, representation: crate::Representation) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, REPR_PARAM_ID, representation.into());
        }
    }

    /// Sets the verbosity level.
    ///
    /// # Arguments
    /// * `verbosity` - The verbosity level.
    pub fn set_verbosity(&mut self, verbosity: i32) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::VERBOSITY_PARAM_ID, verbosity);
        }
    }


    /// Sets the factor update type.
    ///
    /// # Arguments
    /// * `factor_update_type` - The factor update type.
    pub fn set_factor_update_type(&mut self, factor_update_type: crate::FactorUpdateType) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::FACTOR_UPDATE_TYPE_PARAM_ID, factor_update_type.into());
        }
    }

    /// Sets the simplifier type.
    ///
    /// # Arguments
    /// * `simplifier_type` - The simplifier type.
    pub fn set_simplifier_type(&mut self, simplifier_type: crate::Simplifier) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::SIMPLIFIER_PARAM_ID, simplifier_type.into());
        }
    }

    /// Sets the starter type.
    ///
    /// # Arguments
    /// * `starter_type` - The starter type.
    pub fn set_starter_type(&mut self, starter_type: crate::Starter) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::STARTER_PARAM_ID, starter_type.into());
        }
    }

    /// Sets the pricer type.
    ///
    /// # Arguments
    /// * `pricer_type` - The pricer type.
    pub fn set_pricer_type(&mut self, pricer_type: crate::Pricer) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::PRICER_PARAM_ID, pricer_type.into());
        }
    }

    /// Sets the ratio tester type.
    ///
    /// # Arguments
    /// * `ratio_tester_type` - The ratio tester type.
    pub fn set_ratio_tester_type(&mut self, ratio_tester_type: crate::RatioTester) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::RATIO_TESTER_PARAM_ID, ratio_tester_type.into());
        }
    }

    /// Sets the sync mode.
    ///
    /// # Arguments
    /// * `sync_mode` - The sync mode.
    pub fn set_sync_mode(&mut self, sync_mode: crate::SyncMode) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::SYNC_MODE_PARAM_ID, sync_mode.into());
        }
    }


    /// Sets the read mode.
    ///
    /// # Arguments
    /// * `read_mode` - The read mode.
    pub fn set_read_mode(&mut self, read_mode: crate::ReadMode) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::READ_MODE_PARAM_ID, read_mode.into());
        }
    }

    /// Sets the solve mode.
    ///
    /// # Arguments
    /// * `solve_mode` - The solve mode.
    pub fn set_solve_mode(&mut self, solve_mode: crate::SolveMode) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::SOLVE_MODE_PARAM_ID, solve_mode.into());
        }
    }

    /// Sets the check mode.
    ///
    /// # Arguments
    /// * `check_mode` - The check mode.
    pub fn set_check_mode(&mut self, check_mode: crate::CheckMode) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::CHECK_MODE_PARAM_ID, check_mode.into());
        }
    }

    /// Sets the timer mode.
    ///
    /// # Arguments
    /// * `timer_mode` - The timer mode.
    pub fn set_timer_mode(&mut self, timer_mode: crate::Timer) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::TIMER_PARAM_ID, timer_mode.into());
        }
    }

    /// Sets the hyper pricing parameter.
    ///
    /// # Arguments
    /// * `hyper_pricing` - The hyper pricing parameter.
    pub fn set_hyper_pricing(&mut self, hyper_pricing: crate::HyperPricing) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::HYPER_PRICING_PARAM_ID, hyper_pricing.into());
        }
    }

    /// Sets the solution polishing type.
    ///
    /// # Arguments
    /// * `solution_polishing` - The solution polishing type.
    pub fn set_solution_polishing(&mut self, solution_polishing: crate::SolutionPolishing) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::SOLUTION_POLISHING_PARAM_ID, solution_polishing.into());
        }
    }

    /// Sets the decomposition verbosity.
    ///
    /// # Arguments
    /// * `decomp_verbosity` - The decomposition verbosity.
    pub fn set_decomp_verbosity(&mut self, decomp_verbosity: crate::Verbosity) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::DECOMP_VERBOSITY_PARAM_ID, decomp_verbosity.into());
        }
    }

    /// Sets the statistics timer parameter.
    ///
    /// # Arguments
    /// * `stat_timer` - The statistics timer parameter.
    pub fn set_stat_timer(&mut self, stat_timer: crate::Timer) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::STAT_TIMER_PARAM_ID, stat_timer.into());
        }
    }

    /// Sets the scalar type.
    ///
    /// # Arguments
    /// * `scalar_type` - The scalar type.
    pub fn set_scalar_type(&mut self, scalar_type: crate::Scalar) {
        unsafe {
            ffi::SoPlex_setIntParam(*self.inner, crate::SCALAR_PARAM_ID, scalar_type.into());
        }
    }
}

/// A solved linear programming model.
pub struct SolvedModel {
    inner: SoplexPtr,
}

impl SolvedModel {
    /// Returns the number of columns in the model.
    pub fn num_cols(&self) -> usize {
        unsafe { ffi::SoPlex_numCols(*self.inner) as usize }
    }

    /// Returns the number of rows in the model.
    pub fn num_rows(&self) -> usize {
        unsafe { ffi::SoPlex_numRows(*self.inner) as usize }
    }

    /// Returns the `Status` of the model.
    pub fn status(&self) -> Status {
        unsafe { ffi::SoPlex_getStatus(*self.inner) }.into()
    }

    /// Returns the objective value of the model.
    pub fn obj_val(&self) -> f64 {
        unsafe { ffi::SoPlex_objValueReal(*self.inner) }
    }


    /// Returns the primal solution of the model.
    pub fn primal_solution(&self) -> Vec<f64> {
        let mut primal = vec![0.0; self.num_cols()];
        unsafe {
            ffi::SoPlex_getPrimalReal(*self.inner, primal.as_mut_ptr(), self.num_cols() as i32);
        }
        primal
    }

    /// Returns the dual solution of the model.
    pub fn dual_solution(&self) -> Vec<f64> {
        let mut dual = vec![0.0; self.num_rows()];
        unsafe {
            ffi::SoPlex_getDualReal(*self.inner, dual.as_mut_ptr(), self.num_rows() as i32);
        }
        dual
    }

    /// Returns the solving time of the model in seconds.
    pub fn solving_time(&self) -> f64 {
        unsafe { ffi::SoPlex_getSolvingTime(*self.inner) }
    }

    /// Returns the reduced costs of the model.
    pub fn reduced_costs(&self) -> Vec<f64> {
        let mut redcosts = vec![0.0; self.num_cols()];
        unsafe {
            ffi::SoPlex_getRedCostReal(*self.inner, redcosts.as_mut_ptr(), self.num_cols() as i32);
        }
        redcosts
    }

    /// Returns the number of iterations it took to solve the model.
    pub fn num_iterations(&self) -> i32 {
        unsafe { ffi::SoPlex_getNumIterations(*self.inner) }
    }

    /// Returns the basis status of a column.
    ///
    /// # Arguments
    /// * `col_id` - The `ColId` of the column.
    ///
    /// # Returns
    /// The `BasisStatus` of the column.
    pub fn col_basis_status(&self, col_id: ColId) -> ColBasisStatus {
        unsafe { ffi::SoPlex_basisColStatus(*self.inner, col_id.0 as i32) }.into()
    }

    /// Returns the basis status of a row.
    ///
    /// # Arguments
    /// * `row_id` - The `RowId` of the row.
    ///
    /// # Returns
    /// The `BasisStatus` of the row.
    pub fn row_basis_status(&self, row_id: RowId) -> RowBasisStatus {
        unsafe { ffi::SoPlex_basisRowStatus(*self.inner, row_id.0 as i32) }.into()
    }
}

impl From<SolvedModel> for Model {
    fn from(solved_model: SolvedModel) -> Self {
        Self { inner: solved_model.inner }
    }
}


#[cfg(test)]
mod tests {
    use crate::Algorithm;
    use super::*;

    #[test]
    fn simple_problem() {
        let mut lp = Model::new();
        let col1 = lp.add_col([], 1.0, 0.0, 5.0);
        let _col2 = lp.add_col([], 1.0, 0.0, 10.0);
        let row = lp.add_row([1.0, 1.0], 1.0, 5.0);
        assert_eq!(lp.num_cols(), 2);
        assert_eq!(lp.num_rows(), 1);

        let lp = lp.optimize();
        let result = lp.status();
        assert_eq!(result, Status::Optimal);
        assert!((lp.obj_val() - 5.0).abs() < 1e-6);
        let dual_sol = lp.dual_solution();
        assert_eq!(dual_sol.len(), 1);
        assert!((dual_sol[0] - 1.0).abs() < 1e-6);

        let mut lp = Model::from(lp);
        lp.remove_row(row);
        assert_eq!(lp.num_rows(), 0);
        let lp = lp.optimize();
        let new_result = lp.status();
        assert_eq!(new_result, Status::Optimal);
        assert!((lp.obj_val() - 15.0).abs() < 1e-6);
        let primal_sol = lp.primal_solution();
        assert_eq!(primal_sol.len(), 2);
        assert!((primal_sol[0] - 5.0).abs() < 1e-6);
        assert!((primal_sol[1] - 10.0).abs() < 1e-6);

        let mut lp = Model::from(lp);
        lp.remove_col(col1);
        assert_eq!(lp.num_cols(), 1);
        let lp = lp.optimize();
        let new_result = lp.status();
        assert_eq!(new_result, Status::Optimal);
        assert!((lp.obj_val() - 10.0).abs() < 1e-6);

        assert!(lp.solving_time() >= 0.0);
    }

    #[test]
    fn read_file() {
        let mut lp = Model::new();
        lp.read_file("tests/data/simple.mps");
        let lp = lp.optimize();
        let result = lp.status();
        assert_eq!(result, Status::Optimal);
        assert!((lp.obj_val() - -27.66666666).abs() < 1e-6);
    }


    #[test]
    fn num_iterations() {
        let mut lp = Model::new();
        lp.add_col([], 1.0, 0.0, 5.0);
        lp.add_col([], 1.0, 0.0, 10.0);
        lp.add_row([1.0, 1.0], 1.0, 5.0);
        let lp = lp.optimize();
        let num_iterations = lp.num_iterations();
        assert_eq!(num_iterations, 1);
    }

    #[test]
    fn set_int_param() {
        let mut lp = Model::new();
        lp.set_int_param(IntParam::IterLimit, 0);
        lp.add_col([], 1.0, 0.0, 5.0);
        lp.add_col([], 1.0, 0.0, 10.0);
        lp.add_row([1.0, 1.0], 1.0, 5.0);
        let lp = lp.optimize();
        let num_iterations = lp.num_iterations();
        assert_eq!(num_iterations, 0);
        assert_eq!(lp.status(), Status::AbortIter);
    }

    #[test]
    fn set_real_param() {
        let mut lp = Model::new();
        lp.set_real_param(RealParam::TimeLimit, 0.0);
        lp.add_col([], 1.0, 0.0, 5.0);
        lp.add_col([], 1.0, 0.0, 10.0);
        lp.add_row([1.0, 1.0], 1.0, 5.0);
        let lp = lp.optimize();
        assert_eq!(lp.status(), Status::AbortTime);
    }


    #[test]
    fn set_bool_param() {
        // TODO: think of a better test,
        // this just sets a parameter to true and makes sure that it runs
        // from the output, it seems that the parameter is being set
        let mut lp = Model::new();
        lp.set_bool_param(BoolParam::EqTrans, true);
        lp.add_col([], 1.0, 0.0, 5.0);
        lp.add_col([], 1.0, 0.0, 10.0);
        lp.add_row([1.0, 1.0], 1.0, 5.0);
        let lp = lp.optimize();
        assert_eq!(lp.status(), Status::Optimal);
    }

    #[test]
    fn change_col_bounds() {
        let mut lp = Model::new();
        let col1 = lp.add_col([], 1.0, 0.0, 5.0);
        lp.change_col_bounds(col1, 0.0, 10.0);

        let lp = lp.optimize();
        let result = lp.status();
        assert_eq!(result, Status::Optimal);
        assert!((lp.obj_val() - 10.0).abs() < 1e-6);
    }

    #[test]
    fn change_row_range() {
        let mut lp = Model::new();
        lp.add_col([], 1.0, 1.0, 5.0);
        lp.add_col([], 1.0, 1.0, 10.0);
        let row = lp.add_row([1.0, 1.0], 1.0, 5.0);
        lp.change_row_range(row, 0.0, 0.0);

        let lp = lp.optimize();
        let result = lp.status();
        assert_eq!(result, Status::Infeasible);
    }


    #[test]
    fn basis_status() {
        let mut lp = Model::new();
        let col1 = lp.add_col([], 1.0, 0.0, 5.0);
        let _col2 = lp.add_col([], 1.0, 0.0, 10.0);
        let row = lp.add_row([1.0, 1.0], 1.0, 5.0);
        let lp = lp.optimize();
        let col_basis_status = lp.col_basis_status(col1);
        let row_basis_status = lp.row_basis_status(row);
        assert_eq!(col_basis_status, ColBasisStatus::AtLower);
        assert_eq!(row_basis_status, RowBasisStatus::AtUpper);
    }


    #[test]
    fn set_obj_sense() {
        let mut lp = Model::new();
        lp.set_obj_sense(ObjSense::Minimize);
        lp.add_col([], 1.0, 1.0, 5.0);
        let lp = lp.optimize();
        let result = lp.status();
        assert_eq!(result, Status::Optimal);
        assert!((lp.obj_val() - 1.0).abs() < 1e-6);
    }

    fn small_model() -> Model {
        let mut lp = Model::new();
        lp.add_col([], 1.0, 0.0, 5.0);
        lp.add_col([], 1.0, 0.0, 10.0);
        lp.add_row([1.0, 1.0], 1.0, 5.0);
        lp
    }

    #[test]
    fn set_algorithm() {
        let mut lp = small_model();
        lp.set_algorithm(Algorithm::Primal);
        let lp = lp.optimize();
        let result = lp.status();
        assert_eq!(result, Status::Optimal);
        assert!((lp.obj_val() - 5.0).abs() < 1e-6);

        let mut lp = small_model();
        lp.set_algorithm(Algorithm::Dual);
        let lp = lp.optimize();
        let result = lp.status();
        assert_eq!(result, Status::Optimal);
        assert!((lp.obj_val() - 5.0).abs() < 1e-6);
    }
}
