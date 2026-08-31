use crate::{
    document::types::{
        Block, Document, Inline, InlineContent, LayoutBlock, LayoutContent, LayoutDocument,
        LayoutLine, LayoutPage, LayoutText,
    },
    error::AppError,
    font::{loader::Font, shaper::Shaper},
    units::{self, Direction, Position, Pt, Size},
};

pub struct LayoutEngine<'a> {
    font: &'a Font,
}
impl<'a> LayoutEngine<'a> {
    pub fn new(font: &'a Font) -> Self {
        Self { font }
    }
    pub fn create_layout(&self, document: Document) -> Result<LayoutDocument, AppError> {
        let content_width = Pt(document.page.width.value()
            - document.page.margin_left.value()
            - document.page.margin_right.value());
        let start_x = document.page.margin_left;
        let mut cursor_y = document.page.margin_top;
        let mut blocks = Vec::new();
        for block in document.blocks {
            match block {
                Block::Heading { content, style, .. } => {
                    let line_height = Pt(style.line_height + style.font_size);
                    let block = self.create_layout_header(
                        &content,
                        content_width,
                        start_x,
                        &cursor_y,
                        Pt(style.font_size),
                        style.direction,
                        line_height,
                    )?;
                    blocks.push(block);

                    cursor_y = Pt(cursor_y.value() + line_height.value());
                }
                Block::Paragraph { content, style } => {}
            }
        }

        let layout = LayoutDocument {
            pages: vec![LayoutPage {
                size: Size {
                    width: document.page.width,
                    height: document.page.height,
                },
                blocks,
            }],
        };

        Ok(layout)
    }

    fn create_layout_header(
        &self,
        content: &InlineContent,
        content_width: Pt,
        start_x: Pt,
        cursor_y: &Pt,
        font_size: Pt,
        direction: Direction,
        line_height: Pt,
    ) -> Result<LayoutBlock, AppError> {
        let h_text = shape_inline_content(content, self.font, direction, font_size)?;
        let shaped = h_text.shaped;
        let width = shaped.width;

        let line = LayoutLine {
            text: h_text.text,
            glyphs: shaped,
            width,
            position: Position {
                x: line_x(direction, start_x, content_width, width),
                y: cursor_y,
            },
            font_size,
            direction,
        };
        let rect = units::Rectangle {
            position: Position {
                x: start_x,
                y: cursor_y,
            },
            size: Size {
                width: content_width,
                height: line_height,
            },
        };
        let block = LayoutBlock {
            rect,
            content: LayoutContent { lines: vec![line] },
        };

        Ok(block)
    }
}

fn shape_inline_content(
    content: &InlineContent,
    font: &Font,
    direction: Direction,
    font_size: Pt,
) -> Result<LayoutText, AppError> {
    let text = inline_text(content);

    let shaped = Shaper::shaped_text(font, &text, direction, font_size)?;

    Ok(LayoutText { text, shaped })
}
fn inline_text(content: &InlineContent) -> String {
    content
        .items
        .iter()
        .map(|item| match item {
            Inline::Text(text) => text.as_str(),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn line_x(direction: Direction, start_x: Pt, content_width: Pt, text_width: Pt) -> Pt {
    match direction {
        Direction::Ltr => start_x,

        Direction::Rtl => Pt::new(start_x.value() + content_width.value() - text_width.value()),
    }
}
