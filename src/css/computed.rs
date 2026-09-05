use std::collections::HashMap;

use crate::{css::parser::CssParser, html::types::Element};

use super::{
    rules::{Declaration, Property, StyleRule, Value},
    selector::{DomElement, matches_selector_list, selector_list_specificity},
    types::ComputedStyle,
};

#[derive(Debug, Clone)]
struct CascadedDeclaration {
    declaration: Declaration,
    important: bool,
    specificity: u32,
    source_order: usize,
}

pub fn compute_style(
    dom_element: &DomElement<'_>,
    element: &Element,
    parent: Option<&ComputedStyle>,
    rules: &[StyleRule],
    mut base: ComputedStyle,
) -> ComputedStyle {
    inherit_properties(&mut base, parent);

    let mut winners: HashMap<Property, CascadedDeclaration> = HashMap::new();

    for rule in rules {
        if !matches_selector_list(&rule.selector, dom_element) {
            continue;
        }

        let specificity = selector_list_specificity(&rule.selector);
        for declaration in &rule.declarations {
            apply_candidate(
                &mut winners,
                declaration,
                declaration.important,
                specificity,
                rule.source_order,
            );
        }
    }

    if let Some(inline_style) = element.attribute("style") {
        if let Ok(declarations) = CssParser::parse_declarations(inline_style) {
            for declaration in declarations {
                apply_candidate(
                    &mut winners,
                    &declaration,
                    declaration.important,
                    u32::MAX,
                    usize::MAX,
                );
            }
        }
    }

    for winner in winners.values() {
        apply_declaration(&mut base, &winner.declaration);
    }

    base
}

fn apply_candidate(
    winners: &mut HashMap<Property, CascadedDeclaration>,
    declaration: &Declaration,
    important: bool,
    specificity: u32,
    source_order: usize,
) {
    let candidate = CascadedDeclaration {
        declaration: declaration.clone(),
        important,
        specificity,
        source_order,
    };

    let replace = match winners.get(&declaration.property) {
        None => true,
        Some(current) => {
            (
                candidate.important,
                candidate.specificity,
                candidate.source_order,
            ) > (current.important, current.specificity, current.source_order)
        }
    };

    if replace {
        winners.insert(declaration.property, candidate);
    }
}

fn inherit_properties(style: &mut ComputedStyle, parent: Option<&ComputedStyle>) {
    let Some(parent) = parent else {
        return;
    };

    style.direction = parent.direction;
    style.font_family = parent.font_family.clone();
    style.font_size = parent.font_size;
    style.font_weight = parent.font_weight;
    style.line_height = parent.line_height;
    style.text_align = parent.text_align;
    style.color = parent.color;
}

fn apply_declaration(style: &mut ComputedStyle, declaration: &Declaration) {
    match (&declaration.property, &declaration.value) {
        (Property::Display, Value::Display(value)) => style.display = *value,
        (Property::Direction, Value::Direction(value)) => style.direction = *value,
        (Property::FontFamily, Value::String(value)) => style.font_family = Some(value.clone()),
        (Property::FontSize, Value::Pt(value)) => style.font_size = *value,
        (Property::FontWeight, Value::FontWeight(value)) => style.font_weight = *value,
        (Property::LineHeight, Value::Number(value)) => style.line_height = *value,
        (Property::TextAlign, Value::TextAlign(value)) => style.text_align = *value,
        (Property::Color, Value::Color(value)) => style.color = *value,
        (Property::BackgroundColor, Value::Color(value)) => style.background_color = Some(*value),
        (Property::Margin, Value::Edges(value)) => style.margin = *value,
        (Property::Padding, Value::Edges(value)) => style.padding = *value,
        (Property::Border, Value::Border(value)) => style.border = *value,
        _ => {}
    }
}
