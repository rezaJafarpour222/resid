use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct Font {
    pub family: String,
    pub data: Vec<u8>,
}
impl Font {
    pub fn load<P: AsRef<std::path::Path>>(
        family: impl Into<String>,
        path: P,
    ) -> std::io::Result<Self> {
        let data = std::fs::read(path)?;
        Ok(Self {
            family: family.into(),
            data,
        })
    }

    pub fn units_per_em(&self) -> Result<i32, AppError> {
        let face = rustybuzz::Face::from_slice(&self.data, 0)
            .ok_or_else(|| AppError::FontError("Invalid font".to_string()))?;
        Ok(face.units_per_em())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn font_file_can_be_loaded() {
        let path = PathBuf::from("B-NAZANIN.TTF");
        let font = Font::load("B NAZANIN", &path).expect("failed to laod font file");
        assert_eq!(font.family, "B NAZANIN");
        assert!(!font.data.is_empty());
    }
    #[test]
    fn font_contains_persian() {
        let path = PathBuf::from("B-NAZANIN.TTF");
        let font = Font::load("B NAZANIN", &path).expect("failed to laod font file");
        let face = rustybuzz::Face::from_slice(&font.data, 0).expect("invalid font");

        assert!(face.glyph_index('س').is_some());
        assert!(face.glyph_index('ل').is_some());
        assert!(face.glyph_index('ا').is_some());
        assert!(face.glyph_index('م').is_some());
    }

    #[test]
    fn reads_units_per_em() {
        let path = PathBuf::from("B-NAZANIN.TTF");
        let font = Font::load("B NAZANIN", &path).expect("failed to laod font file");
        let units = font.units_per_em().expect("failed to read unit_per_em()");
        println!("units per em ={units}");
        assert!(units > 0)
    }
}
