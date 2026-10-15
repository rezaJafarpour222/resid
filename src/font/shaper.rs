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
