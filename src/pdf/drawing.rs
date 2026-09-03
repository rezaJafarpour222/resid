use lopdf::{Object, content::Operation};

use crate::{
    css::types::{Color, FontWeight},
    document::types::LayoutLine,
    error::AppError,
    font::types::ShapedText,
    pdf::writer::PdfWriter,
    units::Pt,
};

impl PdfWriter {
    fn pdf_y(&self, y: Pt, height: Pt) -> Pt {
        Pt::new(self.page_height.value() - y.value() - height.value())
    }

    fn set_fill_color(&mut self, color: Color) {
        self.operations.push(Operation::new(
            "rg",
            vec![
                (color.r as f32 / 255.0).into(),
                (color.g as f32 / 255.0).into(),
                (color.b as f32 / 255.0).into(),
            ],
        ));
    }

    fn set_stroke_color(&mut self, color: Color) {
        self.operations.push(Operation::new(
            "RG",
            vec![
                (color.r as f32 / 255.0).into(),
                (color.g as f32 / 255.0).into(),
                (color.b as f32 / 255.0).into(),
            ],
        ));
    }

    pub fn draw_background(&mut self, x: Pt, y: Pt, width: Pt, height: Pt, color: Color) {
        let pdf_y = self.pdf_y(y, height);
        self.set_fill_color(color);
        self.operations.push(Operation::new(
            "re",
            vec![
                x.value().into(),
                pdf_y.value().into(),
                width.value().into(),
                height.value().into(),
            ],
        ));
        self.operations.push(Operation::new("f", vec![]));
    }

    pub fn draw_border(
        &mut self,
        x: Pt,
        y: Pt,
        width: Pt,
        height: Pt,
        border_width: Pt,
        color: Color,
    ) {
        if border_width.value() <= 0.0 {
            return;
        }

        let pdf_y = self.pdf_y(y, height);
        self.set_stroke_color(color);
        self.operations
            .push(Operation::new("w", vec![border_width.value().into()]));
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

    pub fn draw_rectangle(&mut self, x: Pt, y: Pt, width: Pt, height: Pt) {
        self.draw_border(x, y, width, height, Pt::new(1.0), Color::BLACK);
    }

    pub fn draw_shaped_text(
        &mut self,
        shaped: &ShapedText,
        x: Pt,
        y: Pt,
        font_size: Pt,
        color: Color,
        font_weight: FontWeight,
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

        self.set_fill_color(color);
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

        // B-Nazanin is the only configured font. Do not pretend to select a
        // different face for bold until a real bold font is supported.
        let _ = font_weight;

        self.operations.push(Operation::new("ET", vec![]));

        Ok(())
    }

    pub fn draw_layout_line(&mut self, line: &LayoutLine) -> Result<(), AppError> {
        self.draw_shaped_text(
            &line.glyphs,
            line.position.x,
            line.position.y,
            line.font_size,
            line.color,
            line.font_weight,
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

    pub fn draw_layout_block(
        &mut self,
        block: &crate::document::types::LayoutBlock,
    ) -> Result<(), AppError> {
        if let Some(color) = block.background {
            self.draw_background(
                block.rect.position.x,
                block.rect.position.y,
                block.rect.size.width,
                block.rect.size.height,
                color,
            );
        }

        if block.border.width.value() > 0.0 {
            self.draw_border(
                block.rect.position.x,
                block.rect.position.y,
                block.rect.size.width,
                block.rect.size.height,
                block.border.width,
                block.border.color,
            );
        }

        for line in &block.content.lines {
            self.draw_layout_line(line)?;
        }

        Ok(())
    }
}
