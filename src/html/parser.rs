use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use crate::error::AppError;

use super::types::{Attribute, Document, Element, Node};

pub struct HtmlParser;

impl HtmlParser {
    pub fn parse(input: &str) -> Result<Document, AppError> {
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut input.as_bytes())
            .map_err(|_| AppError::HtmlParsing("html parsing failed".to_string()))?;

        let children = dom
            .document
            .children
            .borrow()
            .iter()
            .filter_map(convert_node)
            .collect();

        Ok(Document { children })
    }
}

fn convert_node(handle: &Handle) -> Option<Node> {
    match &handle.data {
        NodeData::Element { name, attrs, .. } => {
            let attributes = attrs
                .borrow()
                .iter()
                .map(|attribute| Attribute {
                    name: attribute.name.local.to_string(),
                    value: attribute.value.to_string(),
                })
                .collect::<Vec<_>>();

            let id = attributes
                .iter()
                .find(|attribute| attribute.name == "id")
                .map(|attribute| attribute.value.clone());

            let classes = attributes
                .iter()
                .find(|attribute| attribute.name == "class")
                .map(|attribute| {
                    attribute
                        .value
                        .split_whitespace()
                        .map(str::to_string)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            let children = handle
                .children
                .borrow()
                .iter()
                .filter_map(convert_node)
                .collect();

            Some(Node::Element(Element {
                tag_name: name.local.to_string(),
                id,
                classes,
                attributes,
                children,
            }))
        }

        NodeData::Text { contents } => Some(Node::Text(contents.borrow().to_string())),

        _ => None,
    }
}
