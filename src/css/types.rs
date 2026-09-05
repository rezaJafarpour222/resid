use super::edges::Edges;
use crate::units::{Direction, Pt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    Table,
    TableRow,
    TableCell,
    ListItem,
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
    Justify,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Visible,
    Hidden,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhiteSpace {
    Normal,
    NoWrap,
    Pre,
    PreWrap,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecoration {
    None,
    Underline,
    LineThrough,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListStyleType {
    None,
    Disc,
    Circle,
    Square,
    Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Auto,
    Pt(Pt),
    Percent(f32),
}
impl Length {
    pub const AUTO: Self = Self::Auto;
    pub fn resolve(self, containing: Pt, auto: Pt) -> Pt {
        match self {
            Self::Auto => auto,
            Self::Pt(v) => v,
            Self::Percent(v) => Pt::new(containing.value() * v / 100.0),
        }
    }
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
    pub position: Position,
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
    pub width: Length,
    pub height: Length,
    pub min_width: Length,
    pub max_width: Length,
    pub min_height: Length,
    pub max_height: Length,
    pub box_sizing: BoxSizing,
    pub overflow: Overflow,
    pub font_family: Option<String>,
    pub font_size: Pt,
    pub font_weight: FontWeight,
    pub line_height: f32,
    pub text_align: TextAlign,
    pub color: Color,
    pub text_decoration: TextDecoration,
    pub white_space: WhiteSpace,
    pub letter_spacing: Pt,
    pub word_spacing: Pt,
    pub text_indent: Pt,
    pub background_color: Option<Color>,
    pub opacity: f32,
    pub margin: Edges,
    pub padding: Edges,
    pub border: Border,
    pub border_radius: Pt,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Length,
    pub gap: Pt,
    pub row_gap: Pt,
    pub column_gap: Pt,
    pub list_style_type: ListStyleType,
    pub list_style_position_inside: bool,
    pub page_break_before: bool,
    pub page_break_after: bool,
}
impl Default for ComputedStyle {
    fn default() -> Self {
        Self {
            display: Display::Inline,
            direction: Direction::LTR,
            position: Position::Static,
            top: Length::Auto,
            right: Length::Auto,
            bottom: Length::Auto,
            left: Length::Auto,
            width: Length::Auto,
            height: Length::Auto,
            min_width: Length::Auto,
            max_width: Length::Auto,
            min_height: Length::Auto,
            max_height: Length::Auto,
            box_sizing: BoxSizing::ContentBox,
            overflow: Overflow::Visible,
            font_family: None,
            font_size: Pt::new(12.0),
            font_weight: FontWeight::Normal,
            line_height: 1.5,
            text_align: TextAlign::Start,
            color: Color::BLACK,
            text_decoration: TextDecoration::None,
            white_space: WhiteSpace::Normal,
            letter_spacing: Pt::ZERO,
            word_spacing: Pt::ZERO,
            text_indent: Pt::ZERO,
            background_color: None,
            opacity: 1.0,
            margin: Edges::ZERO,
            padding: Edges::ZERO,
            border: Border::NONE,
            border_radius: Pt::ZERO,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Length::Auto,
            gap: Pt::ZERO,
            row_gap: Pt::ZERO,
            column_gap: Pt::ZERO,
            list_style_type: ListStyleType::Disc,
            list_style_position_inside: false,
            page_break_before: false,
            page_break_after: false,
        }
    }
}
