use crate::{Rangetools, UnboundedRange};

impl Rangetools for UnboundedRange {
    fn is_empty(&self) -> bool {
        false
    }
    type Inner = Self;
    type Set = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }
    fn to_set(self) -> Self::Set {
        self
    }
}
