//! Structs for working with shapes as values.
//!
//! The [graphics] module provides functions for drawing shapes.
//! This modules provides useful struct for when you need to store
//! or manipulate a shape before it can be drawn.
use crate::*;

pub trait Shape {
    fn draw(&self);
}

/// A wrapper for [draw_line].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Line {
    pub a:     Point,
    pub b:     Point,
    pub style: LineStyle,
}

impl Shape for Line {
    fn draw(&self) {
        draw_line(self.a, self.b, self.style)
    }
}

/// A wrapper for [draw_rect].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Rect {
    pub point: Point,
    pub size:  Size,
    pub style: Style,
}

impl Shape for Rect {
    fn draw(&self) {
        draw_rect(self.point, self.size, self.style)
    }
}

impl From<RoundedRect> for Rect {
    fn from(value: RoundedRect) -> Self {
        Self {
            point: value.point,
            size:  value.size,
            style: value.style,
        }
    }
}

/// A wrapper for [draw_rounded_rect].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RoundedRect {
    pub point:  Point,
    pub size:   Size,
    pub corner: Size,
    pub style:  Style,
}

impl Shape for RoundedRect {
    fn draw(&self) {
        draw_rounded_rect(self.point, self.size, self.corner, self.style)
    }
}

impl From<Rect> for RoundedRect {
    fn from(value: Rect) -> Self {
        Self {
            point:  value.point,
            size:   value.size,
            corner: Size {
                width:  0,
                height: 0,
            },
            style:  value.style,
        }
    }
}

/// A wrapper for [draw_circle].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Circle {
    pub point:    Point,
    pub diameter: i32,
    pub style:    Style,
}

impl Shape for Circle {
    fn draw(&self) {
        draw_circle(self.point, self.diameter, self.style)
    }
}

/// A wrapper for [draw_ellipse].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Ellipse {
    pub point: Point,
    pub size:  Size,
    pub style: Style,
}

impl Shape for Ellipse {
    fn draw(&self) {
        draw_ellipse(self.point, self.size, self.style)
    }
}

impl From<Circle> for Ellipse {
    fn from(value: Circle) -> Self {
        Self {
            point: value.point,
            size:  Size {
                width:  value.diameter,
                height: value.diameter,
            },
            style: value.style,
        }
    }
}

/// A wrapper for [draw_triangle].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Triangle {
    pub a:     Point,
    pub b:     Point,
    pub c:     Point,
    pub style: Style,
}

impl Shape for Triangle {
    fn draw(&self) {
        draw_triangle(self.a, self.b, self.c, self.style)
    }
}

/// A wrapper for [draw_arc].
#[derive(Clone, Debug)]
pub struct Arc {
    pub point:    Point,
    pub diameter: i32,
    pub start:    Angle,
    pub sweep:    Angle,
    pub style:    Style,
}

impl Shape for Arc {
    fn draw(&self) {
        draw_arc(
            self.point,
            self.diameter,
            self.start,
            self.sweep,
            self.style,
        )
    }
}

impl From<Sector> for Arc {
    fn from(value: Sector) -> Self {
        Self {
            point:    value.point,
            diameter: value.diameter,
            start:    value.start,
            sweep:    value.sweep,
            style:    value.style,
        }
    }
}

/// A wrapper for [draw_sector].
#[derive(Clone, Debug)]
pub struct Sector {
    pub point:    Point,
    pub diameter: i32,
    pub start:    Angle,
    pub sweep:    Angle,
    pub style:    Style,
}

impl Shape for Sector {
    fn draw(&self) {
        draw_sector(
            self.point,
            self.diameter,
            self.start,
            self.sweep,
            self.style,
        )
    }
}

impl From<Arc> for Sector {
    fn from(value: Arc) -> Self {
        Self {
            point:    value.point,
            diameter: value.diameter,
            start:    value.start,
            sweep:    value.sweep,
            style:    value.style,
        }
    }
}
