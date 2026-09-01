use lopdf::{
    Object,
    content::{Content, Operation},
    dictionary,
};

use crate::{
    document::types::LayoutLine, error::AppError, font::types::ShapedText, pdf::writer::PdfWriter,
    units::Pt,
};

impl PdfWriter {
    fn pdf_y(&self, y: Pt, height: Pt) -> Pt {
        Pt::new(self.page_height.value() - y.value() - height.value())
    }

    pub fn draw_rectangle(&mut self, x: Pt, y: Pt, width: Pt, height: Pt) {
        let pdf_y = self.pdf_y(y, height);

        self.operations.push(Operation::new(
            "re",
            vec![
                x.value().into(),
                pdf_y.value().into(),
                width.value().into(),
                height.value().into(),
            ],
        ));

        self.operations.push(Operation::new("S", vec![]));
    }

    pub fn draw_shaped_text(
        &mut self,
        shaped: &ShapedText,
        x: Pt,
        y: Pt,
        font_size: Pt,
    ) -> Result<(), AppError> {
        if !self.font_installed {
            return Err(AppError::PdfWriter(
                "font must be installed before drawing text".to_string(),
            ));
        }

        let glyph_ids = shaped
            .glyphs
            .iter()
            .map(|glyph| glyph.id)
            .collect::<Vec<_>>();

        let bytes = PdfWriter::encode_cids(&glyph_ids)?;

        let pdf_y = self.pdf_y(y, font_size);

        self.operations.push(Operation::new("BT", vec![]));

        self.operations.push(Operation::new(
            "Tf",
            vec![Object::Name(b"F1".to_vec()), font_size.value().into()],
        ));

        self.operations.push(Operation::new(
            "Td",
            vec![x.value().into(), pdf_y.value().into()],
        ));

        self.operations.push(Operation::new(
            "Tj",
            vec![Object::String(bytes, lopdf::StringFormat::Hexadecimal)],
        ));

        self.operations.push(Operation::new("ET", vec![]));

        Ok(())
    }

    pub fn draw_layout_line(&mut self, line: &LayoutLine) -> Result<(), AppError> {
        self.draw_shaped_text(
            &line.glyphs,
            line.position.x,
            line.position.y,
            line.font_size,
        )
    }

    pub fn encode_cids(glyph_ids: &[u32]) -> Result<Vec<u8>, AppError> {
        let mut bytes = Vec::with_capacity(glyph_ids.len() * 2);

        for &gid in glyph_ids {
            if gid > u16::MAX as u32 {
                return Err(AppError::PdfWriter(
                    "glyph ID exceeds PDF CID range".to_string(),
                ));
            }

            let cid = gid as u16;

            bytes.push((cid >> 8) as u8);

            bytes.push((cid & 0xff) as u8);
        }

        Ok(bytes)
    }

    pub fn finish(mut self) -> Result<Vec<u8>, AppError> {
        let content = Content {
            operations: self.operations,
        };

        let content_data = content
            .encode()
            .map_err(|error| std::io::Error::other(error.to_string()))?;

        let content_id = self.document.new_object_id();

        self.document.objects.insert(
            content_id,
            Object::Stream(lopdf::Stream::new(dictionary! {}, content_data)),
        );

        if let Some(Object::Dictionary(page)) = self.document.objects.get_mut(&self.page_id) {
            page.set("Contents", content_id);
        }

        let mut buffer = Vec::new();

        self.document.save_to(&mut buffer)?;

        Ok(buffer)
    }
}
