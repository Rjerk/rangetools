use crate::{
    BoundedRange, FiniteBound, LowerBoundedRange, RangeIntersection, UnboundedRange,
    UpperBoundedRange,
};

impl<T: Copy + Ord> RangeIntersection<T, std::ops::Range<T>> for LowerBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::Range<T>) -> Self::Output {
        let r: BoundedRange<T> = other.into();
        BoundedRange::new(
            FiniteBound::max_start(r.start_bound(), self.start_bound()),
            r.end_bound(),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeFrom<T>> for LowerBoundedRange<T> {
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeFrom<T>) -> Self::Output {
        let a = self.start_bound();
        let b = LowerBoundedRange::from(other).start_bound();
        LowerBoundedRange::new(FiniteBound::max_start(a, b))
    }
}

impl<T> RangeIntersection<T, std::ops::RangeFull> for LowerBoundedRange<T> {
    type Output = LowerBoundedRange<T>;
    fn intersection(self, _: std::ops::RangeFull) -> Self::Output {
        self
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeInclusive<T>> for LowerBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeInclusive<T>) -> Self::Output {
        let r: BoundedRange<T> = other.into();
        BoundedRange::new(
            FiniteBound::max_start(r.start_bound(), self.start_bound()),
            r.end_bound(),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeTo<T>> for LowerBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeTo<T>) -> Self::Output {
        BoundedRange::new(
            self.start_bound(),
            UpperBoundedRange::from(other).end_bound(),
        )
    }
}
impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeToInclusive<T>> for LowerBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeToInclusive<T>) -> Self::Output {
        BoundedRange::new(
            self.start_bound(),
            UpperBoundedRange::from(other).end_bound(),
        )
    }
}
impl<T: Copy + Ord> RangeIntersection<T, BoundedRange<T>> for LowerBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: BoundedRange<T>) -> Self::Output {
        BoundedRange::new(
            FiniteBound::max_start(self.start_bound(), other.start_bound()),
            other.end_bound(),
        )
    }
}
impl<T: Copy + Ord> RangeIntersection<T, LowerBoundedRange<T>> for LowerBoundedRange<T> {
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: LowerBoundedRange<T>) -> Self::Output {
        LowerBoundedRange::new(FiniteBound::max_start(
            self.start_bound(),
            other.start_bound(),
        ))
    }
}

impl<T: Copy + Ord> RangeIntersection<T, UpperBoundedRange<T>> for LowerBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: UpperBoundedRange<T>) -> Self::Output {
        BoundedRange::new(self.start_bound(), other.end_bound())
    }
}
impl<T> RangeIntersection<T, UnboundedRange<T>> for LowerBoundedRange<T> {
    type Output = LowerBoundedRange<T>;
    fn intersection(self, _: UnboundedRange<T>) -> Self::Output {
        self
    }
}
