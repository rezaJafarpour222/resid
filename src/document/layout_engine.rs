use crate::{
    composition::types::{ComposedDocument, ComposedElement, ComposedNode},
    css::types::{
        AlignItems, Display, FlexDirection, JustifyContent, Length, ListStyleType,
        Position as CssPosition, WhiteSpace,
    },
    document::{
        helper::{line_x, wrap_text},
        types::{LayoutBlock, LayoutContent, LayoutDocument, LayoutLine, LayoutPage},
    },
    error::AppError,
    font::{loader::Font, shaper::Shaper},
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

                if element.style.page_break_before {
                    *cursor_y =
                        Pt::new(cursor_y.value() + self.page_break_distance(available_width));
                }

                let old_y = *cursor_y;
                let start = blocks.len();

                match element.style.display {
                    Display::Flex => {
                        self.layout_flex(element, cursor_y, x, available_width, blocks)?
                    }

                    Display::Table => {
                        self.layout_table(element, cursor_y, x, available_width, blocks)?
                    }

                    Display::ListItem => {
                        self.layout_list_item(element, cursor_y, x, available_width, blocks)?
                    }

                    Display::Inline | Display::InlineBlock => {
                        self.layout_inline_element(element, cursor_y, x, available_width, blocks)?
                    }

                    _ => self.layout_block(element, cursor_y, x, available_width, blocks)?,
                }

                let dx = element
                    .style
                    .left
                    .resolve(available_width, Pt::ZERO)
                    .value()
                    - element
                        .style
                        .right
                        .resolve(available_width, Pt::ZERO)
                        .value();

                let dy = element.style.top.resolve(available_width, Pt::ZERO).value()
                    - element
                        .style
                        .bottom
                        .resolve(available_width, Pt::ZERO)
                        .value();

                if !matches!(element.style.position, CssPosition::Static)
                    && (dx != 0.0 || dy != 0.0)
                {
                    Self::shift_blocks(&mut blocks[start..], dx, dy);
                }

                if matches!(
                    element.style.position,
                    CssPosition::Absolute | CssPosition::Fixed
                ) {
                    *cursor_y = old_y;
                }

                if element.style.page_break_after {
                    *cursor_y =
                        Pt::new(cursor_y.value() + self.page_break_distance(available_width));
                }

                Ok(())
            }
        }
    }

    fn page_break_distance(&self, _available_width: Pt) -> f32 {
        10000.0
    }

    fn shift_blocks(blocks: &mut [LayoutBlock], dx: f32, dy: f32) {
        for block in blocks {
            block.rect.position.x = Pt::new(block.rect.position.x.value() + dx);

            block.rect.position.y = Pt::new(block.rect.position.y.value() + dy);

            for line in &mut block.content.lines {
                line.position.x = Pt::new(line.position.x.value() + dx);

                line.position.y = Pt::new(line.position.y.value() + dy);
            }
        }
    }

    fn box_width(&self, element: &ComposedElement, available: Pt) -> Pt {
        let mut width = element.style.width.resolve(available, available);

        if element.style.max_width != Length::Auto {
            let max_width = element.style.max_width.resolve(available, width);

            width = Pt::new(width.value().min(max_width.value()));
        }

        if element.style.min_width != Length::Auto {
            let min_width = element.style.min_width.resolve(available, Pt::ZERO);

            width = Pt::new(width.value().max(min_width.value()));
        }

        if element.style.box_sizing == crate::css::types::BoxSizing::ContentBox {
            width = Pt::new(
                width.value()
                    + element.style.padding.left.value()
                    + element.style.padding.right.value()
                    + 2.0 * element.style.border.width.value(),
            );
        }

        Pt::new(width.value().min(available.value()).max(0.0))
    }

    fn layout_block(
        &self,
        element: &ComposedElement,
        y: &mut Pt,
        x: Pt,
        available: Pt,
        blocks: &mut Vec<LayoutBlock>,
    ) -> Result<(), AppError> {
        let style = &element.style;

        let margin_top = style.margin.top.value();

        let margin_bottom = style.margin.bottom.value();

        *y = Pt::new(y.value() + margin_top);

        let width = self.box_width(element, available).value().min(
            (available.value() - style.margin.left.value() - style.margin.right.value()).max(0.0),
        );

        let width = Pt::new(width.max(0.0));

        let block_x = Pt::new(x.value() + style.margin.left.value());

        let border = style.border.width.value();

        let inner_width = Pt::new(
            (width.value()
                - 2.0 * border
                - style.padding.left.value()
                - style.padding.right.value())
            .max(0.0),
        );

        let content_x = Pt::new(block_x.value() + border + style.padding.left.value());

        let content_y = Pt::new(y.value() + border + style.padding.top.value());

        let block_y = *y;

        let has_block_children = element.children.iter().any(|child| {
            matches!(
                child,
                ComposedNode::Element(
                    child_element
                ) if !matches!(
                    child_element.style.display,
                    Display::Inline
                        | Display::InlineBlock
                )
            )
        });

        if !has_block_children {
            let mut block = self.layout_text(element, block_y, block_x, width)?;

            block.background = style.background_color;

            block.border = style.border;

            block.border_radius = style.border_radius;

            block.opacity = style.opacity;

            let height = block.rect.size.height.value();

            blocks.push(block);

            *y = Pt::new(y.value() + height + margin_bottom);

            return Ok(());
        }

        let mut child_cursor = content_y;

        let mut child_blocks = Vec::new();

        for child in &element.children {
            self.layout_flow_node(
                child,
                &mut child_cursor,
                content_x,
                inner_width,
                &mut child_blocks,
            )?;
        }

        let content_height = (child_cursor.value() - content_y.value()).max(0.0);

        let height = 2.0 * border
            + style.padding.top.value()
            + content_height
            + style.padding.bottom.value();

        blocks.push(LayoutBlock {
            rect: Rectangle {
                position: Position {
                    x: block_x,
                    y: block_y,
                },
                size: Size {
                    width,
                    height: Pt::new(height),
                },
            },
            content: LayoutContent { lines: Vec::new() },
            background: style.background_color,
            border: style.border,
            border_radius: style.border_radius,
            opacity: style.opacity,
        });

        blocks.extend(child_blocks);

        *y = Pt::new(y.value() + height + margin_bottom);

        Ok(())
    }

    fn layout_inline_element(
        &self,
        element: &ComposedElement,
        cursor_y: &mut Pt,
        x: Pt,
        width: Pt,
        blocks: &mut Vec<LayoutBlock>,
    ) -> Result<(), AppError> {
        if element.text_content().trim().is_empty() {
            return Ok(());
        }

        let block = self.layout_text(element, *cursor_y, x, width)?;

        *cursor_y = Pt::new(cursor_y.value() + block.rect.size.height.value());

        blocks.push(block);

        Ok(())
    }

    fn layout_text(
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
                - 2.0 * border
                - style.padding.left.value()
                - style.padding.right.value())
            .max(0.0),
        );

        let text_x = Pt::new(x.value() + border + style.padding.left.value());

        let text_y = Pt::new(y.value() + border + style.padding.top.value());

        let text = normalize_text(&element.text_content(), style.white_space);

        let line_height = style.font_size.value() * style.line_height.max(1.0);

        let mut layout_lines = Vec::new();

        let mut line_y = text_y;

        for raw_line in text.split('\n') {
            let shaped_lines = if matches!(style.white_space, WhiteSpace::NoWrap) {
                vec![Shaper::shaped_text(
                    self.font,
                    raw_line,
                    style.direction,
                    style.font_size,
                )?]
            } else {
                wrap_text(
                    raw_line,
                    self.font,
                    style.direction,
                    style.font_size,
                    content_width,
                )?
            };

            for shaped in shaped_lines {
                let x_position = line_x(
                    style.direction,
                    style.text_align,
                    text_x,
                    content_width,
                    shaped.width,
                );

                // `shaped` is moved into `glyphs`, so copy
                // everything needed from it before the move.
                let line_text = shaped.text.clone();

                let line_width = shaped.width;

                layout_lines.push(LayoutLine {
                    text: line_text,
                    glyphs: shaped,
                    width: line_width,
                    position: Position {
                        x: x_position,
                        y: line_y,
                    },
                    font_size: style.font_size,
                    direction: style.direction,
                    color: style.color,
                    font_weight: style.font_weight.clone(),
                    text_decoration: style.text_decoration.clone(),
                });

                line_y = Pt::new(line_y.value() + line_height);
            }
        }

        let content_height = if layout_lines.is_empty() {
            0.0_f32
        } else {
            line_height * layout_lines.len() as f32
        };

        let height = 2.0 * border
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
            border_radius: style.border_radius,
            opacity: style.opacity,
        })
    }

    fn layout_flex(
        &self,
        element: &ComposedElement,
        y: &mut Pt,
        x: Pt,
        width: Pt,
        blocks: &mut Vec<LayoutBlock>,
    ) -> Result<(), AppError> {
        let style = &element.style;

        let items = element
            .children
            .iter()
            .filter_map(|child| match child {
                ComposedNode::Element(child) if child.style.display != Display::None => Some(child),

                _ => None,
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return self.layout_block(element, y, x, width, blocks);
        }

        let row = matches!(
            style.flex_direction,
            FlexDirection::Row | FlexDirection::RowReverse
        );

        let reverse = matches!(
            style.flex_direction,
            FlexDirection::RowReverse | FlexDirection::ColumnReverse
        );

        let gap = if row {
            style.column_gap.value().max(style.gap.value())
        } else {
            style.row_gap.value().max(style.gap.value())
        };

        let ordered = if reverse {
            items.iter().rev().copied().collect::<Vec<_>>()
        } else {
            items
        };

        let total_gap = gap * ordered.len().saturating_sub(1) as f32;

        let basis = if row {
            ((width.value() - total_gap).max(0.0)) / ordered.len() as f32
        } else {
            width.value()
        };

        let line_start = *y;

        let mut item_layouts: Vec<(Pt, Vec<LayoutBlock>, f32)> = Vec::new();

        let mut cursor = if row { x.value() } else { y.value() };

        let mut max_height: f32 = 0.0;

        for child in ordered {
            let item_width = if row {
                child
                    .style
                    .flex_basis
                    .resolve(Pt::new(basis), Pt::new(basis))
                    .value()
                    .max(1.0)
            } else {
                width.value()
            };

            let child_x = if row { Pt::new(cursor) } else { x };

            let child_y = if row { line_start } else { Pt::new(cursor) };

            let mut child_cursor = child_y;

            let mut child_blocks = Vec::new();

            self.layout_flow_node(
                &ComposedNode::Element(child.clone()),
                &mut child_cursor,
                child_x,
                Pt::new(item_width),
                &mut child_blocks,
            )?;

            let child_height = (child_cursor.value() - child_y.value()).max(0.0);

            max_height = max_height.max(child_height);

            item_layouts.push((child_x, child_blocks, child_height));

            cursor += if row {
                item_width + gap
            } else {
                child_height + gap
            };
        }

        for (child_x, mut child_blocks, child_height) in item_layouts {
            let offset_y = if !row {
                0.0
            } else {
                match style.align_items {
                    AlignItems::Center => (max_height - child_height).max(0.0) / 2.0,

                    AlignItems::FlexEnd => (max_height - child_height).max(0.0),

                    _ => 0.0,
                }
            };

            let offset_x = if row {
                0.0
            } else {
                self.flex_cross_offset(style.align_items, width.value(), child_x.value(), x.value())
            };

            if offset_y != 0.0 || offset_x != 0.0 {
                Self::shift_blocks(&mut child_blocks, offset_x, offset_y);
            }

            blocks.extend(child_blocks);
        }

        *y = if row {
            Pt::new(line_start.value() + max_height)
        } else {
            Pt::new(cursor - gap)
        };

        if style.justify_content != JustifyContent::FlexStart && row {
            // Equal-basis items currently consume
            // the available main-axis space.
        }

        Ok(())
    }

    fn flex_cross_offset(
        &self,
        _align: AlignItems,
        _container_width: f32,
        _child_x: f32,
        _start_x: f32,
    ) -> f32 {
        0.0
    }

    fn layout_table(
        &self,
        element: &ComposedElement,
        y: &mut Pt,
        x: Pt,
        width: Pt,
        blocks: &mut Vec<LayoutBlock>,
    ) -> Result<(), AppError> {
        let mut rows = Vec::new();

        collect_table_rows(&element.children, &mut rows);

        if rows.is_empty() {
            return self.layout_block(element, y, x, width, blocks);
        }

        let columns = rows
            .iter()
            .map(|row| {
                row.children
                    .iter()
                    .filter(|child| {
                        matches!(
                            child,
                            ComposedNode::Element(cell)
                                if cell.style.display
                                    == Display::TableCell
                        )
                    })
                    .count()
            })
            .max()
            .unwrap_or(1);

        let column_width = Pt::new(width.value() / columns as f32);

        let mut row_y = *y;

        for row in rows {
            let cells = row
                .children
                .iter()
                .filter_map(|child| match child {
                    ComposedNode::Element(cell) if cell.style.display == Display::TableCell => {
                        Some(cell)
                    }

                    _ => None,
                })
                .collect::<Vec<_>>();

            let mut row_height: f32 = 0.0;

            let mut row_blocks = Vec::new();

            for (index, cell) in cells.iter().enumerate() {
                let mut cell_blocks = Vec::new();

                let mut cell_y = row_y;

                self.layout_flow_node(
                    &ComposedNode::Element((*cell).clone()),
                    &mut cell_y,
                    Pt::new(x.value() + index as f32 * column_width.value()),
                    column_width,
                    &mut cell_blocks,
                )?;

                row_height = row_height.max(cell_y.value() - row_y.value());

                row_blocks.extend(cell_blocks);
            }

            blocks.extend(row_blocks);

            row_y = Pt::new(row_y.value() + row_height.max(16.0));
        }

        *y = row_y;

        Ok(())
    }

    fn layout_list_item(
        &self,
        element: &ComposedElement,
        y: &mut Pt,
        x: Pt,
        width: Pt,
        blocks: &mut Vec<LayoutBlock>,
    ) -> Result<(), AppError> {
        let indent = Pt::new(18.0);

        let bullet = match element.style.list_style_type {
            ListStyleType::Disc => "• ",
            ListStyleType::Circle => "○ ",
            ListStyleType::Square => "■ ",
            ListStyleType::Decimal => "1. ",
            ListStyleType::None => "",
        };

        let mut block = self.layout_text(
            element,
            *y,
            Pt::new(x.value() + indent.value()),
            Pt::new((width.value() - indent.value()).max(0.0)),
        )?;

        if !bullet.is_empty() {
            if let Some(first) = block.content.lines.first_mut() {
                let combined = format!("{bullet}{}", first.text);

                let shaped = Shaper::shaped_text(
                    self.font,
                    &combined,
                    element.style.direction,
                    element.style.font_size,
                )?;

                first.text = combined;

                first.width = shaped.width;

                first.glyphs = shaped;
            }
        }

        *y = Pt::new(y.value() + block.rect.size.height.value());

        blocks.push(block);

        Ok(())
    }
}

fn normalize_text(text: &str, white_space: WhiteSpace) -> String {
    match white_space {
        WhiteSpace::Pre | WhiteSpace::PreWrap => text.replace("\r\n", "\n"),

        WhiteSpace::NoWrap | WhiteSpace::Normal => {
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        }
    }
}

fn collect_table_rows<'a>(nodes: &'a [ComposedNode], rows: &mut Vec<&'a ComposedElement>) {
    for node in nodes {
        if let ComposedNode::Element(element) = node {
            match element.style.display {
                Display::TableRow => {
                    rows.push(element);
                }

                Display::Block => {
                    if matches!(
                        element.element.tag_name.as_str(),
                        "thead" | "tbody" | "tfoot"
                    ) {
                        collect_table_rows(&element.children, rows);
                    }
                }

                _ => {}
            }
        }
    }
}
