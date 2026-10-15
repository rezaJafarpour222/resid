use crate::{
    error::AppError,
    font::{
        loader::Font,
        types::{ShapedGlyph, ShapedText},
    },
    units::{Direction, Pt},
};

pub struct Shaper;

impl Shaper {
    pub fn shape_glyphs(
        font: &Font,
        text: &str,
        direction: Direction,
    ) -> Result<Vec<ShapedGlyph>, AppError> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        let prepared_text = isolate_ltr_runs(text);

        let face = rustybuzz::Face::from_slice(font.data, 0)
            .ok_or_else(|| AppError::FontError("Invalid font".to_string()))?;

        let paragraph_level = match direction {
            Direction::RTL => unicode_bidi::Level::rtl(),
            Direction::LTR => unicode_bidi::Level::ltr(),
        };

        let bidi = unicode_bidi::BidiInfo::new(&prepared_text, Some(paragraph_level));

        let para = &bidi.paragraphs[0];
        let line = para.range.clone();

        let (_, runs) = bidi.visual_runs(para, line);

        let mut glyphs = Vec::new();

        for run in runs {
            if run.start == run.end {
                continue;
            }

            let run_text = &prepared_text[run.clone()];

            let run_direction = if bidi.levels[run.start].is_rtl() {
                rustybuzz::Direction::RightToLeft
            } else {
                rustybuzz::Direction::LeftToRight
            };

            let mut buffer = rustybuzz::UnicodeBuffer::new();
            buffer.push_str(run_text);
            buffer.set_direction(run_direction);

            let shaped = rustybuzz::shape(&face, &[], buffer);

            for (info, position) in shaped
                .glyph_infos()
                .iter()
                .zip(shaped.glyph_positions().iter())
            {
                let cluster_char = run_text
                    .get(info.cluster as usize..)
                    .and_then(|slice| slice.chars().next())
                    .unwrap_or('\0');

                if cluster_char == '\u{2066}' || cluster_char == '\u{2069}' {
                    continue;
                }

                glyphs.push(ShapedGlyph {
                    id: info.glyph_id,
                    x_advance: position.x_advance,
                    y_advance: position.y_advance,
                    x_offset: position.x_offset,
                    y_offset: position.y_offset,

                    cluster: original_cluster(&prepared_text, run.start + info.cluster as usize) as u32,
                });
            }
        }

        Ok(glyphs)
    }

    pub fn shaped_text(
        font: &Font,
        text: &str,
        direction: Direction,
        font_size: Pt,
    ) -> Result<ShapedText, AppError> {
        let glyphs = Self::shape_glyphs(font, text, direction)?;

        let units_per_em = font.units_per_em()? as f32;

        let advance: i32 = glyphs.iter().map(|glyph| glyph.x_advance).sum();

        let width = Pt((advance as f32 / units_per_em) * font_size.value());

        Ok(ShapedText {
            text: text.to_string(),
            glyphs,
            width,
        })
    }
}

fn is_ltr_safe(ch: char) -> bool {
    ch.is_ascii_digit()
        || matches!(
            ch,
            '-' | '.' | '/' | ',' | '+' | '(' | ')' | ':' 
        )
}

fn isolate_ltr_runs(text: &str) -> String {
    let mut out = String::new();
    let mut in_run = false;

    for ch in text.chars() {
        if is_ltr_safe(ch) {
            if !in_run {
                out.push('\u{2066}');
                in_run = true;
            }
        } else if in_run {
            out.push('\u{2069}');
            in_run = false;
        }

        out.push(ch);
    }

    if in_run {
        out.push('\u{2069}');
    }

    out
}

fn original_cluster(prepared: &str, cluster: usize) -> usize {
    let mut prepared_offset = 0usize;
    let mut original_offset = 0usize;

    if cluster == 0 {
        return 0;
    }

    for ch in prepared.chars() {
        if prepared_offset >= cluster {
            break;
        }

        let len = ch.len_utf8();
        if prepared_offset + len > cluster {
            break;
        }

        prepared_offset += len;
        if ch != '\u{2066}' && ch != '\u{2069}' {
            original_offset += len;
        }
    }

    original_offset
}
