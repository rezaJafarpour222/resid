use lopdf::{Object, ObjectId, dictionary};

use crate::{
    error::AppError,
    font::{loader::Font, types::ShapedText},
    pdf::writer::PdfWriter,
};

impl PdfWriter {
    pub fn set_font(&mut self, font: Font) {
        self.font = Some(font)
    }
    pub fn embed_font(&mut self) -> Result<ObjectId, AppError> {
        let font = self
            .font
            .as_ref()
            .ok_or_else(|| AppError::PdfWriter("no font configured".to_string()))?;

        let font_data = font.data.to_vec();
        let font_file_id = self.document.new_object_id();

        self.document.objects.insert(
            font_file_id,
            Object::Stream(lopdf::Stream::new(
                dictionary! {
                    "Length1" => font_data.len() as i64,
                },
                font_data,
            )),
        );

        Ok(font_file_id)
    }
    fn font_units_per_em(&self) -> Result<u16, AppError> {
        let font = self
            .font
            .as_ref()
            .ok_or_else(|| AppError::PdfWriter("no font configured".to_string()))?;

        Ok(font.units_per_em()? as u16)
    }

    pub fn create_font_descriptor(
        &mut self,
        font_file_id: lopdf::ObjectId,
    ) -> Result<lopdf::ObjectId, AppError> {
        let font = self
            .font
            .as_ref()
            .ok_or_else(|| AppError::PdfWriter("no font configured".to_string()))?;

        let face = ttf_parser::Face::parse(font.data, 0)
            .map_err(|_| AppError::PdfWriter("invalid TrueType font".to_string()))?;
        let units_per_em = font.units_per_em()? as f64;

        let ascent = face.ascender() as f64 / units_per_em * 1000.0;

        let descent = face.descender() as f64 / units_per_em * 1000.0;

        let bbox = face.global_bounding_box();

        let x_min = bbox.x_min as f64 / units_per_em * 1000.0;

        let y_min = bbox.y_min as f64 / units_per_em * 1000.0;

        let x_max = bbox.x_max as f64 / units_per_em * 1000.0;

        let y_max = bbox.y_max as f64 / units_per_em * 1000.0;

        let descriptor_id = self.document.new_object_id();

        self.document.objects.insert(
            descriptor_id,
            dictionary! {
                "Type" => "FontDescriptor",
                "FontName" => self.font.as_ref().map_or("Unknown", |f| f.family),
                "Flags" => 4,
                "Ascent" => ascent,
                "Descent" => descent,
                "CapHeight" => ascent,
                "ItalicAngle" => 0,
                "StemV" => 80,
                "FontBBox" => vec![
                    x_min.into(),
                    y_min.into(),
                    x_max.into(),
                    y_max.into(),
                ],
                "FontFile2" => font_file_id,
            }
            .into(),
        );

        Ok(descriptor_id)
    }

    pub fn create_cid_to_gid_map(&self) -> Object {
        Object::Name(b"Identity".to_vec())
    }

    pub fn create_cid_font(
        &mut self,
        descriptor_id: lopdf::ObjectId,
        cid_to_gid_map: Object,
        shaped_texts: &[ShapedText],
    ) -> Result<lopdf::ObjectId, AppError> {
        let cid_font_id = self.document.new_object_id();

        let units_per_em = self.font_units_per_em()? as f64;

        let mut entries: Vec<(u32, i64)> = Vec::new();

        for shaped in shaped_texts {
            for glyph in &shaped.glyphs {
                let cid = glyph.id;

                if cid > u16::MAX as u32 {
                    return Err(AppError::PdfWriter(format!(
                        "glyph ID {} exceeds PDF CID range",
                        cid
                    )));
                }

                if entries.iter().any(|(existing, _)| *existing == cid) {
                    continue;
                }

                let width = (glyph.x_advance as f64 / units_per_em * 1000.0).round() as i64;

                entries.push((cid, width));
            }
        }

        entries.sort_by_key(|(cid, _)| *cid);

        let mut widths = Vec::new();

        for (cid, width) in entries {
            widths.push(Object::Integer(cid as i64));
            widths.push(Object::Array(vec![Object::Integer(width)]));
        }

        self.document.objects.insert(
            cid_font_id,
            dictionary! {
                "Type" => "Font",
                "Subtype" => "CIDFontType2",
                "BaseFont" => self.font.as_ref().map_or("Unknown", |f| f.family),

                "CIDSystemInfo" => dictionary! {
                    "Registry" => "Adobe",
                    "Ordering" => "Identity",
                    "Supplement" => 0,
                },

                "FontDescriptor" => descriptor_id,

                "CIDToGIDMap" => cid_to_gid_map,

                "DW" => 1000,

                "W" => Object::Array(widths),
            }
            .into(),
        );

        Ok(cid_font_id)
    }

    pub fn create_to_unicode(
        &mut self,
        shaped_texts: &[ShapedText],
    ) -> Result<lopdf::ObjectId, AppError> {
        let data = PdfWriter::build_to_unicode_cmap(shaped_texts)?;

        let cmap_id = self.document.new_object_id();

        self.document.objects.insert(
            cmap_id,
            Object::Stream(lopdf::Stream::new(dictionary! {}, data)),
        );

        Ok(cmap_id)
    }

    pub fn create_type0_font(
        &mut self,
        cid_font_id: lopdf::ObjectId,
        to_unicode_id: lopdf::ObjectId,
    ) -> lopdf::ObjectId {
        let type0_id = self.document.new_object_id();

        self.document.objects.insert(
            type0_id,
            dictionary! {
                "Type" => "Font",
                "Subtype" => "Type0",
                "BaseFont" => self.font.as_ref().map_or("Unknown", |f| f.family),
                "Encoding" => "Identity-H",
                "DescendantFonts" => vec![
                    cid_font_id.into(),
                ],
                "ToUnicode" => to_unicode_id,
            }
            .into(),
        );

        type0_id
    }

    fn build_to_unicode_cmap(shaped_texts: &[ShapedText]) -> Result<Vec<u8>, AppError> {
        let mut entries: Vec<(u32, String)> = Vec::new();

        for shaped in shaped_texts {
            let text = &shaped.text;

            for glyph in &shaped.glyphs {
                let start = glyph.cluster as usize;

                if start >= text.len() {
                    continue;
                }

                let end = shaped
                    .glyphs
                    .iter()
                    .filter_map(|other| {
                        let cluster = other.cluster as usize;

                        if cluster > start { Some(cluster) } else { None }
                    })
                    .min()
                    .unwrap_or(text.len());

                let unicode = text[start..end].to_string();

                if unicode.is_empty() {
                    continue;
                }

                entries.push((glyph.id, unicode));
            }
        }

        entries.sort_by_key(|entry| entry.0);

        entries.dedup_by_key(|entry| entry.0);

        let mut cmap = String::new();

        cmap.push_str("/CIDInit /ProcSet findresource begin\n");
        cmap.push_str("12 dict begin\n");
        cmap.push_str("begincmap\n");

        cmap.push_str(
            "/CIDSystemInfo <<\n\
         /Registry (Adobe)\n\
         /Ordering (UCS)\n\
         /Supplement 0\n\
         >> def\n",
        );

        cmap.push_str("/CMapName /NevisUnicode def\n");

        cmap.push_str("/CMapType 2 def\n");

        cmap.push_str("1 begincodespacerange\n");

        cmap.push_str("<0000> <FFFF>\n");

        cmap.push_str("endcodespacerange\n");

        cmap.push_str(&format!("{} beginbfchar\n", entries.len()));

        for (glyph_id, unicode) in entries {
            if glyph_id > u16::MAX as u32 {
                return Err(AppError::PdfWriter(
                    "glyph ID exceeds PDF CID range".to_string(),
                ));
            }

            let cid = glyph_id as u16;

            let mut encoded = String::new();

            for unit in unicode.encode_utf16() {
                encoded.push_str(&format!("{:04X}", unit));
            }

            cmap.push_str(&format!("<{:04X}> <{}>\n", cid, encoded));
        }

        cmap.push_str("endbfchar\n");
        cmap.push_str("endcmap\n");
        cmap.push_str("CMapName currentdict /CMap defineresource pop\n");
        cmap.push_str("end\n");
        cmap.push_str("end\n");

        Ok(cmap.into_bytes())
    }
}
