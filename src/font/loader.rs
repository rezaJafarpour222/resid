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
