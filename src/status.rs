use std::convert::From;


/// Status of the solver
#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    /// An error occurred
    Error,
    /// No ratiotester loaded
    NoRatioTester,
    /// No pricer loaded
    NoPricer,
    /// No linear solver loaded
    NoSolver,
    /// Not initialised error
    NotInit,
    /// Solve() aborted to exit decomposition simplex
    AbortExdecomp,
    /// Solve() aborted due to commence decomposition simplex
    AbortDecomp,
    /// Solve() aborted due to detection of cycling
    AbortCycling,
    /// Solve() aborted due to time limit
    AbortTime,
    /// Solve() aborted due to iteration limit
    AbortIter,
    /// Solve() aborted due to objective limit
    AbortValue,
    /// Basis is singular, numerical troubles?
    Singular,
    /// No Problem has been loaded
    NoProblem,
    /// LP has a usable Basis (maybe LP is changed)
    Regular,
    /// Algorithm is running
    Running,
    /// Nothing known on loaded problem
    Unknown,
    /// LP has been solved to optimality
    Optimal,
    /// LP has been proven to be primal unbounded
    Unbounded,
    /// LP has been proven to be primal infeasible
    Infeasible,
    /// LP is primal infeasible or unbounded
    InForUnbd,
    /// LP has been solved to optimality but unscaled solution contains violations
    OptimalUnscaledViolations,
}

impl From<i32> for Status {
    fn from(item: i32) -> Self {
        match item {
            -15 => Status::Error,
            -14 => Status::NoRatioTester,
            -13 => Status::NoPricer,
            -12 => Status::NoSolver,
            -11 => Status::NotInit,
            -10 => Status::AbortExdecomp,
            -9 => Status::AbortDecomp,
            -8 => Status::AbortCycling,
            -7 => Status::AbortTime,
            -6 => Status::AbortIter,
            -5 => Status::AbortValue,
            -4 => Status::Singular,
            -3 => Status::NoProblem,
            -2 => Status::Regular,
            -1 => Status::Running,
            0 => Status::Unknown,
            1 => Status::Optimal,
            2 => Status::Unbounded,
            3 => Status::Infeasible,
            4 => Status::InForUnbd,
            5 => Status::OptimalUnscaledViolations,
            _ => panic!("Invalid value for Status"),
        }
    }
}
