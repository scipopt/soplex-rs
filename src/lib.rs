mod status;

use std::ffi::c_void;
use crate::status::Status;

pub mod ffi {
    pub use soplex_sys::*;
}


struct SoPlex {
    inner: *mut c_void,
}

impl SoPlex {
    pub fn new() -> Self {
        let inner = unsafe { ffi::SoPlex_create() };
        assert!(!inner.is_null());
        Self { inner }
    }

    pub fn add_col_real<const N: usize>(&mut self, mut colentries: [f64; N], objval: f64, lb: f64, ub: f64) {
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

    pub fn add_row_real<const N: usize>(&mut self, mut rowentries: [f64; N], lhs: f64, rhs: f64) {
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

    pub fn optimize(&mut self) -> Status {
        unsafe { ffi::SoPlex_optimize(self.inner) }.into()
    }

    pub fn status(&self) -> Status {
        unsafe { ffi::SoPlex_getStatus(self.inner) }.into()
    }
}


impl Drop for SoPlex {
    fn drop(&mut self) {
        unsafe { ffi::SoPlex_free(self.inner) };
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn simple_problem() {
        let mut lp = SoPlex::new();
        lp.add_col_real([], 1.0, 0.0, 5.0);
        lp.add_col_real([], 1.0, 0.0, 5.0);
        lp.add_row_real([1.0, 1.0], 1.0, 5.0);
        let result = lp.optimize();
        assert_eq!(result, Status::Optimal);
    }
}
