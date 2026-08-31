use crate::{
    error::AppError,
    font::{
        loader::Font,
        types::{Direction, ShapedGlyph, ShapedText},
        units::Point,
    },
};

pub struct Shaper;
impl Shaper {
    pub fn get_shaped_glyphs(
        &self,
        font: &Font,
        text: &str,
        direction: Direction,
    ) -> Result<Vec<ShapedGlyph>, AppError> {
        let face = rustybuzz::Face::from_slice(&font.data, 0)
            .ok_or_else(|| AppError::FontError("Invalid font".to_string()))?;

        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(text);

        match direction {
            Direction::RTL => buffer.set_direction(rustybuzz::Direction::RightToLeft),
            Direction::LTR => buffer.set_direction(rustybuzz::Direction::LeftToRight),
        }
        let buffer = rustybuzz::shape(&face, &[], buffer);
        let glyph = buffer
            .glyph_infos()
            .iter()
            .zip(buffer.glyph_positions().iter())
            .map(|(info, position)| ShapedGlyph {
                id: info.glyph_id,
                x_advance: position.x_advance,
                y_advance: position.y_advance,
                x_offset: position.x_offset,
                y_offset: position.y_offset,
                cluster: info.cluster,
            })
            .collect();

        Ok(glyph)
    }
    pub fn text_shaper(
        &self,
        font: &Font,
        text: &str,
        direction: Direction,
        font_size: Point,
    ) -> Result<ShapedText, AppError> {
        let glyphs = self.get_shaped_glyphs(font, text, direction)?;

        let units_per_em = font.unit_per_em()? as f32;
        let advance: i32 = glyphs.iter().map(|glyph| glyph.x_advance).sum();
        let width = Point((advance as f32 / units_per_em) * font_size.value());

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
    use std::path::PathBuf;

    use crate::font::loader::Font;

    fn test_font() -> Font {
        Font::load("B NAZANIN", PathBuf::from("B-NAZANIN.TTF")).expect("failed to load test font")
    }
    #[test]
    fn shape_return_glyphs() {
        let font = test_font();
        let text = "سلام دنیا";
        let dir = Direction::RTL;

        let glyphs = Shaper
            .get_shaped_glyphs(&font, text, dir)
            .expect("failed to shape text");

        for (index, glyph) in glyphs.iter().enumerate() {
            println!(
                "glyph[{index}]: id={}, x_advance={}, y_advance={}, x_offset={}, y_offset={}",
                glyph.id, glyph.x_advance, glyph.y_advance, glyph.x_offset, glyph.y_offset,
            );
        }
        assert!(!glyphs.is_empty())
    }
    #[test]
    fn shaped_text() {
        let font = test_font();
        let text = "سلام دنیا";
        let dir = Direction::RTL;
        let font_size = Point(14.0);
        let shaped_text = Shaper
            .text_shaper(&font, text, dir, font_size)
            .expect("failed to shape the text");
        assert_eq!(text, shaped_text.text);
        assert!(!shaped_text.glyphs.is_empty());
        assert!(shaped_text.width.value() > 0.0)
    }
}
