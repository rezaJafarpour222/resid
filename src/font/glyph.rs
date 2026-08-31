use crate::{
    error::AppError,
    font::{loader::Font, types::Direction},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ShapedGlyph {
    pub id: u32,
    pub x_advance: i32,
    pub y_advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub cluster: u32,
}
impl ShapedGlyph {
    pub fn get_shaped_glyphs(
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
    // pub fn text_shaper(font:&Font,text:&str,direction: Direction,font_size:)
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
        let glyphs = ShapedGlyph::get_shaped_glyphs(&font, "سلام دنیا", Direction::RTL)
            .expect("failed to shape text");

        for (index, glyph) in glyphs.iter().enumerate() {
            println!(
                "glyph[{index}]: id={}, x_advance={}, y_advance={}, x_offset={}, y_offset={}",
                glyph.id, glyph.x_advance, glyph.y_advance, glyph.x_offset, glyph.y_offset,
            );
        }
        assert!(!glyphs.is_empty())
    }
}
