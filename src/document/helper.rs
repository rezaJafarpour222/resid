use crate::{
    error::AppError,
    font::{loader::Font, shaper::Shaper, types::ShapedText},
    units::{Direction, Pt},
};

pub fn line_x(
    direction: Direction,
    text_align: crate::css::types::TextAlign,
    start_x: Pt,
    content_width: Pt,
    text_width: Pt,
) -> Pt {
    use crate::css::types::TextAlign;

    match text_align {
        TextAlign::Center => {
            Pt::new(start_x.value() + (content_width.value() - text_width.value()) / 2.0)
        }
        TextAlign::Left => start_x,
        TextAlign::Right => Pt::new(start_x.value() + content_width.value() - text_width.value()),
        TextAlign::Start => match direction {
            Direction::LTR => start_x,
            Direction::RTL => Pt::new(start_x.value() + content_width.value() - text_width.value()),
        },
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
            lines.push(Shaper::shaped_text(font, &current, direction, font_size)?);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(Shaper::shaped_text(font, &current, direction, font_size)?);
    }

    Ok(lines)
}
