use super::Point;
use core::ops::*;

pub const WIDTH: i32 = 240;
pub const HEIGHT: i32 = 160;

/// Size of a bounding box for a shape.
///
/// The width and height must be positive.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Size {
    pub width:  i32,
    pub height: i32,
}

impl Size {
    /// The screen size.
    pub const MAX: Size = Size {
        width:  WIDTH,
        height: HEIGHT,
    };

    pub fn abs(&self) -> Self {
        Self {
            width:  self.width.abs(),
            height: self.height.abs(),
        }
    }

    pub fn component_min(self, other: Self) -> Self {
        Self {
            width:  self.width.min(other.width),
            height: self.height.min(other.height),
        }
    }

    pub fn component_max(self, other: Self) -> Self {
        Self {
            width:  self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, other: Size) -> Self {
        Size {
            width:  self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Size) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, other: Size) -> Self {
        Size {
            width:  self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl Sub<Point> for Size {
    type Output = Size;

    fn sub(self, other: Point) -> Self {
        Size {
            width:  self.width - other.x,
            height: self.height - other.y,
        }
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, other: Size) {
        self.width -= other.width;
        self.height -= other.height;
    }
}

impl Mul<i32> for Size {
    type Output = Size;

    fn mul(self, rhs: i32) -> Self {
        Size {
            width:  self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl MulAssign<i32> for Size {
    fn mul_assign(&mut self, rhs: i32) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl Div<i32> for Size {
    type Output = Size;

    fn div(self, rhs: i32) -> Self {
        Size {
            width:  self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl DivAssign<i32> for Size {
    fn div_assign(&mut self, rhs: i32) {
        self.width /= rhs;
        self.height /= rhs;
    }
}

impl Index<usize> for Size {
    type Output = i32;

    fn index(&self, idx: usize) -> &i32 {
        match idx {
            0 => &self.width,
            1 => &self.height,
            _ => panic!("index out of bounds: the len is 2 but the index is {}", idx),
        }
    }
}

impl From<Point> for Size {
    fn from(value: Point) -> Self {
        Self {
            width:  value.x,
            height: value.y,
        }
    }
}
impl From<(i32, i32)> for Size {
    fn from(other: (i32, i32)) -> Self {
        Size {
            width:  other.0,
            height: other.1,
        }
    }
}

impl From<[i32; 2]> for Size {
    fn from(other: [i32; 2]) -> Self {
        Size {
            width:  other[0],
            height: other[1],
        }
    }
}

impl From<&[i32; 2]> for Size {
    fn from(other: &[i32; 2]) -> Self {
        Size {
            width:  other[0],
            height: other[1],
        }
    }
}

impl From<Size> for (i32, i32) {
    fn from(other: Size) -> (i32, i32) {
        (other.width, other.height)
    }
}

impl From<Size> for [i32; 2] {
    fn from(other: Size) -> [i32; 2] {
        [other.width, other.height]
    }
}

impl From<&Size> for (i32, i32) {
    fn from(other: &Size) -> (i32, i32) {
        (other.width, other.height)
    }
}
