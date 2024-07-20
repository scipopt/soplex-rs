/// Represents the boolean parameters for some LP solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolParam {
    /// Should lifting be used to reduce range of nonzero matrix coefficients?
    Lifting = 0,
    /// Should LP be transformed to equality form before a rational solve?
    EqTrans = 1,
    /// Should dual infeasibility be tested in order to try to return a dual solution even if primal infeasible?
    TestDualInf = 2,
    /// Should a rational factorization be performed after iterative refinement?
    RatFac = 3,
    /// Should the decomposition based dual simplex be used to solve the LP?
    /// Setting this to true forces the solve mode to SOLVEMODE_REAL and the basis representation to REPRESENTATION_ROW
    UseDecompDualSimplex = 4,
    /// Should the degeneracy be computed for each basis?
    ComputeDegen = 5,
    /// Should the dual of the complementary problem be used in the decomposition simplex?
    UseCompDual = 6,
    /// Should row and bound violations be computed explicitly in the update of reduced problem in the decomposition simplex
    ExplicitViol = 7,
    /// Should cycling solutions be accepted during iterative refinement?
    AcceptCycling = 8,
    /// Apply rational reconstruction after each iterative refinement?
    RatRec = 9,
    /// Round scaling factors for iterative refinement to powers of two?
    PowerScaling = 10,
    /// Continue iterative refinement with exact basic solution if not optimal?
    RatFacJump = 11,
    /// Use bound flipping also for row representation?
    RowBoundFlips = 12,
    /// Use persistent scaling?
    PersistentScaling = 13,
    /// Perturb the entire problem or only the relevant bounds of a single pivot?
    FullPerturbation = 14,
    /// Re-optimize the original problem to get a proof (ray) of infeasibility/unboundedness?
    EnsureRay = 15,
    /// Try to enforce that the optimal solution is a basic solution
    ForceBasic = 16,
    /// Enable presolver SingletonCols in PaPILO?
    SimplifierSingletonCols = 17,
    /// Enable presolver ConstraintPropagation in PaPILO?
    SimplifierConstraintPropagation = 18,
    /// Enable presolver ParallelRowDetection in PaPILO?
    SimplifierParallelRowDetection = 19,
    /// Enable presolver ParallelColDetection in PaPILO?
    SimplifierParallelColDetection = 20,
    /// Enable presolver SingletonStuffing in PaPILO?
    SimplifierSingletonStuffing = 21,
    /// Enable presolver DualFix in PaPILO?
    SimplifierDualFix = 22,
    /// Enable presolver FixContinuous in PaPILO?
    SimplifierFixContinuous = 23,
    /// Enable presolver DominatedCols in PaPILO?
    SimplifierDominatedCols = 24,
}

pub(crate) const OBJSENSE_PARAM_ID: i32 = 0;
pub(crate) const REPR_PARAM_ID: i32 = 1;
pub(crate) const ALGORITHM_PARAM_ID: i32 = 2;
pub(crate) const FACTOR_UPDATE_TYPE_PARAM_ID: i32 = 3;
pub(crate) const VERBOSITY_PARAM_ID: i32 = 9;
pub(crate) const SIMPLIFIER_PARAM_ID: i32 = 10;
pub(crate) const SCALAR_PARAM_ID: i32 = 11;
pub(crate) const STARTER_PARAM_ID: i32 = 12;
pub(crate) const PRICER_PARAM_ID: i32 = 13;
pub(crate) const RATIO_TESTER_PARAM_ID: i32 = 14;
pub(crate) const SYNC_MODE_PARAM_ID: i32 = 15;
pub(crate) const READ_MODE_PARAM_ID: i32 = 16;
pub(crate) const SOLVE_MODE_PARAM_ID: i32 = 17;
pub(crate) const CHECK_MODE_PARAM_ID: i32 = 18;
pub(crate) const TIMER_PARAM_ID: i32 = 19;
pub(crate) const HYPER_PRICING_PARAM_ID: i32 = 20;
pub(crate) const SOLUTION_POLISHING_PARAM_ID: i32 = 23;
pub(crate) const DECOMP_VERBOSITY_PARAM_ID: i32 = 27;
pub(crate) const STAT_TIMER_PARAM_ID: i32 = 29;

/// Enum representing the objective sense for optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjSense {
    /// Minimize the objective function.
    Minimize = -1,
    /// Maximize the objective function.
    Maximize = 1,
}

/// Enum representing the type of representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Representation {
    /// Automatically determine the representation type.
    Auto = 0,
    /// Column-based representation.
    Column = 1,
    /// Row-based representation.
    Row = 2,
}

/// Enum representing the type of algorithm used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    /// Primal algorithm.
    Primal = 0,
    /// Dual algorithm.
    Dual = 1,
}

/// Enum representing the factor update type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactorUpdateType {
    /// ETA update type.
    Eta = 0,
    /// FT update type.
    Ft = 1,
}

/// Enum representing verbosity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Only show errors.
    Error = 0,
    /// Show warnings and errors.
    Warning = 1,
    /// Show debug information.
    Debug = 2,
    /// Show normal information.
    Normal = 3,
    /// Show high-level information.
    High = 4,
    /// Show full details.
    Full = 5,
}

/// Enum representing the simplifier type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Simplifier {
    /// Simplification is turned off.
    Off = 0,
    /// Use internal simplifier.
    Internal = 3,
    /// Use Papilo simplifier.
    Papilo = 2,
    /// Automatically choose the simplifier.
    Auto = 1,
}

/// Enum representing the scalar type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scalar {
    /// Scalar operation is turned off.
    Off = 0,
    /// Use uniequi scalar.
    Uniequi = 1,
    /// Use biequi scalar.
    Biequi = 2,
    /// GE01 scalar.
    Ge01 = 3,
    /// GE08 scalar.
    Ge08 = 4,
    /// Least squares scalar.
    Leastsq = 5,
    /// Geometric equivalence scalar.
    Geoequi = 6,
}

/// Enum representing the starter type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Starter {
    /// Starter is turned off.
    Off = 0,
    /// Use weight-based starter.
    Weight = 1,
    /// Use sum-based starter.
    Sum = 2,
    /// Use vector-based starter.
    Vector = 3,
}

/// Enum representing the pricer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pricer {
    /// Automatically choose the pricer.
    Auto = 0,
    /// Use Dantzig pricer.
    Dantzig = 1,
    /// Use parallel multiple pricer.
    Parmult = 2,
    /// Use Devex pricer.
    Devex = 3,
    /// Use quick steepest edge pricer.
    Quicksteep = 4,
    /// Use steepest edge pricer.
    Steep = 5,
}

/// Enum representing the ratio tester type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatioTester {
    /// Use textbook ratio test.
    Textbook = 0,
    /// Use Harris ratio test.
    Harris = 1,
    /// Use fast ratio test.
    Fast = 2,
    /// Use bound flipping ratio test.
    Boundflipping = 3,
}

/// Enum representing the synchronization mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum SyncMode {
    /// Only sync real values.
    Onlyreal = 0,
    /// Automatically sync.
    Auto = 1,
    /// Manually sync.
    Manual = 2,
}

/// Enum representing the read mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode {
    /// Read real values.
    Real = 0,
    /// Read rational values.
    Rational = 1,
}

/// Enum representing the solve mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum SolveMode {
    /// Solve with real values.
    Real = 0,
    /// Automatically determine the solve mode.
    Auto = 1,
    /// Solve with rational values.
    Rational = 2,
}

/// Enum representing the check mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum CheckMode {
    /// Check real values.
    Real = 0,
    /// Automatically determine the check mode.
    Auto = 1,
    /// Check rational values.
    Rational = 2,
}

/// Enum representing the timer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Timer {
    /// Timer is turned off.
    Off = 0,
    /// Use CPU timer.
    Cpu = 1,
    /// Use wall clock timer.
    Wallclock = 2,
}

/// Enum representing the hyperpricing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum HyperPricing {
    /// Hyperpricing is turned off.
    Off = 0,
    /// Automatically choose hyperpricing.
    Auto = 1,
    /// Hyperpricing is turned on.
    On = 2,
}

/// Enum representing the solution polishing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum SolutionPolishing {
    /// Solution polishing is turned off.
    Off = 0,
    /// Polishing for integrality.
    Integrality = 1,
    /// Polishing for fractionality.
    Fractionality = 2,
}

/// Represents the integer parameters for some LP solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntParam {
    /// Maximum number of updates without fresh factorization
    FactorUpdateMax = 4,
    /// Iteration limit (-1 if unlimited)
    IterLimit = 5,
    /// Refinement limit (-1 if unlimited)
    RefLimit = 6,
    /// Stalling refinement limit (-1 if unlimited)
    StallRefLimit = 7,
    /// Display frequency
    DisplayFreq = 8,
    /// Minimum number of stalling refinements since last pivot to trigger rational factorization
    RatFacMinStalls = 21,
    /// Maximum number of conjugate gradient iterations in least square scaling
    LeastSqMaxRounds = 22,
    /// Number of iterations before the decomposition simplex initialization is terminated
    DecompIterLimit = 24,
    /// Maximum number of rows added in each iteration of the decomposition based simplex
    DecompMaxAddedRows = 25,
    /// Iteration frequency at which the decomposition solve output is displayed
    DecompDisplayFreq = 26,
    /// Print condition number during the solve
    PrintBasisMetric = 28,
}

/// Represents the real number parameters for some LP solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealParam {
    /// Primal feasibility tolerance
    FeasTol = 0,
    /// Dual feasibility tolerance
    OptTol = 1,
    /// General zero tolerance
    EpsilonZero = 2,
    /// Zero tolerance used in factorization
    EpsilonFactorization = 3,
    /// Zero tolerance used in update of the factorization
    EpsilonUpdate = 4,
    /// Pivot zero tolerance used in factorization
    EpsilonPivot = 5,
    /// Infinity threshold
    Infty = 6,
    /// Time limit in seconds (INFTY if unlimited)
    TimeLimit = 7,
    /// Lower limit on objective value
    ObjLimitLower = 8,
    /// Upper limit on objective value
    ObjLimitUpper = 9,
    /// Working tolerance for feasibility in floating-point solver during iterative refinement
    FpFeasTol = 10,
    /// Working tolerance for optimality in floating-point solver during iterative refinement
    FpOptTol = 11,
    /// Maximum increase of scaling factors between refinements
    MaxScaleIncr = 12,
    /// Lower threshold in lifting (nonzero matrix coefficients with smaller absolute value will be reformulated)
    LiftMinVal = 13,
    /// Upper threshold in lifting (nonzero matrix coefficients with larger absolute value will be reformulated)
    LiftMaxVal = 14,
    /// Sparse pricing threshold (#violations < dimension * SPARSITY_THRESHOLD activates sparse pricing)
    SparsityThreshold = 15,
    /// Threshold on number of rows vs. number of columns for switching from column to row representations in auto mode
    RepresentationSwitch = 16,
    /// Geometric frequency at which to apply rational reconstruction
    RatRecFreq = 17,
    /// Minimal reduction (sum of removed rows/cols) to continue simplification
    MinRed = 18,
    /// Refactor threshold for nonzeros in last factorized basis matrix compared to updated basis matrix
    RefacBasisNnz = 19,
    /// Refactor threshold for fill-in in current factor update compared to fill-in in last factorization
    RefacUpdateFill = 20,
    /// Refactor threshold for memory growth in factorization since last refactorization
    RefacMemFactor = 21,
    /// Accuracy of conjugate gradient method in least squares scaling (higher value leads to more iterations)
    LeastSqAcrcy = 22,
    /// Objective offset
    ObjOffset = 23,
    /// Minimal Markowitz threshold to control sparsity/stability in LU factorization
    MinMarkowitz = 24,
    /// Minimal modification threshold to apply presolve reductions
    SimplifierModifyRowFac = 25,
}

macro_rules! impl_from_int_param {
    ($($param:ident),*) => {
        $(
            impl From<$param> for i32 {
                fn from(param: $param) -> i32 {
                    param as i32
                }
            }
        )*
    };
}

impl From<i32> for ObjSense {
    fn from(value: i32) -> Self {
        match value {
            -1 => ObjSense::Minimize,
            1 => ObjSense::Maximize,
            _ => panic!("Invalid value for ObjSense: {}", value),
        }
    }
}

impl_from_int_param!(
    BoolParam,
    IntParam,
    RealParam,
    ObjSense,
    Representation,
    Algorithm,
    FactorUpdateType,
    Verbosity,
    Simplifier,
    Scalar,
    Starter,
    Pricer,
    RatioTester,
    SyncMode,
    SolveMode,
    CheckMode,
    Timer,
    HyperPricing,
    SolutionPolishing,
    ReadMode
);
