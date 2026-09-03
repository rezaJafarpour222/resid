use crate::{
    css::types::ComputedStyle,
    document::types::Page,
    html::types::{Element, Node},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ComposedDocument {
    pub page: Page,
    pub children: Vec<ComposedNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComposedNode {
    Element(ComposedElement),
    Text(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComposedElement {
    pub element: Element,
    pub style: ComputedStyle,
    pub children: Vec<ComposedNode>,
}

impl ComposedElement {
    pub fn text_content(&self) -> String {
        let mut text = String::new();
        collect_text(&self.children, &mut text);
        text
    }
}

fn collect_text(nodes: &[ComposedNode], output: &mut String) {
    for node in nodes {
        match node {
            ComposedNode::Text(value) => output.push_str(value),
            ComposedNode::Element(element) => collect_text(&element.children, output),
        }
    }
}

impl From<Node> for ComposedNode {
    fn from(value: Node) -> Self {
        match value {
            Node::Text(text) => Self::Text(text),
            Node::Element(element) => Self::Element(ComposedElement {
                element,
                style: ComputedStyle::default(),
                children: Vec::new(),
            }),
        }
    }
}
