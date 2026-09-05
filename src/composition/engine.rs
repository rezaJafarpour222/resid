use crate::{
    css::{
        computed::compute_style,
        parser::CssParser,
        rules::StyleRule,
        selector::DomElement,
        types::{ComputedStyle, Display, FontWeight},
    },
    document::page::Page,
    error::AppError,
    html::{
        parser::HtmlParser,
        types::{Document as HtmlDocument, Element, Node},
    },
};

use super::types::{ComposedDocument, ComposedElement, ComposedNode};

pub struct CompositionEngine {
    default_page: Page,
}

impl CompositionEngine {
    pub fn new(page: Page) -> Self {
        Self { default_page: page }
    }

    pub fn compose(&self, html: &str) -> Result<ComposedDocument, AppError> {
        self.compose_with_css(html, "")
    }

    pub fn compose_with_css(
        &self,
        html: &str,
        extra_css: &str,
    ) -> Result<ComposedDocument, AppError> {
        let document = HtmlParser::parse(html)?;
        let embedded_css = collect_stylesheet_text(&document);

        let mut css = embedded_css;
        if !extra_css.trim().is_empty() {
            css.push('\n');
            css.push_str(extra_css);
        }

        let rules = CssParser::parse_stylesheet(&css)?;

        let mut path = Vec::new();
        let children = document
            .children
            .iter()
            .enumerate()
            .filter_map(|(index, node)| {
                path.push(index);
                let composed = self.compose_node(node, None, &mut path, &document, &rules);
                path.pop();
                composed
            })
            .collect();

        Ok(ComposedDocument {
            page: self.default_page,
            children,
        })
    }

    fn compose_node(
        &self,
        node: &Node,
        parent_style: Option<&ComputedStyle>,
        path: &mut Vec<usize>,
        document: &HtmlDocument,
        rules: &[StyleRule],
    ) -> Option<ComposedNode> {
        match node {
            Node::Text(text) => Some(ComposedNode::Text(text.clone())),
            Node::Element(element) => {
                if element.tag_name == "style" {
                    return None;
                }

                let base = default_style_for_element(element);
                let dom_element = DomElement::new(document, path.clone());
                let style = compute_style(&dom_element, element, parent_style, rules, base);

                if style.display == Display::None {
                    return None;
                }

                let children = element
                    .children
                    .iter()
                    .enumerate()
                    .filter_map(|(index, child)| {
                        path.push(index);
                        let composed =
                            self.compose_node(child, Some(&style), path, document, rules);
                        path.pop();
                        composed
                    })
                    .collect();

                Some(ComposedNode::Element(ComposedElement {
                    element: element.clone(),
                    style,
                    children,
                }))
            }
        }
    }
}

fn collect_stylesheet_text(document: &HtmlDocument) -> String {
    let mut output = String::new();
    collect_style_nodes(&document.children, &mut output);
    output
}

fn collect_style_nodes(nodes: &[Node], output: &mut String) {
    for node in nodes {
        if let Node::Element(element) = node {
            if element.tag_name == "style" {
                collect_raw_text(&element.children, output);
                output.push('\n');
            } else {
                collect_style_nodes(&element.children, output);
            }
        }
    }
}

fn collect_raw_text(nodes: &[Node], output: &mut String) {
    for node in nodes {
        match node {
            Node::Text(text) => output.push_str(text),
            Node::Element(element) => collect_raw_text(&element.children, output),
        }
    }
}

fn default_style_for_element(element: &Element) -> ComputedStyle {
    let mut style = ComputedStyle::default();

    style.display = match element.tag_name.as_str() {
        "html" | "body" | "div" | "section" | "article" | "header" | "footer" | "main"
        | "aside" | "nav" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "table" | "thead"
        | "tbody" | "tfoot" | "tr" | "td" | "th" => Display::Block,
        "head" | "style" | "script" | "meta" | "title" | "link" => Display::None,
        "br" => Display::Inline,
        "b" | "strong" => {
            style.font_weight = FontWeight::Bold;
            Display::Inline
        }
        _ => Display::Inline,
    };

    match element.tag_name.as_str() {
        "h1" => {
            style.font_size = crate::units::Pt::new(24.0);
            style.line_height = 1.3;
            style.font_weight = FontWeight::Bold;
        }
        "h2" => {
            style.font_size = crate::units::Pt::new(20.0);
            style.line_height = 1.3;
            style.font_weight = FontWeight::Bold;
        }
        "h3" => {
            style.font_size = crate::units::Pt::new(16.0);
            style.line_height = 1.3;
            style.font_weight = FontWeight::Bold;
        }
        "h4" => {
            style.font_size = crate::units::Pt::new(14.0);
            style.line_height = 1.3;
            style.font_weight = FontWeight::Bold;
        }
        "p" => {
            style.line_height = 1.8;
        }
        _ => {}
    }

    if let Some(direction) = element.attribute("dir") {
        match direction {
            "rtl" => style.direction = crate::units::Direction::RTL,
            "ltr" => style.direction = crate::units::Direction::LTR,
            _ => {}
        }
    }

    style
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::types::{Color, TextAlign};
    use crate::document::page::Page;

    #[test]
    fn embedded_css_is_applied_during_composition() {
        let html = r#"
            <html>
                <head>
                    <style>
                        body { background-color: #eeeeee; margin: 10pt; }
                        .title { color: #123456; font-size: 24pt; text-align: center; padding: 5pt; }
                    </style>
                </head>
                <body>
                    <div class="title">Invoice</div>
                </body>
            </html>
        "#;

        let document = CompositionEngine::new(Page::a4_portrait())
            .compose(html)
            .expect("composition failed");

        let body = match &document.children[0] {
            ComposedNode::Element(element) => element,
            _ => panic!("expected html element"),
        };

        assert_eq!(body.style.background_color, Some(Color::rgb(238, 238, 238)));
        assert_eq!(body.style.margin.left, crate::units::Pt::new(10.0));

        let title = match &body.children[0] {
            ComposedNode::Element(element) => element,
            _ => panic!("expected title element"),
        };

        assert_eq!(title.style.color, Color::rgb(0x12, 0x34, 0x56));
        assert_eq!(title.style.font_size, crate::units::Pt::new(24.0));
        assert_eq!(title.style.text_align, TextAlign::Center);
        assert_eq!(title.style.padding.left, crate::units::Pt::new(5.0));
    }
}
