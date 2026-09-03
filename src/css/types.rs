use crate::units::{Direction, Pt};

use super::edges::Edges;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Block,
    Inline,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Bold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Start,
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Border {
    pub width: Pt,
    pub color: Color,
}

impl Border {
    pub const NONE: Self = Self {
        width: Pt::ZERO,
        color: Color::BLACK,
    };

    pub const fn solid(width: Pt, color: Color) -> Self {
        Self { width, color }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyle {
    pub display: Display,
    pub direction: Direction,

    pub font_family: Option<String>,
    pub font_size: Pt,
    pub font_weight: FontWeight,
    pub line_height: f32,

    pub text_align: TextAlign,
    pub color: Color,
    pub background_color: Option<Color>,

    pub margin: Edges,
    pub padding: Edges,
    pub border: Border,
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self {
            display: Display::Inline,
            direction: Direction::LTR,
            font_family: None,
            font_size: Pt::new(12.0),
            font_weight: FontWeight::Normal,
            line_height: 1.5,
            text_align: TextAlign::Start,
            color: Color::BLACK,
            background_color: None,
            margin: Edges::ZERO,
            padding: Edges::ZERO,
            border: Border::NONE,
        }
    }
}
