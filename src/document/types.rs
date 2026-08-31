use crate::{
    font::types::ShapedText,
    units::{Direction, Millimeter, Position, Pt, Rectangle, Size},
};

#[derive(Debug, PartialEq)]
pub struct Document {
    pub page: Page,
    pub blocks: Vec<Block>,
}

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
}

#[derive(Debug, PartialEq)]
pub struct Style {
    pub direction: Direction,
    pub font_size: f32,
    pub line_height: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            direction: Direction::LTR,
            font_size: 12.0,
            line_height: 1.5,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Block {
    Paragraph {
        content: InlineContent,
        style: Style,
    },

    Heading {
        level: u8,
        content: InlineContent,
        style: Style,
    },
}

#[derive(Debug, PartialEq)]
pub struct InlineContent {
    pub items: Vec<Inline>,
}

#[derive(Debug, PartialEq)]
pub enum Inline {
    Text(String),
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
}

#[derive(Debug, PartialEq)]
pub struct LayoutText {
    pub text: String,
    pub shaped: ShapedText,
}
