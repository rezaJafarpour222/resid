use super::{
    edges::Edges,
    rules::{Declaration, Property, StyleRule, Value},
    selector::{CompoundSelector, Selector},
    types::{Border, Color, Display, FontWeight, TextAlign},
};
use crate::units::{Direction, Pt};

pub struct CssParser;

impl CssParser {
    pub fn parse_stylesheet(input: &str) -> Result<Vec<StyleRule>, String> {
        let input = strip_comments(input);
        let mut rules = Vec::new();
        let mut source_order = 0;

        for raw_rule in input.split('}') {
            let Some((selector_text, declarations_text)) = raw_rule.split_once('{') else {
                continue;
            };

            let selector_text = selector_text.trim();
            if selector_text.is_empty() {
                continue;
            }

            let declarations = Self::parse_declarations(declarations_text)?;

            for selector_text in selector_text.split(',') {
                let selector = parse_selector(selector_text.trim())?;

                rules.push(StyleRule {
                    selector,
                    declarations: declarations.clone(),
                    source_order,
                });

                source_order += 1;
            }
        }

        Ok(rules)
    }

    pub fn parse_declarations(input: &str) -> Result<Vec<Declaration>, String> {
        let mut declarations = Vec::new();

        for raw in input.split(';') {
            let raw = raw.trim();
            if raw.is_empty() {
                continue;
            }

            let Some((property, value)) = raw.split_once(':') else {
                return Err(format!("invalid CSS declaration: {raw}"));
            };

            declarations.push(parse_declaration(property.trim(), value.trim())?);
        }

        Ok(declarations)
    }
}

fn strip_comments(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut rest = input;

    while let Some(start) = rest.find("/*") {
        result.push_str(&rest[..start]);

        let after_start = &rest[start + 2..];
        let Some(end) = after_start.find("*/") else {
            return result;
        };

        rest = &after_start[end + 2..];
    }

    result.push_str(rest);
    result
}

fn parse_selector(input: &str) -> Result<Selector, String> {
    let mut parts = Vec::new();

    for component in input.split_whitespace() {
        parts.push(parse_compound_selector(component)?);
    }

    if parts.is_empty() {
        return Err("empty selector".to_string());
    }

    Ok(Selector { parts })
}

fn parse_compound_selector(input: &str) -> Result<CompoundSelector, String> {
    if input == "*" {
        return Ok(CompoundSelector {
            tag: None,
            id: None,
            classes: Vec::new(),
        });
    }

    let chars = input.chars().collect::<Vec<_>>();
    let mut index = 0;
    let mut tag = None;
    let mut id = None;
    let mut classes = Vec::new();

    if index < chars.len() && chars[index] != '#' && chars[index] != '.' {
        let start = index;
        while index < chars.len() && chars[index] != '#' && chars[index] != '.' {
            index += 1;
        }

        tag = Some(chars[start..index].iter().collect::<String>());
    }

    while index < chars.len() {
        let marker = chars[index];
        if marker != '#' && marker != '.' {
            return Err(format!("invalid selector: {input}"));
        }

        index += 1;
        let start = index;

        while index < chars.len() && chars[index] != '#' && chars[index] != '.' {
            index += 1;
        }

        if start == index {
            return Err(format!("invalid selector: {input}"));
        }

        let value = chars[start..index].iter().collect::<String>();

        match marker {
            '#' => {
                if id.is_some() {
                    return Err(format!("multiple IDs in selector: {input}"));
                }
                id = Some(value);
            }
            '.' => classes.push(value),
            _ => unreachable!(),
        }
    }

    Ok(CompoundSelector { tag, id, classes })
}

fn parse_declaration(property: &str, value: &str) -> Result<Declaration, String> {
    let (property, value) = match property.trim().to_ascii_lowercase().as_str() {
        "display" => (Property::Display, Value::Display(parse_display(value)?)),
        "direction" => (
            Property::Direction,
            Value::Direction(parse_direction(value)?),
        ),
        "font-family" => (
            Property::FontFamily,
            Value::String(parse_font_family(value)),
        ),
        "font-size" => (Property::FontSize, Value::Pt(parse_length(value)?)),
        "font-weight" => (
            Property::FontWeight,
            Value::FontWeight(parse_font_weight(value)?),
        ),
        "line-height" => (Property::LineHeight, Value::Number(parse_number(value)?)),
        "text-align" => (
            Property::TextAlign,
            Value::TextAlign(parse_text_align(value)?),
        ),
        "color" => (Property::Color, Value::Color(parse_color(value)?)),
        "background" | "background-color" => (
            Property::BackgroundColor,
            Value::Color(parse_background_color(value)?),
        ),
        "margin" => (Property::Margin, Value::Edges(parse_edges(value)?)),
        "padding" => (Property::Padding, Value::Edges(parse_edges(value)?)),
        "border" => (Property::Border, Value::Border(parse_border(value)?)),
        _ => return Err(format!("unsupported CSS property: {property}")),
    };

    Ok(Declaration { property, value })
}

fn parse_display(value: &str) -> Result<Display, String> {
    match value.trim() {
        "block" => Ok(Display::Block),
        "inline" => Ok(Display::Inline),
        "none" => Ok(Display::None),
        _ => Err(format!("invalid display value: {value}")),
    }
}

fn parse_direction(value: &str) -> Result<Direction, String> {
    match value.trim() {
        "ltr" => Ok(Direction::LTR),
        "rtl" => Ok(Direction::RTL),
        _ => Err(format!("invalid direction value: {value}")),
    }
}

fn parse_font_weight(value: &str) -> Result<FontWeight, String> {
    match value.trim() {
        "normal" => Ok(FontWeight::Normal),
        "bold" => Ok(FontWeight::Bold),
        _ => Err(format!("invalid font-weight value: {value}")),
    }
}

fn parse_text_align(value: &str) -> Result<TextAlign, String> {
    match value.trim() {
        "start" => Ok(TextAlign::Start),
        "left" => Ok(TextAlign::Left),
        "right" => Ok(TextAlign::Right),
        "center" => Ok(TextAlign::Center),
        _ => Err(format!("invalid text-align value: {value}")),
    }
}

fn parse_font_family(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

fn parse_number(value: &str) -> Result<f32, String> {
    value
        .trim()
        .parse::<f32>()
        .map_err(|_| format!("invalid number: {value}"))
}

fn parse_length(value: &str) -> Result<Pt, String> {
    let value = value.trim();

    if let Some(number) = value.strip_suffix("pt") {
        return Ok(Pt::new(parse_number(number)?));
    }

    if let Some(number) = value.strip_suffix("px") {
        return Ok(Pt::new(parse_number(number)? * 72.0 / 96.0));
    }

    Err(format!("unsupported length unit: {value}"))
}

fn parse_edges(value: &str) -> Result<Edges, String> {
    let values = value
        .split_whitespace()
        .map(parse_length)
        .collect::<Result<Vec<_>, _>>()?;

    match values.as_slice() {
        [] => Err("empty edge value".to_string()),
        [one] => Ok(Edges::all(*one)),
        [vertical, horizontal] => Ok(Edges::vertical_horizontal(*vertical, *horizontal)),
        [top, horizontal, bottom] => Ok(Edges {
            top: *top,
            right: *horizontal,
            bottom: *bottom,
            left: *horizontal,
        }),
        [top, right, bottom, left] => Ok(Edges {
            top: *top,
            right: *right,
            bottom: *bottom,
            left: *left,
        }),
        _ => Err("invalid edge value".to_string()),
    }
}

fn parse_color(value: &str) -> Result<Color, String> {
    let value = value.trim();

    match value.to_ascii_lowercase().as_str() {
        "black" => return Ok(Color::BLACK),
        "white" => return Ok(Color::WHITE),
        "red" => return Ok(Color::rgb(255, 0, 0)),
        "green" => return Ok(Color::rgb(0, 128, 0)),
        "blue" => return Ok(Color::rgb(0, 0, 255)),
        "gray" | "grey" => return Ok(Color::rgb(128, 128, 128)),
        _ => {}
    }

    let hex = value
        .strip_prefix('#')
        .ok_or_else(|| format!("unsupported color value: {value}"))?;

    let expanded = match hex.len() {
        3 => hex.chars().flat_map(|c| [c, c]).collect::<String>(),
        6 => hex.to_string(),
        _ => return Err(format!("invalid hex color: {value}")),
    };

    let r =
        u8::from_str_radix(&expanded[0..2], 16).map_err(|_| format!("invalid color: {value}"))?;
    let g =
        u8::from_str_radix(&expanded[2..4], 16).map_err(|_| format!("invalid color: {value}"))?;
    let b =
        u8::from_str_radix(&expanded[4..6], 16).map_err(|_| format!("invalid color: {value}"))?;

    Ok(Color::rgb(r, g, b))
}

fn parse_background_color(value: &str) -> Result<Color, String> {
    let first = value.split_whitespace().next().unwrap_or(value);
    parse_color(first)
}

fn parse_border(value: &str) -> Result<Border, String> {
    let mut width = None;
    let mut color = Color::BLACK;
    let mut saw_style = false;

    for token in value.split_whitespace() {
        if token == "solid" {
            saw_style = true;
            continue;
        }

        if matches!(token, "none" | "dashed" | "dotted" | "double") {
            if token == "none" {
                return Ok(Border::NONE);
            }
            return Err(format!("unsupported border style: {token}"));
        }

        if token.ends_with("pt") || token.ends_with("px") {
            width = Some(parse_length(token)?);
            continue;
        }

        color = parse_color(token)?;
    }

    let width = width.ok_or_else(|| "border width is required".to_string())?;

    if !saw_style {
        return Err("border style must be solid".to_string());
    }

    Ok(Border::solid(width, color))
}
