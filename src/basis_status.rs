/// Column basis status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColBasisStatus {
    /// column is set to its upper bound
    AtUpper = 0,
    /// column is set to its lower bound
    AtLower = 1,
    /// column is fixed to its identical bounds
    Fixed = 2,
    /// column is free and fixed to zero
    Free = 3,
    /// column is basic
    Basic = 4,
    /// nothing known about basis status
    Unknown = 5,
}


/// Row basis status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowBasisStatus {
    /// row is set to its upper bound
    AtUpper = 0,
    /// row is set to its lower bound
    AtLower = 1,
    /// row is fixed to its identical bounds
    Fixed = 2,
    /// row is basic
    Basic = 4,
    /// nothing known about basis status
    Unknown = 5,
}

impl From<i32> for ColBasisStatus {
    fn from(item: i32) -> Self {
        match item {
            0 => ColBasisStatus::AtUpper,
            1 => ColBasisStatus::AtLower,
            2 => ColBasisStatus::Fixed,
            3 => ColBasisStatus::Free,
            4 => ColBasisStatus::Basic,
            5 => ColBasisStatus::Unknown,
            _ => panic!("Invalid value for BasisStatus"),
        }
    }
}

impl From<i32> for RowBasisStatus {
    fn from(item: i32) -> Self {
        match item {
            0 => RowBasisStatus::AtUpper,
            1 => RowBasisStatus::AtLower,
            2 => RowBasisStatus::Fixed,
            4 => RowBasisStatus::Basic,
            5 => RowBasisStatus::Unknown,
            _ => panic!("Invalid value for BasisStatus"),
        }
    }
}