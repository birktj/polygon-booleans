use nalgebra as na;
use num::{One, Zero};

pub type Point<T> = na::Point2<T>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Line<T: na::Scalar> {
    o: Point<T>,
    d: na::Vector2<T>,
}

impl<T: na::Scalar> Line<T> {
    /// Construct a new line given an origin point and a direction.
    pub fn new(origin: Point<T>, direction: na::Vector2<T>) -> Self {
        Self {
            o: origin,
            d: direction,
        }
    }

    /// Construct a new line that goes through two points.
    ///
    /// Panics if `p1` and `p2` are equal.
    pub fn from_two_points(p1: Point<T>, p2: Point<T>) -> Self
    where
        T: na::ClosedSubAssign,
    {
        assert!(
            p1 != p2,
            "a line must be constructed from two distinct points"
        );

        Self {
            // We deliberately choose to not normalize here to minimize numerical errors.
            d: p2 - &p1,
            o: p1,
        }
    }

    pub fn origin(&self) -> &Point<T> {
        &self.o
    }

    pub fn dir(&self) -> &na::Vector2<T> {
        &self.d
    }

    /// The point at `t`.
    pub fn point_at(&self, t: T) -> Point<T>
    where
        T: na::ClosedAddAssign + na::ClosedMulAssign,
    {
        self.o.clone() + self.d.clone() * t
    }

    /// Project a point onto this line.
    pub fn project(&self, point: &Point<T>) -> T::SimdRealField
    where
        T: na::SimdComplexField,
    {
        // https://mathworld.wolfram.com/Point-LineDistance3-Dimensional.html
        -(&self.o - point).dot(&self.d).simd_real() / self.d.norm_squared()
    }

    /// Distance from this line to a point.
    pub fn point_distance(&self, point: &Point<T>) -> T
    where
        T: na::SimdRealField,
    {
        let p = self.point_at(self.project(point));
        (point - p).norm()
    }

    /// Signed point distance.
    pub fn signed_point_distance(&self, point: &Point<T>) -> T
    where
        T: na::SimdRealField,
    {
        // https://mathworld.wolfram.com/Point-LineDistance2-Dimensional.html
        ((point - &self.o).perp(&self.d)) / self.d.norm()
    }
}

/// A line segment between two points.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct LineSegment<T: na::Scalar> {
    pub from: Point<T>,
    pub to: Point<T>,
}

impl<T: na::Scalar> LineSegment<T> {
    /// Create a new line segement from two points.
    pub fn new(from: Point<T>, to: Point<T>) -> Self {
        Self { from, to }
    }

    /// The endpoints of the line.
    pub fn points(&self) -> [Point<T>; 2] {
        [self.from.clone(), self.to.clone()]
    }

    /// The line that this line segment lies on.
    pub fn line(&self) -> Line<T>
    where
        T: na::ClosedSubAssign,
    {
        Line::from_two_points(self.from.clone(), self.to.clone())
    }

    /// Project a point to the closest point on this line segment.
    ///
    /// The result will always be clamped to the range `0..=1`.
    pub fn project(&self, point: &Point<T>) -> T::SimdRealField
    where
        T: na::SimdComplexField,
    {
        // TODO: make this robust against shared endpoints
        use na::SimdPartialOrd;

        self.line()
            .project(point)
            .simd_clamp(<T::SimdRealField>::zero(), <T::SimdRealField>::one())
    }

    /// The closest point to `point` that is on this line segment.
    pub fn closest_point(&self, point: &Point<T>) -> Point<T>
    where
        T: na::SimdRealField,
    {
        self.line().point_at(self.project(point))
    }

    /// Distance from this line segment to a point.
    pub fn point_distance(&self, point: &Point<T>) -> T
    where
        T: na::SimdRealField,
    {
        (point - &self.closest_point(point)).norm()
    }

    /// Squared distance from this line segment to a point.
    pub fn point_distance_squared(&self, point: &Point<T>) -> T
    where
        T: na::SimdRealField,
    {
        (point - &self.closest_point(point)).norm_squared()
    }
}

impl<T: na::Scalar + rstar::RTreeNum> From<LineSegment<T>>
    for rstar::primitives::Line<mint::Point2<T>>
{
    fn from(value: LineSegment<T>) -> Self {
        Self {
            from: value.from.into(),
            to: value.to.into(),
        }
    }
}

impl<T: na::Scalar + rstar::RTreeNum> From<rstar::primitives::Line<mint::Point2<T>>>
    for LineSegment<T>
{
    fn from(value: rstar::primitives::Line<mint::Point2<T>>) -> Self {
        Self {
            from: value.from.into(),
            to: value.to.into(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Rectangle<T: na::Scalar> {
    min_p: Point<T>,
    max_p: Point<T>,
}

impl<T: na::Scalar> Rectangle<T> {
    pub fn from_points(p1: Point<T>, p2: Point<T>) -> Self
    where
        T: na::SimdPartialOrd,
    {
        let (min_p, max_p) = p1.coords.inf_sup(&p2.coords);

        Self {
            min_p: min_p.into(),
            max_p: max_p.into(),
        }
    }

    pub fn min_x(&self) -> T {
        self.min_p.x.clone()
    }

    pub fn max_x(&self) -> T {
        self.max_p.x.clone()
    }

    pub fn min_y(&self) -> T {
        self.min_p.y.clone()
    }

    pub fn max_y(&self) -> T {
        self.max_p.y.clone()
    }

    pub fn corners(&self) -> [Point<T>; 4] {
        [
            self.min_p.clone(),
            Point::new(self.min_x(), self.max_y()),
            Point::new(self.max_x(), self.min_y()),
            self.max_p.clone(),
        ]
    }

    pub fn contains_point(&self, point: &Point<T>) -> T::SimdBool
    where
        T: na::SimdPartialOrd,
    {
        self.min_p.x.clone().simd_ge(point.x.clone())
            & self.min_p.y.clone().simd_ge(point.y.clone())
            & self.max_p.x.clone().simd_le(point.x.clone())
            & self.max_p.y.clone().simd_le(point.y.clone())
    }

    pub fn closest_point(&self, point: &Point<T>) -> Point<T>
    where
        T: na::SimdPartialOrd,
    {
        self.max_p
            .coords
            .inf(&self.min_p.coords.sup(&point.coords))
            .into()
    }

    pub fn point_distance(&self, point: &Point<T>) -> T
    where
        T: na::SimdRealField,
    {
        use na::SimdBool;

        self.contains_point(point)
            .if_else(|| T::zero(), || (self.closest_point(point) - point).norm())
    }

    // pub fn project_onto_line(&self, line: &Line<T>) -> (T, T)
    // where
    //     T: na::SimdRealField,
    // {
    //     self.corners()
    //         .into_iter()
    //         .map(|c| line.project(&c))
    //         .map(|v| (v.clone(), v))
    //         .reduce(|(mi1, ma1), (mi2, ma2)| (mi1.simd_min(mi2), ma1.simd_max(ma2)))
    //         .unwrap()
    // }

    pub fn line_segment_distance(&self, line: &LineSegment<T>) -> T
    where
        T: na::SimdRealField,
    {
        self.corners()
            .into_iter()
            .map(|c| line.closest_point(&c))
            .map(|p| self.point_distance(&p))
            .reduce(|a, b| a.simd_min(b))
            .unwrap()
    }
}

impl<T: na::Scalar + rstar::RTreeNum> From<rstar::AABB<mint::Point2<T>>> for Rectangle<T> {
    fn from(value: rstar::AABB<mint::Point2<T>>) -> Self {
        Self {
            min_p: value.lower().into(),
            max_p: value.upper().into(),
        }
    }
}
