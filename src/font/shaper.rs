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

        let face = rustybuzz::Face::from_slice(font.data, 0)
            .ok_or_else(|| AppError::FontError("Invalid font".to_string()))?;

        /*
         * rustybuzz handles glyph shaping, but not Unicode BiDi reordering.
         *
         * For Persian text containing numbers or Latin text, we first resolve
         * the Unicode Bidirectional Algorithm, get the visual runs, and then
         * shape every run in its own direction.
         */
        let paragraph_level = match direction {
            Direction::RTL => unicode_bidi::Level::rtl(),
            Direction::LTR => unicode_bidi::Level::ltr(),
        };

        let bidi = unicode_bidi::BidiInfo::new(text, Some(paragraph_level));

        let para = &bidi.paragraphs[0];
        let line = para.range.clone();

        let (_, runs) = bidi.visual_runs(para, line);

        let mut glyphs = Vec::new();

        for run in runs {
            if run.start == run.end {
                continue;
            }

            let run_text = &text[run.clone()];

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
                glyphs.push(ShapedGlyph {
                    id: info.glyph_id,
                    x_advance: position.x_advance,
                    y_advance: position.y_advance,
                    x_offset: position.x_offset,
                    y_offset: position.y_offset,

                    // rustybuzz's cluster is relative to this run.
                    // Convert it back to the original logical text offset.
                    cluster: run.start as u32 + info.cluster,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::loader::Font;

    fn test_font() -> Font {
        super::Font::get_font("B-Nazanin").unwrap()
    }

    #[test]
    fn shape_return_glyphs() {
        let font = test_font();
        let text = "سلام دنیا";

        let glyphs =
            Shaper::shape_glyphs(&font, text, Direction::RTL).expect("failed to shape text");

        assert!(!glyphs.is_empty());
    }

    #[test]
    fn shaped_text() {
        let font = test_font();
        let text = "سلام دنیا";
        let font_size = Pt(14.0);

        let shaped_text = Shaper::shaped_text(&font, text, Direction::RTL, font_size)
            .expect("failed to shape the text");

        assert_eq!(text, shaped_text.text);
        assert!(!shaped_text.glyphs.is_empty());
        assert!(shaped_text.width.value() > 0.0);
    }

    #[test]
    fn bidi_keeps_number_sequence_together() {
        let text = "شماره فاکتور: ۱۴۰۵-۰۰۱۲۵";

        let bidi = unicode_bidi::BidiInfo::new(text, Some(unicode_bidi::Level::ltr()));

        let para = &bidi.paragraphs[0];
        let display = bidi.reorder_line(para, para.range.clone());

        assert!(
            display.contains("۱۴۰۵-۰۰۱۲۵"),
            "unexpected visual order: {display}"
        );
    }

    #[test]
    fn bidi_creates_separate_directional_runs() {
        let text = "مبلغ 1250000 ریال";

        let bidi = unicode_bidi::BidiInfo::new(text, Some(unicode_bidi::Level::rtl()));

        let para = &bidi.paragraphs[0];
        let (_, runs) = bidi.visual_runs(para, para.range.clone());

        assert!(
            runs.len() >= 2,
            "expected multiple bidi runs, got {}",
            runs.len()
        );
    }
}
