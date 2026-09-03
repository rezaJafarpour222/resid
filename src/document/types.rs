use crate::{
    css::types::{Border, Color, FontWeight},
    font::types::ShapedText,
    units::{Direction, Millimeter, Position, Pt, Rectangle, Size},
};

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
    pub fn a4() -> Self {
        Self {
            width: Millimeter::new(210.0).into(),
            height: Millimeter::new(297.0).into(),
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

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutDocument {
    pub pages: Vec<LayoutPage>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutPage {
    pub size: Size,
    pub blocks: Vec<LayoutBlock>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutBlock {
    pub rect: Rectangle,
    pub content: LayoutContent,
    pub background: Option<Color>,
    pub border: Border,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutContent {
    pub lines: Vec<LayoutLine>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutLine {
    pub text: String,
    pub glyphs: ShapedText,
    pub width: Pt,
    pub position: Position,
    pub font_size: Pt,
    pub direction: Direction,
    pub color: Color,
    pub font_weight: FontWeight,
}
