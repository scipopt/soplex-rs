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

/// Represents the integer parameters for some LP solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntParam {
    /// Objective sense
    /// TODO: This should be an enum
    ObjSense = 0,
    /// Type of computational form i.e., column or row representation
    Representation = 1,
    /// Type of algorithm i.e., primal or dual
    /// TODO: This should be an enum
    Algorithm = 2,
    /// Type of LU update
    FactorUpdateType = 3,
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
    /// Verbosity level
    /// TODO: This should be an enum
    Verbosity = 9,
    /// Type of simplifier
    /// TODO: This should be an enum
    Simplifier = 10,
    /// Type of scaler
    /// TODO: This should be an enum
    Scaler = 11,
    /// Type of starter used to create crash basis
    /// TODO: This should be an enum
    Starter = 12,
    /// Type of pricer
    /// TODO: This should be an enum
    Pricer = 13,
    /// Type of ratio test
    /// TODO: This should be an enum
    RatioTester = 14,
    /// Mode for synchronizing real and rational LP
    /// TODO: This should be an enum
    SyncMode = 15,
    /// Mode for reading LP files
    /// TODO: This should be an enum
    ReadMode = 16,
    /// Mode for iterative refinement strategy
    /// TODO: This should be an enum
    SolveMode = 17,
    /// Mode for a posteriori feasibility checks
    /// TODO: This should be an enum
    CheckMode = 18,
    /// Type of timer
    /// TODO: This should be an enum
    Timer = 19,
    /// Mode for hyper sparse pricing
    /// TODO: This should be an enum
    HyperPricing = 20,
    /// Minimum number of stalling refinements since last pivot to trigger rational factorization
    RatFacMinStalls = 21,
    /// Maximum number of conjugate gradient iterations in least square scaling
    LeastSqMaxRounds = 22,
    /// Mode for solution polishing
    /// TODO: This should be an enum
    SolutionPolishing = 23,
    /// Number of iterations before the decomposition simplex initialization is terminated
    DecompIterLimit = 24,
    /// Maximum number of rows added in each iteration of the decomposition based simplex
    DecompMaxAddedRows = 25,
    /// Iteration frequency at which the decomposition solve output is displayed
    DecompDisplayFreq = 26,
    /// Verbosity of the decomposition based simplex
    /// TODO: This should be an enum
    DecompVerbosity = 27,
    /// Print condition number during the solve
    PrintBasisMetric = 28,
    /// Type of timer for statistics
    /// TODO: This should be an enum
    StatTimer = 29,
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

impl From<BoolParam> for i32 {
    fn from(param: BoolParam) -> i32 {
        param as i32
    }
}

impl From<IntParam> for i32 {
    fn from(param: IntParam) -> i32 {
        param as i32
    }
}

impl From<RealParam> for i32 {
    fn from(param: RealParam) -> i32 {
        param as i32
    }
}