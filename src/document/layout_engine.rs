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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        document::types::{Block, Document, Inline, InlineContent, Page, Style},
        font::loader::Font,
        units::Direction,
    };

    fn test_font() -> Font {
        Font::load("B Nazanin", "B-NAZANIN.TTF").expect("failed to load test font")
    }

    fn text_content(text: &str) -> InlineContent {
        InlineContent {
            items: vec![Inline::Text(text.to_string())],
        }
    }

    fn rtl_style(font_size: f32, line_height: f32) -> Style {
        Style {
            direction: Direction::RTL,
            font_size,
            line_height,
        }
    }

    #[test]
    fn creates_layout_document() {
        let document = Document {
            page: Page::a4(),
            blocks: vec![],
        };

        let font = test_font();
        let engine = LayoutEngine::new(&font);

        let layout = engine.create_layout(&document).expect("layout failed");

        assert_eq!(layout.pages.len(), 1);

        assert_eq!(layout.pages[0].size.width, document.page.width);

        assert_eq!(layout.pages[0].size.height, document.page.height);

        assert!(layout.pages[0].blocks.is_empty());
    }

    #[test]
    fn lays_out_heading() {
        let style = rtl_style(24.0, 1.5);

        let document = Document {
            page: Page::a4(),
            blocks: vec![Block::Heading {
                level: 1,
                content: text_content("فاکتور فروش"),
                style,
            }],
        };

        let font = test_font();
        let engine = LayoutEngine::new(&font);

        let layout = engine.create_layout(&document).expect("layout failed");

        assert_eq!(layout.pages[0].blocks.len(), 1);

        let block = &layout.pages[0].blocks[0];

        assert_eq!(block.content.lines.len(), 1);

        let line = &block.content.lines[0];

        assert_eq!(line.text, "فاکتور فروش");

        assert_eq!(line.direction, Direction::RTL);

        assert!(line.width.value() > 0.0);

        assert!(line.position.y.value() == document.page.margin_top.value());

        assert_eq!(block.rect.position.x, document.page.margin_left);

        assert_eq!(
            block.rect.size.width,
            Pt(document.page.width.value()
                - document.page.margin_left.value()
                - document.page.margin_right.value())
        );
    }

    #[test]
    fn lays_out_paragraph() {
        let style = rtl_style(14.0, 1.5);

        let document = Document {
            page: Page::a4(),
            blocks: vec![Block::Paragraph {
                content: text_content("سلام دنیا"),
                style,
            }],
        };

        let font = test_font();
        let engine = LayoutEngine::new(&font);

        let layout = engine.create_layout(&document).expect("layout failed");

        let block = &layout.pages[0].blocks[0];

        assert_eq!(block.content.lines.len(), 1);

        let line = &block.content.lines[0];

        assert_eq!(line.text, "سلام دنیا");

        assert_eq!(line.direction, Direction::RTL);

        assert!(line.width.value() > 0.0);

        assert!(!line.glyphs.glyphs.is_empty());
    }

    #[test]
    fn heading_is_positioned_before_paragraph() {
        let heading_style = rtl_style(24.0, 1.5);

        let paragraph_style = rtl_style(14.0, 1.5);

        let document = Document {
            page: Page::a4(),
            blocks: vec![
                Block::Heading {
                    level: 1,
                    content: text_content("فاکتور فروش"),
                    style: heading_style,
                },
                Block::Paragraph {
                    content: text_content("سلام دنیا"),
                    style: paragraph_style,
                },
            ],
        };

        let font = test_font();
        let engine = LayoutEngine::new(&font);

        let layout = engine.create_layout(&document).expect("layout failed");

        assert_eq!(layout.pages[0].blocks.len(), 2);

        let heading = &layout.pages[0].blocks[0];

        let paragraph = &layout.pages[0].blocks[1];

        assert!(
            paragraph.rect.position.y.value()
                >= heading.rect.position.y.value() + heading.rect.size.height.value()
        );

        assert!(
            paragraph.content.lines[0].position.y.value()
                >= heading.content.lines[0].position.y.value()
        );
    }

    #[test]
    fn rtl_line_is_aligned_to_right_content_edge() {
        let style = rtl_style(14.0, 1.5);

        let document = Document {
            page: Page::a4(),
            blocks: vec![Block::Paragraph {
                content: text_content("سلام دنیا"),
                style,
            }],
        };

        let font = test_font();
        let engine = LayoutEngine::new(&font);

        let layout = engine.create_layout(&document).expect("layout failed");

        let line = &layout.pages[0].blocks[0].content.lines[0];

        let content_right = document.page.width.value() - document.page.margin_right.value();

        let expected_x = content_right - line.width.value();

        assert!((line.position.x.value() - expected_x).abs() < 0.001);
    }

    #[test]
    fn long_paragraph_wraps_into_multiple_lines() {
        let style = rtl_style(12.0, 1.5);

        let document = Document {
            page: Page {
                width: Pt::new(200.0),
                height: Pt::new(300.0),
                margin_top: Pt::new(20.0),
                margin_right: Pt::new(20.0),
                margin_bottom: Pt::new(20.0),
                margin_left: Pt::new(20.0),
            },
            blocks: vec![Block::Paragraph {
                content: text_content(
                    "این یک متن فارسی بسیار بسیار بسیار بسیار بسیار بسیار طولانی است",
                ),
                style,
            }],
        };

        let font = test_font();
        let engine = LayoutEngine::new(&font);

        let layout = engine.create_layout(&document).expect("layout failed");

        let paragraph = &layout.pages[0].blocks[0];

        println!("content width = {}", paragraph.rect.size.width.value());

        println!("number of lines = {}", paragraph.content.lines.len());

        for line in &paragraph.content.lines {
            println!("line: {:?}, width = {}", line.text, line.width.value());
        }

        assert!(
            paragraph.content.lines.len() > 1,
            "expected paragraph to wrap"
        );

        for line in &paragraph.content.lines {
            assert!(
                line.width.value() <= paragraph.rect.size.width.value()
                    || line.text.split_whitespace().count() == 1
            );
        }
    }

    #[test]
    fn wrapped_lines_move_down() {
        let style = rtl_style(12.0, 1.5);

        let document = Document {
            page: Page {
                width: Pt::new(200.0),
                height: Pt::new(300.0),
                margin_top: Pt::new(20.0),
                margin_right: Pt::new(20.0),
                margin_bottom: Pt::new(20.0),
                margin_left: Pt::new(20.0),
            },
            blocks: vec![Block::Paragraph {
                content: text_content(
                    "این یک متن فارسی بسیار بسیار بسیار بسیار بسیار بسیار طولانی است",
                ),
                style,
            }],
        };

        let font = test_font();
        let engine = LayoutEngine::new(&font);

        let layout = engine.create_layout(&document).expect("layout failed");

        let lines = &layout.pages[0].blocks[0].content.lines;

        assert!(lines.len() > 1, "expected multiple lines");

        for pair in lines.windows(2) {
            assert!(
                pair[1].position.y.value() > pair[0].position.y.value(),
                "expected each line to have a greater y position"
            );
        }
    }
}
