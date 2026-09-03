use super::{
    edges::Edges,
    selector::Selector,
    types::{Border, Color, Display, FontWeight, TextAlign},
};
use crate::units::{Direction, Pt};

#[derive(Debug, Clone, PartialEq)]
pub struct StyleRule {
    pub selector: Selector,
    pub declarations: Vec<Declaration>,
    pub source_order: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Property {
    Display,
    Direction,
    FontFamily,
    FontSize,
    FontWeight,
    LineHeight,
    TextAlign,
    Color,
    BackgroundColor,
    Margin,
    Padding,
    Border,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Display(Display),
    Direction(Direction),
    String(String),
    Pt(Pt),
    FontWeight(FontWeight),
    Number(f32),
    TextAlign(TextAlign),
    Color(Color),
    Edges(Edges),
    Border(Border),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub property: Property,
    pub value: Value,
}
