use markup5ever_rcdom::{Handle, NodeData};

use crate::document::types::{Inline, InlineContent};

pub fn collect_inline(handle: &Handle) -> InlineContent {
    let mut items = Vec::new();

    collect_inline_nodes(handle, &mut items);

    InlineContent { items }
}

pub fn collect_inline_nodes(handle: &Handle, items: &mut Vec<Inline>) {
    for child in handle.children.borrow().iter() {
        match &child.data {
            NodeData::Text { contents } => {
                let text = contents.borrow().to_string();

                if !text.trim().is_empty() {
                    items.push(Inline::Text(text));
                }
            }

            NodeData::Element { .. } => {
                collect_inline_nodes(child, items);
            }

            _ => {}
        }
    }
}
