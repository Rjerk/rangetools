use crate::{
    BoundedSet, LowerBoundedSet, RangeUnion, Rangetools, UnboundedRange, UnboundedSet,
    UpperBoundedSet,
};

impl<T, R> RangeUnion<R, BoundedSet<T>> for std::ops::RangeFrom<T>
where
    R: Rangetools<Set = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, LowerBoundedSet<T>> for std::ops::RangeFrom<T>
where
    R: Rangetools<Set = LowerBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, UpperBoundedSet<T>> for std::ops::RangeFrom<T>
where
    R: Rangetools<Set = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, UnboundedSet<T>> for std::ops::RangeFrom<T>
where
    R: Rangetools<Set = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, UnboundedRange> for std::ops::RangeFrom<T>
where
    R: Rangetools<Set = UnboundedRange>,
    T: Copy + Ord,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(other.to_set(), self)
    }
}
