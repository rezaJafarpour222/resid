use super::{
    rules::{Declaration, Property, StyleRule, Value},
    selector::{DomElement, matches_selector_list, selector_list_specificity},
    types::*,
};
use crate::{css::parser::CssParser, html::types::Element};
use std::collections::HashMap;
#[derive(Clone, Debug)]
struct Winner {
    declaration: Declaration,
    important: bool,
    specificity: u32,
    order: usize,
}
pub fn compute_style(
    dom: &DomElement<'_>,
    element: &Element,
    parent: Option<&ComputedStyle>,
    rules: &[StyleRule],
    mut base: ComputedStyle,
) -> ComputedStyle {
    inherit(&mut base, parent);
    let mut winners: HashMap<Property, Winner> = HashMap::new();
    for rule in rules {
        if !matches_selector_list(&rule.selector, dom) {
            continue;
        }
        let spec = selector_list_specificity(&rule.selector);
        for d in &rule.declarations {
            candidate(&mut winners, d, spec, rule.source_order);
        }
    }
    if let Some(inline) = element.attribute("style") {
        if let Ok(ds) = CssParser::parse_declarations(inline) {
            for d in &ds {
                candidate(&mut winners, d, u32::MAX, usize::MAX);
            }
        }
    }
    for w in winners.values() {
        apply(&mut base, &w.declaration);
    }
    base
}
fn candidate(map: &mut HashMap<Property, Winner>, d: &Declaration, spec: u32, order: usize) {
    let c = Winner {
        declaration: d.clone(),
        important: d.important,
        specificity: spec,
        order,
    };
    let replace = map.get(&d.property).map_or(true, |x| {
        (c.important, c.specificity, c.order) > (x.important, x.specificity, x.order)
    });
    if replace {
        map.insert(d.property, c);
    }
}
fn inherit(s: &mut ComputedStyle, p: Option<&ComputedStyle>) {
    if let Some(p) = p {
        s.direction = p.direction;
        s.font_family = p.font_family.clone();
        s.font_size = p.font_size;
        s.font_weight = p.font_weight;
        s.line_height = p.line_height;
        s.text_align = p.text_align;
        s.color = p.color;
        s.text_decoration = p.text_decoration;
        s.white_space = p.white_space;
        s.letter_spacing = p.letter_spacing;
        s.word_spacing = p.word_spacing;
        s.opacity = p.opacity;
    }
}
fn apply(s: &mut ComputedStyle, d: &Declaration) {
    match (&d.property, &d.value) {
        (Property::Display, Value::Display(v)) => s.display = *v,
        (Property::Direction, Value::Direction(v)) => s.direction = *v,
        (Property::Position, Value::Position(v)) => s.position = *v,
        (Property::Top, Value::Length(v)) => s.top = *v,
        (Property::Right, Value::Length(v)) => s.right = *v,
        (Property::Bottom, Value::Length(v)) => s.bottom = *v,
        (Property::Left, Value::Length(v)) => s.left = *v,
        (Property::Width, Value::Length(v)) => s.width = *v,
        (Property::Height, Value::Length(v)) => s.height = *v,
        (Property::MinWidth, Value::Length(v)) => s.min_width = *v,
        (Property::MaxWidth, Value::Length(v)) => s.max_width = *v,
        (Property::MinHeight, Value::Length(v)) => s.min_height = *v,
        (Property::MaxHeight, Value::Length(v)) => s.max_height = *v,
        (Property::BoxSizing, Value::BoxSizing(v)) => s.box_sizing = *v,
        (Property::Overflow, Value::Overflow(v)) => s.overflow = *v,
        (Property::FontFamily, Value::FontFamily(v)) => s.font_family = Some(v.clone()),
        (Property::FontSize, Value::FontSize(v)) => s.font_size = *v,
        (Property::FontWeight, Value::FontWeight(v)) => s.font_weight = *v,
        (Property::LineHeight, Value::Number(v)) => s.line_height = *v,
        (Property::TextAlign, Value::TextAlign(v)) => s.text_align = *v,
        (Property::Color, Value::Color(v)) => s.color = *v,
        (Property::TextDecoration, Value::TextDecoration(v)) => s.text_decoration = *v,
        (Property::WhiteSpace, Value::WhiteSpace(v)) => s.white_space = *v,
        (Property::LetterSpacing, Value::Length(Length::Pt(v))) => s.letter_spacing = *v,
        (Property::WordSpacing, Value::Length(Length::Pt(v))) => s.word_spacing = *v,
        (Property::TextIndent, Value::Length(Length::Pt(v))) => s.text_indent = *v,
        (Property::BackgroundColor, Value::Color(v)) => s.background_color = Some(*v),
        (Property::Opacity, Value::Number(v)) => s.opacity = v.clamp(0.0, 1.0),
        (Property::Margin, Value::Edges(v)) => s.margin = *v,
        (Property::Padding, Value::Edges(v)) => s.padding = *v,
        (Property::Border, Value::Border(v)) => s.border = *v,
        (Property::BorderRadius, Value::Length(Length::Pt(v))) => s.border_radius = *v,
        (Property::FlexDirection, Value::FlexDirection(v)) => s.flex_direction = *v,
        (Property::FlexWrap, Value::FlexWrap(v)) => s.flex_wrap = *v,
        (Property::JustifyContent, Value::JustifyContent(v)) => s.justify_content = *v,
        (Property::AlignItems, Value::AlignItems(v)) => s.align_items = *v,
        (Property::FlexGrow, Value::Number(v)) => s.flex_grow = *v,
        (Property::FlexShrink, Value::Number(v)) => s.flex_shrink = *v,
        (Property::FlexBasis, Value::Length(v)) => s.flex_basis = *v,
        (Property::Gap, Value::Length(Length::Pt(v))) => {
            s.gap = *v;
            s.row_gap = *v;
            s.column_gap = *v
        }
        (Property::RowGap, Value::Length(Length::Pt(v))) => s.row_gap = *v,
        (Property::ColumnGap, Value::Length(Length::Pt(v))) => s.column_gap = *v,
        (
            Property::Flex,
            Value::Flex {
                grow,
                shrink,
                basis,
            },
        ) => {
            s.flex_grow = *grow;
            s.flex_shrink = *shrink;
            s.flex_basis = *basis
        }
        (Property::ListStyleType, Value::ListStyleType(v)) => s.list_style_type = *v,
        (Property::ListStylePosition, Value::Bool(v)) => s.list_style_position_inside = *v,
        (Property::PageBreakBefore, Value::Bool(v)) => s.page_break_before = *v,
        (Property::PageBreakAfter, Value::Bool(v)) => s.page_break_after = *v,
        _ => {}
    }
}
