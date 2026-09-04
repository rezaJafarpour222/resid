use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct Font {
    pub family: &'static str,
    pub data: &'static [u8],
}

pub const FONTS: &[Font] = &[
    Font::load(
        "B-Nazanin",
        include_bytes!("../../resources/fonts/B-NAZANIN.TTF"),
    ),
    Font::load(
        "Vazirmatn",
        include_bytes!("../../resources/fonts/Vazirmatn.ttf"),
    ),
];
impl Font {
    const fn load(family: &'static str, data: &'static [u8]) -> Self {
        Font { family, data }
    }

    pub fn get_font(name: &str) -> Result<Self, AppError> {
        FONTS
            .iter()
            .find(|f| f.family == name)
            .cloned()
            .ok_or_else(|| AppError::FontError("Font is not available.".to_string()))
    }

    pub fn units_per_em(&self) -> Result<i32, AppError> {
        let face = rustybuzz::Face::from_slice(self.data, 0)
            .ok_or_else(|| AppError::FontError("Invalid font".to_string()))?;
        Ok(face.units_per_em())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn load_font() -> Font {
        super::Font::get_font("B-Nazanin").unwrap()
    }
    #[test]
    fn font_file_can_be_loaded() {
        let font = load_font();
        assert_eq!(font.family, "B-Nazanin");
        assert!(!font.data.is_empty());
    }
    #[test]
    fn font_contains_persian() {
        let font = load_font();
        let face = rustybuzz::Face::from_slice(font.data, 0).expect("invalid font");

        assert!(face.glyph_index('س').is_some());
        assert!(face.glyph_index('ل').is_some());
        assert!(face.glyph_index('ا').is_some());
        assert!(face.glyph_index('م').is_some());
    }

    #[test]
    fn reads_units_per_em() {
        let font = load_font();
        let units = font.units_per_em().expect("failed to read unit_per_em()");
        println!("units per em ={units}");
        assert!(units > 0)
    }
}
