use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use crate::{
    document::types::{Block, Document, Page, Style},
    error::AppError,
    html::helper::collect_inline,
    units::Direction,
};

pub struct HtmlBuilder;

impl HtmlBuilder {
    pub fn parse(html: &str) -> Result<Document, AppError> {
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .map_err(|_| AppError::HtmlParsing("html parsing failed".to_string()))?;
        let mut blocks = Vec::new();
        HtmlBuilder::collect_blocks(&dom.document, &mut blocks, Direction::LTR);
        Ok(Document {
            page: Page::a4(),
            blocks,
        })
    }

    fn collect_blocks(handle: &Handle, blocks: &mut Vec<Block>, inherited_direction: Direction) {
        for child in handle.children.borrow().iter() {
            match &child.data {
                NodeData::Element { name, .. } => {
                    let tag = name.local.as_ref();
                    let direction =
                        HtmlBuilder::read_direction(child).unwrap_or(inherited_direction);

                    match tag {
                        "p" => {
                            HtmlBuilder::paragraph(direction, child, blocks);
                        }

                        "h1" | "h2" | "h3" => {
                            HtmlBuilder::heading(tag, child, direction, blocks);
                        }

                        _ => {
                            HtmlBuilder::collect_blocks(child, blocks, direction);
                        }
                    }
                }

                _ => {
                    HtmlBuilder::collect_blocks(child, blocks, inherited_direction);
                }
            }
        }
    }
    fn read_direction(handle: &Handle) -> Option<Direction> {
        match &handle.data {
            NodeData::Element { attrs, .. } => {
                for attr in attrs.borrow().iter() {
                    if attr.name.local.as_ref() == "dir" {
                        return match attr.value.as_ref() {
                            "rtl" => Some(Direction::RTL),
                            "ltr" => Some(Direction::LTR),
                            _ => None,
                        };
                    }
                }
                None
            }
            _ => None,
        }
    }
    fn paragraph(direction: Direction, handle: &Handle, blocks: &mut Vec<Block>) {
        let style = Style {
            direction,
            font_size: 12.0,
            line_height: 1.8,
        };

        blocks.push(Block::Paragraph {
            content: collect_inline(handle),
            style,
        });
    }
    fn heading(tag: &str, handle: &Handle, direction: Direction, blocks: &mut Vec<Block>) {
        let level = match tag {
            "h1" => 1,
            "h2" => 2,
            "h3" => 3,
            _ => unreachable!(),
        };

        let font_size = match level {
            1 => 24.0,
            2 => 20.0,
            3 => 16.0,
            _ => 12.0,
        };

        let style = Style {
            direction,
            font_size,
            line_height: 1.3,
        };

        blocks.push(Block::Heading {
            level,
            content: collect_inline(handle),
            style,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{document::types::Inline, units::Millimeter};

    use super::*;

    #[test]
    fn parses_rtl_document() {
        let document = HtmlBuilder::parse(
            r#"
            <html>
                <body dir="rtl">
                    <h1>فاکتور فروش</h1>
                    <p>سلام دنیا</p>
                </body>
            </html>
            "#,
        )
        .unwrap();

        assert_eq!(document.page.width, Millimeter::new(210.0).into());
        assert_eq!(document.page.height, Millimeter::new(297.0).into());

        assert_eq!(document.blocks.len(), 2);

        match &document.blocks[0] {
            Block::Heading {
                level,
                style,
                content,
            } => {
                assert_eq!(*level, 1);
                assert_eq!(style.direction, Direction::RTL);
                assert_eq!(content.items, vec![Inline::Text("فاکتور فروش".to_string())]);
            }

            _ => panic!("expected heading"),
        }

        match &document.blocks[1] {
            Block::Paragraph { style, .. } => {
                assert_eq!(style.direction, Direction::RTL);
                assert_eq!(style.font_size, 12.0);
                assert_eq!(style.line_height, 1.8);
            }

            _ => panic!("expected paragraph"),
        }
    }
}
