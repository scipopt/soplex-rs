#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasisStatus {
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

impl From<i32> for BasisStatus {
    fn from(item: i32) -> Self {
        match item {
            0 => BasisStatus::AtUpper,
            1 => BasisStatus::AtLower,
            2 => BasisStatus::Fixed,
            3 => BasisStatus::Free,
            4 => BasisStatus::Basic,
            5 => BasisStatus::Unknown,
            _ => panic!("Invalid value for BasisStatus"),
        }
    }
}