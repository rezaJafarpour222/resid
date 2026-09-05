use cssparser::{AtRuleParser, Parser, ParserInput, QualifiedRuleParser, StyleSheetParser, Token};

use crate::{
    css::{
        edges::Edges,
        rules::{Declaration, Property, StyleRule, Value},
        selector::SelectorList,
        types::{
            AlignItems, Border, BoxSizing, Color, Display, FlexDirection, FlexWrap, FontWeight,
            JustifyContent, Length, ListStyleType, Overflow, Position, TextAlign, TextDecoration,
            WhiteSpace,
        },
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
                skip_to_declaration_boundary(input);
                continue;
            }
            Err(_) => break,
        };
        if input.try_parse(|i| i.expect_colon()).is_err() {
            skip_to_declaration_boundary(input);
            continue;
        }
        let value_start = input.position();
        let mut saw_semicolon = false;
        while !input.is_exhausted() {
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
        if let Some((property, value)) = parse_declaration(&name, raw_value) {
            declarations.push(Declaration {
                property,
                value,
                important,
            });
        }
    }
    Ok(declarations)
}

fn skip_to_declaration_boundary<'i, 't>(input: &mut Parser<'i, 't>) {
    while !input.is_exhausted() {
        match input.next() {
            Ok(Token::Semicolon) | Err(_) => break,
            Ok(_) => {}
        }
    }
}

fn strip_important(value: &str) -> (&str, bool) {
    let trimmed = value.trim();
    if trimmed.len() >= 10 && trimmed[trimmed.len() - 10..].eq_ignore_ascii_case("!important") {
        (trimmed[..trimmed.len() - 10].trim_end(), true)
    } else {
        (trimmed, false)
    }
}

fn parse_declaration(name: &str, value: &str) -> Option<(Property, Value)> {
    let property = match name.to_ascii_lowercase().as_str() {
        "display" => Property::Display,
        "direction" => Property::Direction,
        "position" => Property::Position,
        "top" => Property::Top,
        "right" => Property::Right,
        "bottom" => Property::Bottom,
        "left" => Property::Left,
        "width" => Property::Width,
        "height" => Property::Height,
        "min-width" => Property::MinWidth,
        "max-width" => Property::MaxWidth,
        "min-height" => Property::MinHeight,
        "max-height" => Property::MaxHeight,
        "box-sizing" => Property::BoxSizing,
        "overflow" => Property::Overflow,
        "font-family" => Property::FontFamily,
        "font-size" => Property::FontSize,
        "font-weight" => Property::FontWeight,
        "line-height" => Property::LineHeight,
        "text-align" => Property::TextAlign,
        "color" => Property::Color,
        "text-decoration" => Property::TextDecoration,
        "white-space" => Property::WhiteSpace,
        "letter-spacing" => Property::LetterSpacing,
        "word-spacing" => Property::WordSpacing,
        "text-indent" => Property::TextIndent,
        "background" | "background-color" => Property::BackgroundColor,
        "opacity" => Property::Opacity,
        "margin" => Property::Margin,
        "padding" => Property::Padding,
        "border" => Property::Border,
        "border-radius" => Property::BorderRadius,
        "flex-direction" => Property::FlexDirection,
        "flex-wrap" => Property::FlexWrap,
        "justify-content" => Property::JustifyContent,
        "align-items" => Property::AlignItems,
        "flex-grow" => Property::FlexGrow,
        "flex-shrink" => Property::FlexShrink,
        "flex-basis" => Property::FlexBasis,
        "flex" => Property::Flex,
        "gap" => Property::Gap,
        "row-gap" => Property::RowGap,
        "column-gap" => Property::ColumnGap,
        "list-style-type" => Property::ListStyleType,
        "list-style-position" => Property::ListStylePosition,
        "list-style" => Property::ListStyleType,
        "page-break-before" => Property::PageBreakBefore,
        "page-break-after" => Property::PageBreakAfter,
        "break-before" => Property::PageBreakBefore,
        "break-after" => Property::PageBreakAfter,
        _ => return None,
    };
    let parsed = match property {
        Property::Display => parse_display(value).map(Value::Display),
        Property::Direction => parse_direction(value).map(Value::Direction),
        Property::Position => parse_position(value).map(Value::Position),
        Property::Top
        | Property::Right
        | Property::Bottom
        | Property::Left
        | Property::Width
        | Property::Height
        | Property::MinWidth
        | Property::MaxWidth
        | Property::MinHeight
        | Property::MaxHeight
        | Property::FlexBasis
        | Property::BorderRadius => parse_length(value).map(Value::Length),
        Property::BoxSizing => parse_box_sizing(value).map(Value::BoxSizing),
        Property::Overflow => parse_overflow(value).map(Value::Overflow),
        Property::FontFamily => parse_font_family(value).map(Value::FontFamily),
        Property::FontSize => parse_length(value).and_then(|v| match v {
            Length::Pt(p) => Some(Value::FontSize(p)),
            Length::Percent(p) => Some(Value::FontSize(Pt::new(12.0 * p / 100.0))),
            Length::Auto => None,
        }),
        Property::FontWeight => parse_font_weight(value).map(Value::FontWeight),
        Property::LineHeight => parse_line_height(value).map(Value::Number),
        Property::TextAlign => parse_text_align(value).map(Value::TextAlign),
        Property::Color | Property::BackgroundColor => parse_color(value).map(Value::Color),
        Property::TextDecoration => parse_text_decoration(value).map(Value::TextDecoration),
        Property::WhiteSpace => parse_white_space(value).map(Value::WhiteSpace),
        Property::LetterSpacing | Property::WordSpacing | Property::TextIndent => {
            parse_length(value).map(Value::Length)
        }
        Property::Opacity | Property::FlexGrow | Property::FlexShrink => {
            parse_number(value).map(Value::Number)
        }
        Property::Margin | Property::Padding => parse_edges(value).map(Value::Edges),
        Property::Border => parse_border(value).map(Value::Border),
        Property::FlexDirection => parse_flex_direction(value).map(Value::FlexDirection),
        Property::FlexWrap => parse_flex_wrap(value).map(Value::FlexWrap),
        Property::JustifyContent => parse_justify(value).map(Value::JustifyContent),
        Property::AlignItems => parse_align(value).map(Value::AlignItems),
        Property::Flex => parse_flex(value).map(|(grow, shrink, basis)| Value::Flex {
            grow,
            shrink,
            basis,
        }),
        Property::Gap | Property::RowGap | Property::ColumnGap => {
            parse_length(value).and_then(|l| match l {
                Length::Pt(p) => Some(Value::Length(Length::Pt(p))),
                Length::Percent(_) | Length::Auto => None,
            })
        }
        Property::ListStyleType => parse_list_style(value).map(Value::ListStyleType),
        Property::ListStylePosition => parse_inside(value).map(Value::Bool),
        Property::PageBreakBefore | Property::PageBreakAfter => parse_break(value).map(Value::Bool),
    }?;
    Some((property, parsed))
}

fn one_token(input: &str) -> Option<Token<'_>> {
    let mut state = ParserInput::new(input);
    let mut parser = Parser::new(&mut state);
    let token = parser.next().ok()?.clone();
    parser.expect_exhausted().ok()?;
    Some(token)
}
fn ident(input: &str) -> Option<String> {
    match one_token(input)? {
        Token::Ident(x) => Some(x.as_ref().to_owned()),
        _ => None,
    }
}
fn parse_number(v: &str) -> Option<f32> {
    match one_token(v)? {
        Token::Number { value, .. } => Some(value),
        _ => None,
    }
}
fn parse_display(v: &str) -> Option<Display> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "block" => Some(Display::Block),
        "inline" => Some(Display::Inline),
        "inline-block" => Some(Display::InlineBlock),
        "flex" => Some(Display::Flex),
        "table" => Some(Display::Table),
        "table-row" => Some(Display::TableRow),
        "table-cell" => Some(Display::TableCell),
        "list-item" => Some(Display::ListItem),
        "none" => Some(Display::None),
        _ => None,
    }
}
fn parse_direction(v: &str) -> Option<Direction> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "rtl" => Some(Direction::RTL),
        "ltr" => Some(Direction::LTR),
        _ => None,
    }
}
fn parse_position(v: &str) -> Option<Position> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "static" => Some(Position::Static),
        "relative" => Some(Position::Relative),
        "absolute" => Some(Position::Absolute),
        "fixed" => Some(Position::Fixed),
        _ => None,
    }
}
fn parse_box_sizing(v: &str) -> Option<BoxSizing> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "content-box" => Some(BoxSizing::ContentBox),
        "border-box" => Some(BoxSizing::BorderBox),
        _ => None,
    }
}
fn parse_overflow(v: &str) -> Option<Overflow> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "visible" => Some(Overflow::Visible),
        "hidden" => Some(Overflow::Hidden),
        _ => None,
    }
}
fn parse_text_decoration(v: &str) -> Option<TextDecoration> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "none" => Some(TextDecoration::None),
        "underline" => Some(TextDecoration::Underline),
        "line-through" => Some(TextDecoration::LineThrough),
        _ => None,
    }
}
fn parse_white_space(v: &str) -> Option<WhiteSpace> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "normal" => Some(WhiteSpace::Normal),
        "nowrap" => Some(WhiteSpace::NoWrap),
        "pre" => Some(WhiteSpace::Pre),
        "pre-wrap" => Some(WhiteSpace::PreWrap),
        _ => None,
    }
}
fn parse_flex_direction(v: &str) -> Option<FlexDirection> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "row" => Some(FlexDirection::Row),
        "row-reverse" => Some(FlexDirection::RowReverse),
        "column" => Some(FlexDirection::Column),
        "column-reverse" => Some(FlexDirection::ColumnReverse),
        _ => None,
    }
}
fn parse_flex_wrap(v: &str) -> Option<FlexWrap> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "nowrap" => Some(FlexWrap::NoWrap),
        "wrap" => Some(FlexWrap::Wrap),
        _ => None,
    }
}
fn parse_justify(v: &str) -> Option<JustifyContent> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "flex-start" => Some(JustifyContent::FlexStart),
        "flex-end" => Some(JustifyContent::FlexEnd),
        "center" => Some(JustifyContent::Center),
        "space-between" => Some(JustifyContent::SpaceBetween),
        "space-around" => Some(JustifyContent::SpaceAround),
        "space-evenly" => Some(JustifyContent::SpaceEvenly),
        _ => None,
    }
}
fn parse_align(v: &str) -> Option<AlignItems> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "flex-start" => Some(AlignItems::FlexStart),
        "flex-end" => Some(AlignItems::FlexEnd),
        "center" => Some(AlignItems::Center),
        "stretch" => Some(AlignItems::Stretch),
        _ => None,
    }
}
fn parse_flex(v: &str) -> Option<(f32, f32, Length)> {
    let value = v.trim();
    if value.eq_ignore_ascii_case("none") {
        return Some((0.0, 0.0, Length::Auto));
    }
    if value.eq_ignore_ascii_case("auto") {
        return Some((1.0, 1.0, Length::Auto));
    }
    let parts = value.split_whitespace().collect::<Vec<_>>();
    match parts.as_slice() {
        [a] => {
            let grow = parse_number(a)?;
            Some((grow, 1.0, Length::Pt(Pt::ZERO)))
        }
        [a, b] => {
            let grow = parse_number(a)?;
            if let Some(shrink) = parse_number(b) {
                Some((grow, shrink, Length::Pt(Pt::ZERO)))
            } else {
                Some((grow, 1.0, parse_length(b)?))
            }
        }
        [a, b, c] => Some((parse_number(a)?, parse_number(b)?, parse_length(c)?)),
        _ => None,
    }
}
fn parse_list_style(v: &str) -> Option<ListStyleType> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "none" => Some(ListStyleType::None),
        "disc" => Some(ListStyleType::Disc),
        "circle" => Some(ListStyleType::Circle),
        "square" => Some(ListStyleType::Square),
        "decimal" => Some(ListStyleType::Decimal),
        _ => None,
    }
}
fn parse_inside(v: &str) -> Option<bool> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "inside" => Some(true),
        "outside" => Some(false),
        _ => None,
    }
}
fn parse_break(v: &str) -> Option<bool> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "always" => Some(true),
        "page" => Some(true),
        "auto" | "avoid" | "left" | "right" => Some(false),
        _ => None,
    }
}
fn parse_font_family(v: &str) -> Option<String> {
    let t = v.trim();
    if t.is_empty() {
        None
    } else {
        Some(
            t.split(',')
                .next()?
                .trim()
                .trim_matches(['\'', '\"'])
                .to_owned(),
        )
    }
}
fn parse_length(v: &str) -> Option<Length> {
    match one_token(v)? {
        Token::Number { value, .. } => {
            if value == 0.0 {
                Some(Length::Pt(Pt::ZERO))
            } else {
                None
            }
        }
        Token::Dimension { value, unit, .. } => {
            convert_length(value, unit.as_ref()).map(Length::Pt)
        }
        Token::Percentage { unit_value, .. } => Some(Length::Percent(unit_value)),
        Token::Ident(name) if name.eq_ignore_ascii_case("auto") => Some(Length::Auto),
        _ => None,
    }
}
fn convert_length(v: f32, u: &str) -> Option<Pt> {
    Some(Pt::new(match u.to_ascii_lowercase().as_str() {
        "pt" => v,
        "px" => v * 0.75,
        "in" => v * 72.0,
        "cm" => v * 72.0 / 2.54,
        "mm" => v * 72.0 / 25.4,
        "pc" => v * 12.0,
        "q" => v * 72.0 / 101.6,
        _ => return None,
    }))
}
fn parse_font_weight(v: &str) -> Option<FontWeight> {
    match one_token(v)? {
        Token::Ident(n) if n.eq_ignore_ascii_case("normal") => Some(FontWeight::Normal),
        Token::Ident(n) if n.eq_ignore_ascii_case("bold") => Some(FontWeight::Bold),
        Token::Number { value, .. } if value < 500.0 => Some(FontWeight::Normal),
        Token::Number { value, .. } => {
            if value >= 500.0 {
                Some(FontWeight::Bold)
            } else {
                None
            }
        }
        _ => None,
    }
}
fn parse_line_height(v: &str) -> Option<f32> {
    match one_token(v)? {
        Token::Number { value, .. } => Some(value),
        Token::Dimension { value, unit, .. } => {
            Some(convert_length(value, unit.as_ref())?.value() / 12.0)
        }
        _ => None,
    }
}
fn parse_text_align(v: &str) -> Option<TextAlign> {
    match ident(v)?.to_ascii_lowercase().as_str() {
        "start" => Some(TextAlign::Start),
        "left" => Some(TextAlign::Left),
        "right" => Some(TextAlign::Right),
        "center" => Some(TextAlign::Center),
        "justify" => Some(TextAlign::Justify),
        _ => None,
    }
}
fn parse_color(v: &str) -> Option<Color> {
    let t = v.trim();
    if let Some(h) = t.strip_prefix('#') {
        let b = h.as_bytes();
        match h.len() {
            3 => Some(Color::rgb(
                hex(b[0])? * 17,
                hex(b[1])? * 17,
                hex(b[2])? * 17,
            )),
            6 => Some(Color::rgb(
                (hex(b[0])? * 16) + hex(b[1])?,
                (hex(b[2])? * 16) + hex(b[3])?,
                (hex(b[4])? * 16) + hex(b[5])?,
            )),
            8 => Some(Color::rgb(
                (hex(b[0])? * 16) + hex(b[1])?,
                (hex(b[2])? * 16) + hex(b[3])?,
                (hex(b[4])? * 16) + hex(b[5])?,
            )),
            _ => None,
        }
    } else {
        let low = t.to_ascii_lowercase();
        if let Some(args) = low.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
            let n = args
                .split(',')
                .map(|x| x.trim().parse::<u8>().ok())
                .collect::<Option<Vec<_>>>()?;
            return (n.len() == 3).then(|| Color::rgb(n[0], n[1], n[2]));
        }
        match low.as_str() {
            "black" => Some(Color::BLACK),
            "white" => Some(Color::WHITE),
            "red" => Some(Color::rgb(255, 0, 0)),
            "green" => Some(Color::rgb(0, 128, 0)),
            "blue" => Some(Color::rgb(0, 0, 255)),
            "yellow" => Some(Color::rgb(255, 255, 0)),
            "orange" => Some(Color::rgb(255, 165, 0)),
            "purple" => Some(Color::rgb(128, 0, 128)),
            "pink" => Some(Color::rgb(255, 192, 203)),
            "brown" => Some(Color::rgb(165, 42, 42)),
            "gray" | "grey" => Some(Color::rgb(128, 128, 128)),
            "silver" => Some(Color::rgb(192, 192, 192)),
            "navy" => Some(Color::rgb(0, 0, 128)),
            "teal" => Some(Color::rgb(0, 128, 128)),
            "transparent" => Some(Color::WHITE),
            _ => None,
        }
    }
}
fn hex(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}
fn parse_edges(v: &str) -> Option<Edges> {
    let mut state = ParserInput::new(v);
    let mut parser = Parser::new(&mut state);
    let mut vals = Vec::new();
    while !parser.is_exhausted() {
        match parser.next().ok()? {
            Token::Number { value, .. } => {
                if *value != 0.0 {
                    return None;
                } else {
                    vals.push(Pt::ZERO)
                }
            }
            Token::Dimension { value, unit, .. } => {
                vals.push(convert_length(*value, unit.as_ref())?)
            }
            Token::WhiteSpace(_) => {}
            _ => return None,
        }
        if vals.len() > 4 {
            return None;
        }
    }
    match vals.as_slice() {
        [a] => Some(Edges::all(*a)),
        [v, h] => Some(Edges::vertical_horizontal(*v, *h)),
        [t, h, b] => Some(Edges {
            top: *t,
            right: *h,
            bottom: *b,
            left: *h,
        }),
        [t, r, b, l] => Some(Edges {
            top: *t,
            right: *r,
            bottom: *b,
            left: *l,
        }),
        _ => None,
    }
}
fn parse_border(v: &str) -> Option<Border> {
    let mut state = ParserInput::new(v);
    let mut parser = Parser::new(&mut state);
    let mut width = None;
    let mut color = None;
    while !parser.is_exhausted() {
        match parser.next().ok()? {
            Token::Dimension { value, unit, .. } => {
                width = Some(convert_length(*value, unit.as_ref())?)
            }
            Token::Number { value, .. } if *value == 0.0 => width = Some(Pt::ZERO),
            Token::Ident(name) => color = color.or_else(|| parse_color(name.as_ref())),
            _ => {}
        }
    }
    Some(Border::solid(
        width.unwrap_or(Pt::new(1.0)),
        color.unwrap_or(Color::BLACK),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_extended_properties() {
        let d=CssParser::parse_declarations("display:flex; width:50%; padding:2pt 4pt; color:rgb(10,20,30); justify-content:space-between; white-space:nowrap").unwrap();
        assert_eq!(d.len(), 6);
    }
}
