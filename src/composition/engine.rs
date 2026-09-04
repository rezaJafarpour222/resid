use crate::{
    css::{
        computed::compute_style,
        parser::CssParser,
        rules::StyleRule,
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

        let rules = CssParser::parse_stylesheet(&css).map_err(AppError::CssParsing)?;

        let mut ancestors = Vec::new();
        let children = document
            .children
            .iter()
            .filter_map(|node| self.compose_node(node, None, &mut ancestors, &rules))
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
        ancestors: &mut Vec<Element>,
        rules: &[StyleRule],
    ) -> Option<ComposedNode> {
        match node {
            Node::Text(text) => Some(ComposedNode::Text(text.clone())),
            Node::Element(element) => {
                if element.tag_name == "style" {
                    return None;
                }

                let mut base = default_style_for_element(element);

                if let Some(parent) = parent_style {
                    base.direction = parent.direction;
                    base.font_family = parent.font_family.clone();
                    base.font_size = parent.font_size;
                    base.font_weight = parent.font_weight;
                    base.line_height = parent.line_height;
                    base.text_align = parent.text_align;
                    base.color = parent.color;
                }

                let style = compute_style(element, ancestors, parent_style, rules, base);

                if style.display == Display::None {
                    return None;
                }

                ancestors.push(element.clone());

                let children = element
                    .children
                    .iter()
                    .filter_map(|child| self.compose_node(child, Some(&style), ancestors, rules))
                    .collect();

                ancestors.pop();

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
