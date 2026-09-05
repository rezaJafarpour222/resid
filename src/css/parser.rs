use cssparser::{AtRuleParser, Parser, ParserInput, QualifiedRuleParser, StyleSheetParser, Token};

use crate::{
    css::{
        edges::Edges,
        rules::{Declaration, Property, StyleRule, Value},
        selector::{SelectorList, parse_selector_list},
        types::{Border, Color, Display, FontWeight, TextAlign},
    },
    error::AppError,
    units::{Direction, Pt},
};

pub struct CssParser;

struct StylesheetRuleParser {
    source_order: usize,
}

impl<'i> AtRuleParser<'i> for StylesheetRuleParser {
    type Prelude = ();
    type AtRule = StyleRule;
    type Error = String;
}

impl<'i> QualifiedRuleParser<'i> for StylesheetRuleParser {
    type Prelude = SelectorList;
    type QualifiedRule = StyleRule;
    type Error = String;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, cssparser::ParseError<'i, Self::Error>> {
        let selector_parser = super::selector::PdfSelectorParser;

        SelectorList::parse(
            &selector_parser,
            input,
            selectors::parser::ParseRelative::No,
        )
        .map_err(|error| input.new_custom_error(format!("{error:?}")))
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &cssparser::ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, cssparser::ParseError<'i, Self::Error>> {
        let declarations =
            parse_declarations_from_parser(input).map_err(|error| input.new_custom_error(error))?;

        let source_order = self.source_order;
        self.source_order += 1;

        Ok(StyleRule {
            selector: prelude,
            declarations,
            source_order,
        })
    }
}

impl CssParser {
    pub fn parse_stylesheet(input: &str) -> Result<Vec<StyleRule>, AppError> {
        let mut input_state = ParserInput::new(input);
        let mut parser = Parser::new(&mut input_state);
        let mut rule_parser = StylesheetRuleParser { source_order: 0 };
        let mut rules = Vec::new();

        for result in StyleSheetParser::new(&mut parser, &mut rule_parser) {
            if let Ok(rule) = result {
                rules.push(rule);
            }
        }

        Ok(rules)
    }

    pub fn parse_declarations(input: &str) -> Result<Vec<Declaration>, String> {
        let mut input_state = ParserInput::new(input);
        let mut parser = Parser::new(&mut input_state);
        parse_declarations_from_parser(&mut parser)
    }

    #[allow(dead_code)]
    pub fn parse(input: &str) -> Result<Vec<StyleRule>, AppError> {
        Self::parse_stylesheet(input)
    }
}

fn parse_declarations_from_parser<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Vec<Declaration>, String> {
    let mut declarations = Vec::new();

    while !input.is_exhausted() {
        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }

        let name = match input.next() {
            Ok(Token::Ident(name)) => name.as_ref().to_owned(),
            Ok(_) => {
                skip_to_declaration_boundary(input)?;
                continue;
            }
            Err(_) => break,
        };

        if input.try_parse(|i| i.expect_colon()).is_err() {
            skip_to_declaration_boundary(input)?;
            continue;
        }

        let value_start = input.position();
        let mut saw_semicolon = false;
        loop {
            if input.is_exhausted() {
                break;
            }

            match input.next() {
                Ok(Token::Semicolon) => {
                    saw_semicolon = true;
                    break;
                }
                Ok(_) => {}
                Err(_) => break,
            }
        }

        let mut raw_value = input.slice_from(value_start).trim().to_owned();
        if saw_semicolon {
            raw_value = raw_value.trim_end_matches(';').trim().to_owned();
        }

        let (raw_value, important) = strip_important(&raw_value);

        if let Some(declaration) = parse_declaration(&name, raw_value) {
            declarations.push(Declaration {
                property: declaration.0,
                value: declaration.1,
                important,
            });
        }
    }

    Ok(declarations)
}

fn skip_to_declaration_boundary<'i, 't>(input: &mut Parser<'i, 't>) -> Result<(), String> {
    while !input.is_exhausted() {
        match input.next() {
            Ok(Token::Semicolon) => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }
    Ok(())
}

fn strip_important(value: &str) -> (&str, bool) {
    let trimmed = value.trim();
    let suffix = "!important";
    if trimmed.len() >= suffix.len()
        && trimmed[trimmed.len() - suffix.len()..].eq_ignore_ascii_case(suffix)
    {
        (trimmed[..trimmed.len() - suffix.len()].trim_end(), true)
    } else {
        (trimmed, false)
    }
}

fn parse_declaration(name: &str, value: &str) -> Option<(Property, Value)> {
    let property = match name.to_ascii_lowercase().as_str() {
        "display" => Property::Display,
        "direction" => Property::Direction,
        "font-family" => Property::FontFamily,
        "font-size" => Property::FontSize,
        "font-weight" => Property::FontWeight,
        "line-height" => Property::LineHeight,
        "text-align" => Property::TextAlign,
        "color" => Property::Color,
        "background" | "background-color" => Property::BackgroundColor,
        "margin" => Property::Margin,
        "padding" => Property::Padding,
        "border" => Property::Border,
        _ => return None,
    };

    let parsed = match property {
        Property::Display => parse_display(value).map(Value::Display),
        Property::Direction => parse_direction(value).map(Value::Direction),
        Property::FontFamily => parse_font_family(value).map(Value::String),
        Property::FontSize => parse_length(value).map(Value::Pt),
        Property::FontWeight => parse_font_weight(value).map(Value::FontWeight),
        Property::LineHeight => parse_line_height(value).map(|number| Value::Number(number)),
        Property::TextAlign => parse_text_align(value).map(Value::TextAlign),
        Property::Color => parse_color(value).map(Value::Color),
        Property::BackgroundColor => parse_color(value).map(Value::Color),
        Property::Margin | Property::Padding => parse_edges(value).map(Value::Edges),
        Property::Border => parse_border(value).map(Value::Border),
    }?;

    Some((property, parsed))
}

fn one_token(input: &str) -> Option<Token<'_>> {
    let mut input_state = ParserInput::new(input);
    let mut parser = Parser::new(&mut input_state);
    let token = parser.next().ok()?.clone();
    parser.expect_exhausted().ok()?;
    Some(token)
}

fn parse_display(value: &str) -> Option<Display> {
    let token = one_token(value)?;
    match token {
        Token::Ident(name) if name.eq_ignore_ascii_case("block") => Some(Display::Block),
        Token::Ident(name) if name.eq_ignore_ascii_case("inline") => Some(Display::Inline),
        Token::Ident(name) if name.eq_ignore_ascii_case("none") => Some(Display::None),
        _ => None,
    }
}

fn parse_direction(value: &str) -> Option<Direction> {
    let token = one_token(value)?;
    match token {
        Token::Ident(name) if name.eq_ignore_ascii_case("rtl") => Some(Direction::RTL),
        Token::Ident(name) if name.eq_ignore_ascii_case("ltr") => Some(Direction::LTR),
        _ => None,
    }
}

fn parse_font_family(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    let first = value.split(',').next()?.trim();
    Some(first.trim_matches(['\'', '"']).to_owned())
}

fn parse_length(value: &str) -> Option<Pt> {
    let token = one_token(value)?;
    match token {
        Token::Number { value, .. } if value == 0.0 => Some(Pt::ZERO),
        Token::Dimension { value, unit, .. } => convert_length(value, unit.as_ref()),
        _ => None,
    }
}

fn convert_length(value: f32, unit: &str) -> Option<Pt> {
    let points = match unit.to_ascii_lowercase().as_str() {
        "pt" => value,
        "px" => value * 0.75,
        "in" => value * 72.0,
        "cm" => value * 72.0 / 2.54,
        "mm" => value * 72.0 / 25.4,
        "pc" => value * 12.0,
        _ => return None,
    };
    Some(Pt::new(points))
}

fn parse_font_weight(value: &str) -> Option<FontWeight> {
    let token = one_token(value)?;
    match token {
        Token::Ident(name) if name.eq_ignore_ascii_case("normal") => Some(FontWeight::Normal),
        Token::Ident(name) if name.eq_ignore_ascii_case("bold") => Some(FontWeight::Bold),
        Token::Number { value, .. } if (value - 400.0).abs() < f32::EPSILON => {
            Some(FontWeight::Normal)
        }
        Token::Number { value, .. } if value >= 500.0 => Some(FontWeight::Bold),
        _ => None,
    }
}

fn parse_line_height(value: &str) -> Option<f32> {
    let token = one_token(value)?;
    match token {
        Token::Number { value, .. } => Some(value),
        Token::Dimension { value, unit, .. } => {
            let points = convert_length(value, unit.as_ref())?;
            Some(points.value() / 12.0)
        }
        _ => None,
    }
}

fn parse_text_align(value: &str) -> Option<TextAlign> {
    let token = one_token(value)?;
    match token {
        Token::Ident(name) if name.eq_ignore_ascii_case("start") => Some(TextAlign::Start),
        Token::Ident(name) if name.eq_ignore_ascii_case("left") => Some(TextAlign::Left),
        Token::Ident(name) if name.eq_ignore_ascii_case("right") => Some(TextAlign::Right),
        Token::Ident(name) if name.eq_ignore_ascii_case("center") => Some(TextAlign::Center),
        _ => None,
    }
}

fn parse_color(value: &str) -> Option<Color> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
        let hex = hex.trim();
        return match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                Some(Color::rgb(r, g, b))
            }
            6 => Some(Color::rgb(
                u8::from_str_radix(&hex[0..2], 16).ok()?,
                u8::from_str_radix(&hex[2..4], 16).ok()?,
                u8::from_str_radix(&hex[4..6], 16).ok()?,
            )),
            _ => None,
        };
    }

    match value.to_ascii_lowercase().as_str() {
        "black" => Some(Color::BLACK),
        "white" => Some(Color::WHITE),
        "red" => Some(Color::rgb(255, 0, 0)),
        "green" => Some(Color::rgb(0, 128, 0)),
        "blue" => Some(Color::rgb(0, 0, 255)),
        "yellow" => Some(Color::rgb(255, 255, 0)),
        "gray" | "grey" => Some(Color::rgb(128, 128, 128)),
        "transparent" => Some(Color::rgb(255, 255, 255)),
        _ => None,
    }
}

fn parse_edges(value: &str) -> Option<Edges> {
    let mut input_state = ParserInput::new(value);
    let mut parser = Parser::new(&mut input_state);
    let mut values = Vec::with_capacity(4);

    while !parser.is_exhausted() {
        values.push(parse_length_from_parser(&mut parser)?);
        if values.len() > 4 {
            return None;
        }
    }

    match values.as_slice() {
        [a] => Some(Edges::all(*a)),
        [vertical, horizontal] => Some(Edges::vertical_horizontal(*vertical, *horizontal)),
        [top, horizontal, bottom] => Some(Edges {
            top: *top,
            right: *horizontal,
            bottom: *bottom,
            left: *horizontal,
        }),
        [top, right, bottom, left] => Some(Edges {
            top: *top,
            right: *right,
            bottom: *bottom,
            left: *left,
        }),
        _ => None,
    }
}

fn parse_length_from_parser<'i, 't>(parser: &mut Parser<'i, 't>) -> Option<Pt> {
    match parser.next().ok()? {
        Token::Number { value, .. } if *value == 0.0 => Some(Pt::ZERO),
        Token::Dimension { value, unit, .. } => convert_length(*value, unit.as_ref()),
        _ => None,
    }
}

fn parse_border(value: &str) -> Option<Border> {
    let mut input_state = ParserInput::new(value);
    let mut parser = Parser::new(&mut input_state);
    let mut width = None;
    let mut color = None;

    while !parser.is_exhausted() {
        let start = parser.position();
        let token = parser.next().ok()?;

        match token {
            Token::Dimension { value, unit, .. } => {
                if width.is_some() {
                    return None;
                }
                width = convert_length(*value, unit.as_ref());
            }
            Token::Number { value, .. } if *value == 0.0 => {
                width = Some(Pt::ZERO);
            }
            Token::Ident(name) => {
                if color.is_none() {
                    color = parse_color(name.as_ref());
                }
            }
            _ => {
                if color.is_none() {
                    let raw = parser.slice_from(start).trim();
                    color = parse_color(raw);
                }
            }
        }
    }

    Some(Border::solid(width?, color.unwrap_or(Color::BLACK)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stylesheet_parser_reads_rules_and_important() {
        let rules = CssParser::parse_stylesheet(
            r#"
            invoice > .row[data-kind="total"]:nth-child(2) {
                color: #123456 !important;
                margin: 1pt 2pt;
            }
            "#,
        )
        .unwrap();

        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].declarations.len(), 2);
        assert!(rules[0].declarations[0].important);
    }
}
