use crate::font::units::Pt;

pub enum Direction {
    RTL,
    LTR,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapedGlyph {
    pub id: u32,
    pub x_advance: i32,
    pub y_advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub cluster: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapedText {
    pub text: String,
    pub glyphs: Vec<ShapedGlyph>,
    pub width: Pt,
}
