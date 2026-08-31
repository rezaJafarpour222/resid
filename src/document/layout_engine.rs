use crate::{
    document::{
        helper::{inline_text, line_x, shape_inline_content, wrap_text},
        types::{
            Block, Document, InlineContent, LayoutBlock, LayoutContent, LayoutDocument, LayoutLine,
            LayoutPage, Style,
        },
    },
    error::AppError,
    font::loader::Font,
    units::{Position, Pt, Rectangle, Size},
};

pub struct LayoutEngine<'a> {
    font: &'a Font,
}

impl<'a> LayoutEngine<'a> {
    pub fn new(font: &'a Font) -> Self {
        Self { font }
    }

    pub fn create_layout(&self, document: &Document) -> Result<LayoutDocument, AppError> {
        let content_width = Pt(document.page.width.value()
            - document.page.margin_left.value()
            - document.page.margin_right.value());

        let start_x = document.page.margin_left;
        let mut cursor_y = document.page.margin_top;

        let mut blocks = Vec::new();

        for block in &document.blocks {
            match block {
                Block::Heading { content, style, .. } => {
                    let layout_block = self.create_layout_heading(
                        content,
                        content_width,
                        start_x,
                        &cursor_y,
                        style,
                    )?;

                    blocks.push(layout_block);

                    let line_height = Pt(style.font_size * style.line_height);

                    cursor_y = Pt(cursor_y.value() + line_height.value());
                }

                Block::Paragraph { content, style } => {
                    let layout_block = self.create_layout_paragraph(
                        style,
                        content,
                        content_width,
                        &mut cursor_y,
                        &start_x,
                    )?;

                    blocks.push(layout_block);
                }
            }
        }

        Ok(LayoutDocument {
            pages: vec![LayoutPage {
                size: Size {
                    width: document.page.width,
                    height: document.page.height,
                },
                blocks,
            }],
        })
    }

    fn create_layout_heading(
        &self,
        content: &InlineContent,
        content_width: Pt,
        start_x: Pt,
        cursor_y: &Pt,
        style: &Style,
    ) -> Result<LayoutBlock, AppError> {
        let font_size = Pt(style.font_size);

        let line_height = Pt(style.font_size * style.line_height);

        let layout_text = shape_inline_content(content, self.font, style.direction, font_size)?;

        let shaped = layout_text.shaped;
        let width = shaped.width;

        let line = LayoutLine {
            text: layout_text.text,
            glyphs: shaped,
            width,
            position: Position {
                x: line_x(style.direction, start_x, content_width, width),
                y: *cursor_y,
            },
            font_size,
            direction: style.direction,
        };

        let rect = Rectangle {
            position: Position {
                x: start_x,
                y: *cursor_y,
            },
            size: Size {
                width: content_width,
                height: line_height,
            },
        };

        Ok(LayoutBlock {
            rect,
            content: LayoutContent { lines: vec![line] },
        })
    }

    fn create_layout_paragraph(
        &self,
        style: &Style,
        content: &InlineContent,
        content_width: Pt,
        cursor_y: &mut Pt,
        start_x: &Pt,
    ) -> Result<LayoutBlock, AppError> {
        let text = inline_text(content);

        let font_size = Pt(style.font_size);

        let line_height = Pt(style.font_size * style.line_height);

        let shaped_lines = wrap_text(&text, self.font, style.direction, font_size, content_width)?;

        let block_start_y = *cursor_y;

        let mut lines = Vec::new();

        for shaped in shaped_lines {
            let width = shaped.width;

            let line = LayoutLine {
                text: shaped.text.clone(),
                glyphs: shaped,
                width,
                position: Position {
                    x: line_x(style.direction, *start_x, content_width, width),
                    y: *cursor_y,
                },
                font_size,
                direction: style.direction,
            };

            lines.push(line);

            *cursor_y = Pt(cursor_y.value() + line_height.value());
        }

        let block_height = Pt(cursor_y.value() - block_start_y.value());

        let rect = Rectangle {
            position: Position {
                x: *start_x,
                y: block_start_y,
            },
            size: Size {
                width: content_width,
                height: block_height,
            },
        };

        Ok(LayoutBlock {
            rect,
            content: LayoutContent { lines },
        })
    }
}
