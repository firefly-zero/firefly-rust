use super::*;
use core::num::TryFromIntError;
use core::ops::*;
#[cfg(feature = "nalgebra_support")]
use nalgebra::{base::Scalar, Vector2};

/// A point on the screen.
///
/// Typically, the upper-left corner of a bounding box of a shape.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Point {
    /// Pixel distance from left.
    pub x: i32,
    /// Pixel distance from the top.
    pub y: i32,
}

impl Point {
    /// The coordinate of the bottom-right corner on the screen.
    pub const MAX: Point = Point {
        x: WIDTH,
        y: HEIGHT,
    };
    /// The coordinate of the top-left corner on the screen.
    pub const MIN: Point = Point { x: 0, y: 0 };

    /// Create a new point casting the argument types.
    #[must_use]
    pub fn new<I: Into<i32>>(x: I, y: I) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    /// Set x and y to their absolute (non-negative) value.
    #[must_use]
    pub const fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Set both x and y to their minimum in the two given points.
    #[must_use]
    pub fn component_min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Set both x and y to their maximum in the two given points.
    #[must_use]
    pub fn component_max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Size> for Point {
    type Output = Self;

    fn add(self, other: Size) -> Self {
        Self {
            x: self.x + other.width,
            y: self.y + other.height,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign<Size> for Point {
    fn add_assign(&mut self, other: Size) {
        self.x += other.width;
        self.y += other.height;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Point) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<Size> for Point {
    type Output = Self;

    fn sub(self, other: Size) -> Self {
        Self {
            x: self.x - other.width,
            y: self.y - other.height,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl SubAssign<Size> for Point {
    fn sub_assign(&mut self, other: Size) {
        self.x -= other.width;
        self.y -= other.height;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<i32> for Point {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Index<usize> for Point {
    type Output = i32;

    fn index(&self, idx: usize) -> &i32 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds: the len is 2 but the index is {}", idx),
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<Size> for Point {
    fn from(value: Size) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(other: (i32, i32)) -> Self {
        Self {
            x: other.0,
            y: other.1,
        }
    }
}

impl From<[i32; 2]> for Point {
    fn from(other: [i32; 2]) -> Self {
        Self {
            x: other[0],
            y: other[1],
        }
    }
}

impl From<&[i32; 2]> for Point {
    fn from(other: &[i32; 2]) -> Self {
        Self {
            x: other[0],
            y: other[1],
        }
    }
}

impl From<Point> for (i32, i32) {
    fn from(other: Point) -> (i32, i32) {
        (other.x, other.y)
    }
}

impl From<Point> for [i32; 2] {
    fn from(other: Point) -> [i32; 2] {
        [other.x, other.y]
    }
}

impl From<&Point> for (i32, i32) {
    fn from(other: &Point) -> (i32, i32) {
        (other.x, other.y)
    }
}

impl TryFrom<Point> for (u32, u32) {
    type Error = TryFromIntError;

    fn try_from(point: Point) -> Result<Self, Self::Error> {
        Ok((point.x.try_into()?, point.y.try_into()?))
    }
}

impl TryFrom<(u32, u32)> for Point {
    type Error = TryFromIntError;

    fn try_from(point: (u32, u32)) -> Result<Self, Self::Error> {
        let x = point.0.try_into()?;
        let y = point.1.try_into()?;

        Ok(Self { x, y })
    }
}

impl TryFrom<Point> for [u32; 2] {
    type Error = TryFromIntError;

    fn try_from(point: Point) -> Result<Self, Self::Error> {
        Ok([point.x.try_into()?, point.y.try_into()?])
    }
}

impl TryFrom<[u32; 2]> for Point {
    type Error = TryFromIntError;

    fn try_from(point: [u32; 2]) -> Result<Self, Self::Error> {
        let x = point[0].try_into()?;
        let y = point[1].try_into()?;

        Ok(Self { x, y })
    }
}

impl TryFrom<&[u32; 2]> for Point {
    type Error = TryFromIntError;

    fn try_from(point: &[u32; 2]) -> Result<Self, Self::Error> {
        let x = point[0].try_into()?;
        let y = point[1].try_into()?;

        Ok(Self { x, y })
    }
}

#[cfg(feature = "nalgebra_support")]
impl<N> From<Vector2<N>> for Point
where
    N: Into<i32> + Scalar + Copy,
{
    fn from(other: Vector2<N>) -> Self {
        Self {
            x: other[0].into(),
            y: other[1].into(),
        }
    }
}

#[cfg(feature = "nalgebra_support")]
impl<N> From<&Vector2<N>> for Point
where
    N: Into<i32> + Scalar + Copy,
{
    fn from(other: &Vector2<N>) -> Self {
        Self {
            x: other[0].into(),
            y: other[1].into(),
        }
    }
}
