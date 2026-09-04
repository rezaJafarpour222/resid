use crate::{
    css::types::{Border, Color, FontWeight},
    font::types::ShapedText,
    units::{Direction, Position, Pt, Rectangle, Size},
};

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
