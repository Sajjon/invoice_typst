use crate::prelude::*;

/// A month of the year, e.g. 1 for January, 2 for February, etc.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Serialize, Deserialize, Deref,
)]
pub struct Month(u8);

impl From<u8> for Month {
    fn from(month: u8) -> Self {
        assert!((1..=12).contains(&month), "Month must be between 1 and 12");
        Self(month)
    }
}

impl From<i32> for Month {
    fn from(month: i32) -> Self {
        Self::from(month as u8)
    }
}

impl From<u32> for Month {
    fn from(month: u32) -> Self {
        Self::from(month as i32)
    }
}
