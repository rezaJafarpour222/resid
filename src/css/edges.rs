use crate::units::Pt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edges {
    pub top: Pt,
    pub right: Pt,
    pub bottom: Pt,
    pub left: Pt,
}

impl Edges {
    pub const ZERO: Self = Self {
        top: Pt::ZERO,
        right: Pt::ZERO,
        bottom: Pt::ZERO,
        left: Pt::ZERO,
    };

    pub const fn all(value: Pt) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub const fn vertical_horizontal(vertical: Pt, horizontal: Pt) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}
