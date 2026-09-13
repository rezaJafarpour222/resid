use crate::units::{Millimeter, Pt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Page {
    pub width: Pt,
    pub height: Pt,
    pub margin_top: Pt,
    pub margin_right: Pt,
    pub margin_bottom: Pt,
    pub margin_left: Pt,
}

impl Page {
    pub fn a4_portrait() -> Self {
        Self {
            width: Millimeter::new(210.0).into(),
            height: Millimeter::new(297.0).into(),
            margin_top: Millimeter::new(20.0).into(),
            margin_right: Millimeter::new(20.0).into(),
            margin_bottom: Millimeter::new(20.0).into(),
            margin_left: Millimeter::new(20.0).into(),
        }
    }
    pub fn a4_landscape() -> Self {
        Self {
            width: Millimeter::new(297.0).into(),
            height: Millimeter::new(210.0).into(),
            margin_top: Millimeter::new(20.0).into(),
            margin_right: Millimeter::new(20.0).into(),
            margin_bottom: Millimeter::new(20.0).into(),
            margin_left: Millimeter::new(20.0).into(),
        }
    }
    pub fn a5() -> Self {
        Self {
            width: Millimeter::new(148.0).into(),
            height: Millimeter::new(210.0).into(),
            margin_top: Millimeter::new(20.0).into(),
            margin_right: Millimeter::new(20.0).into(),
            margin_bottom: Millimeter::new(20.0).into(),
            margin_left: Millimeter::new(20.0).into(),
        }
    }

    pub fn a3() -> Self {
        Self {
            width: Millimeter::new(297.0).into(),
            height: Millimeter::new(420.0).into(),
            margin_top: Millimeter::new(20.0).into(),
            margin_right: Millimeter::new(20.0).into(),
            margin_bottom: Millimeter::new(20.0).into(),
            margin_left: Millimeter::new(20.0).into(),
        }
    }
    pub fn a6() -> Self {
        Self {
            width: Millimeter::new(105.0).into(),
            height: Millimeter::new(148.0).into(),
            margin_top: Millimeter::new(20.0).into(),
            margin_right: Millimeter::new(20.0).into(),
            margin_bottom: Millimeter::new(20.0).into(),
            margin_left: Millimeter::new(20.0).into(),
        }
    }

    pub fn content_width(self) -> Pt {
        Pt::new(self.width.value() - self.margin_left.value() - self.margin_right.value())
    }

    pub fn content_height(self) -> Pt {
        Pt::new(self.height.value() - self.margin_top.value() - self.margin_bottom.value())
    }
}
