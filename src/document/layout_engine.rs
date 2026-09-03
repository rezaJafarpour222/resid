use crate::{
    composition::types::{ComposedDocument, ComposedElement, ComposedNode},
    css::types::Display,
    document::{
        helper::{line_x, wrap_text},
        types::{LayoutBlock, LayoutContent, LayoutDocument, LayoutLine, LayoutPage},
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

    pub fn create_layout(&self, document: &ComposedDocument) -> Result<LayoutDocument, AppError> {
        let content_width = document.page.content_width();
        let start_x = document.page.margin_left;
        let start_y = document.page.margin_top;
        let mut blocks = Vec::new();
        let mut cursor_y = start_y;

        for node in &document.children {
            self.layout_flow_node(node, &mut cursor_y, start_x, content_width, &mut blocks)?;
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

    fn layout_flow_node(
        &self,
        node: &ComposedNode,
        cursor_y: &mut Pt,
        x: Pt,
        available_width: Pt,
        blocks: &mut Vec<LayoutBlock>,
    ) -> Result<(), AppError> {
        match node {
            ComposedNode::Text(_) => Ok(()),
            ComposedNode::Element(element) => {
                if element.style.display == Display::None {
                    return Ok(());
                }

                if element.style.display == Display::Inline {
                    return self.layout_inline_element(
                        element,
                        cursor_y,
                        x,
                        available_width,
                        blocks,
                    );
                }

                let margin = element.style.margin;
                let outer_x = Pt::new(x.value() + margin.left.value());
                let width = Pt::new(
                    (available_width.value() - margin.left.value() - margin.right.value()).max(0.0),
                );

                *cursor_y = Pt::new(cursor_y.value() + margin.top.value());

                let (own_block, children) =
                    self.layout_block_element(element, *cursor_y, outer_x, width)?;
                let own_height = own_block.rect.size.height.value();

                blocks.push(own_block);
                blocks.extend(children);

                *cursor_y = Pt::new(cursor_y.value() + own_height + margin.bottom.value());

                Ok(())
            }
        }
    }

    fn layout_inline_element(
        &self,
        element: &ComposedElement,
        cursor_y: &mut Pt,
        x: Pt,
        width: Pt,
        blocks: &mut Vec<LayoutBlock>,
    ) -> Result<(), AppError> {
        let text = element.text_content();
        if text.trim().is_empty() {
            return Ok(());
        }

        let leaf = self.layout_text_content(element, *cursor_y, x, width)?;
        let height = leaf.rect.size.height.value();
        blocks.push(leaf);
        *cursor_y = Pt::new(cursor_y.value() + height);
        Ok(())
    }

    fn layout_block_element(
        &self,
        element: &ComposedElement,
        y: Pt,
        x: Pt,
        width: Pt,
    ) -> Result<(LayoutBlock, Vec<LayoutBlock>), AppError> {
        let style = &element.style;
        let border = style.border.width.value();
        let inner_width = Pt::new(
            (width.value()
                - border * 2.0
                - style.padding.left.value()
                - style.padding.right.value())
            .max(0.0),
        );
        let content_x = Pt::new(x.value() + border + style.padding.left.value());
        let content_y = Pt::new(y.value() + border + style.padding.top.value());

        let has_block_child = element.children.iter().any(|child| {
            matches!(child, ComposedNode::Element(child) if child.style.display == Display::Block)
        });

        if !has_block_child {
            let mut block = self.layout_text_content(element, y, x, width)?;
            block.background = style.background_color;
            block.border = style.border;
            return Ok((block, Vec::new()));
        }

        let mut cursor_y = content_y;
        let mut child_blocks = Vec::new();

        for child in &element.children {
            self.layout_flow_node(
                child,
                &mut cursor_y,
                content_x,
                inner_width,
                &mut child_blocks,
            )?;
        }

        let content_height = (cursor_y.value() - content_y.value()).max(0.0);
        let height = border * 2.0
            + style.padding.top.value()
            + content_height
            + style.padding.bottom.value();

        let block = LayoutBlock {
            rect: Rectangle {
                position: Position { x, y },
                size: Size {
                    width,
                    height: Pt::new(height),
                },
            },
            content: LayoutContent { lines: Vec::new() },
            background: style.background_color,
            border: style.border,
        };

        Ok((block, child_blocks))
    }

    fn layout_text_content(
        &self,
        element: &ComposedElement,
        y: Pt,
        x: Pt,
        width: Pt,
    ) -> Result<LayoutBlock, AppError> {
        let style = &element.style;
        let border = style.border.width.value();
        let content_width = Pt::new(
            (width.value()
                - border * 2.0
                - style.padding.left.value()
                - style.padding.right.value())
            .max(0.0),
        );
        let text_x = Pt::new(x.value() + border + style.padding.left.value());
        let text_y = Pt::new(y.value() + border + style.padding.top.value());
        let text = normalize_text(&element.text_content());
        let line_height = Pt::new(style.font_size.value() * style.line_height.max(1.0));

        let lines = wrap_text(
            &text,
            self.font,
            style.direction,
            style.font_size,
            content_width,
        )?;

        let mut layout_lines = Vec::with_capacity(lines.len());
        let mut line_y = text_y;

        for shaped in lines {
            let text_width = shaped.width;
            let line_x = line_x(
                style.direction,
                style.text_align,
                text_x,
                content_width,
                text_width,
            );

            layout_lines.push(LayoutLine {
                text: shaped.text.clone(),
                glyphs: shaped,
                width: text_width,
                position: Position {
                    x: line_x,
                    y: line_y,
                },
                font_size: style.font_size,
                direction: style.direction,
                color: style.color,
                font_weight: style.font_weight,
            });

            line_y = Pt::new(line_y.value() + line_height.value());
        }

        let content_height = if layout_lines.is_empty() {
            0.0
        } else {
            line_height.value() * layout_lines.len() as f32
        };

        let height = border * 2.0
            + style.padding.top.value()
            + content_height
            + style.padding.bottom.value();

        Ok(LayoutBlock {
            rect: Rectangle {
                position: Position { x, y },
                size: Size {
                    width,
                    height: Pt::new(height),
                },
            },
            content: LayoutContent {
                lines: layout_lines,
            },
            background: style.background_color,
            border: style.border,
        })
    }
}

fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
