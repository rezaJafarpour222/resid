#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub f32);
impl Point {
    pub const ZERO: Self = Self(0.0);
    pub fn new(value: f32) -> Self {
        Self(value)
    }
    pub fn value(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Millimeter(pub f32);
impl Millimeter {
    pub fn new(value: f32) -> Self {
        Self(value)
    }
    pub fn value(self) -> f32 {
        self.0
    }
}
impl From<Point> for Millimeter {
    fn from(value: Point) -> Self {
        Millimeter(value.value() * 25.4 / 72.0)
    }
}

impl From<Millimeter> for Point {
    fn from(value: Millimeter) -> Self {
        Point(value.value() * 72.0 / 25.4)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: Point,
    pub height: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: Point,
    pub y: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub position: Position,
    pub size: Size,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_mm_to_points() {
        let pt: Point = Millimeter::new(25.4).into();

        assert!((pt.value() - 72.0).abs() < 0.001);
    }

    #[test]
    fn converts_points_to_mm() {
        let mm: Millimeter = Point::new(72.0).into();

        assert!((mm.0 - 25.4).abs() < 0.001);
    }

    #[test]
    fn zero_point_is_zero() {
        assert_eq!(Point::ZERO, Point::new(0.0));
    }
}
