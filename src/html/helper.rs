use super::types::{Element, Node};

pub fn text_content(element: &Element) -> String {
    let mut text = String::new();
    collect_text(&element.children, &mut text);
    text
}

fn collect_text(nodes: &[Node], output: &mut String) {
    for node in nodes {
        match node {
            Node::Text(value) => output.push_str(value),
            Node::Element(element) => collect_text(&element.children, output),
        }
    }
}
