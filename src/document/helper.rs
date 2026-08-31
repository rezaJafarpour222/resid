use crate::{
    document::types::{Inline, InlineContent, LayoutText},
    error::AppError,
    font::{loader::Font, shaper::Shaper, types::ShapedText},
    units::{Direction, Pt},
};

pub fn shape_inline_content(
    content: &InlineContent,
    font: &Font,
    direction: Direction,
    font_size: Pt,
) -> Result<LayoutText, AppError> {
    let text = inline_text(content);

    let shaped = Shaper::shaped_text(font, &text, direction, font_size)?;

    Ok(LayoutText { text, shaped })
}
pub fn inline_text(content: &InlineContent) -> String {
    content
        .items
        .iter()
        .map(|item| match item {
            Inline::Text(text) => text.as_str(),
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn line_x(direction: Direction, start_x: Pt, content_width: Pt, text_width: Pt) -> Pt {
    match direction {
        Direction::LTR => start_x,

        Direction::RTL => Pt::new(start_x.value() + content_width.value() - text_width.value()),
    }
}

pub fn wrap_text(
    text: &str,
    font: &Font,
    direction: Direction,
    font_size: Pt,
    max_width: Pt,
) -> Result<Vec<ShapedText>, AppError> {
    let words = text.split_whitespace().collect::<Vec<_>>();

    if words.is_empty() {
        return Ok(Vec::new());
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in words {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{current} {word}")
        };

        let shaped = Shaper::shaped_text(font, &candidate, direction, font_size)?;

        if shaped.width.value() <= max_width.value() || current.is_empty() {
            current = candidate;
        } else {
            let line = Shaper::shaped_text(font, &current, direction, font_size)?;

            lines.push(line);

            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(Shaper::shaped_text(font, &current, direction, font_size)?);
    }

    Ok(lines)
}
