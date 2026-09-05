use super::types::{ComposedDocument, ComposedElement, ComposedNode};

use crate::{
    css::{
        computed::compute_style,
        parser::CssParser,
        rules::StyleRule,
        selector::DomElement,
        types::{ComputedStyle, Display, FontWeight, ListStyleType},
    },
    document::page::Page,
    error::AppError,
    html::{
        parser::HtmlParser,
        types::{Document as HtmlDocument, Element, Node},
    },
};

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
        let mut css = collect_stylesheet_text(&document);
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
            .filter_map(|(i, n)| {
                path.push(i);
                let r = self.compose_node(n, None, &mut path, &document, &rules);
                path.pop();
                r
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
        parent: Option<&ComputedStyle>,
        path: &mut Vec<usize>,
        document: &HtmlDocument,
        rules: &[StyleRule],
    ) -> Option<ComposedNode> {
        match node {
            Node::Text(t) => Some(ComposedNode::Text(t.clone())),
            Node::Element(element) => {
                if matches!(
                    element.tag_name.as_str(),
                    "style" | "script" | "meta" | "link" | "title"
                ) {
                    return None;
                }
                let base = default_style_for_element(element);
                let dom = DomElement::new(document, path.clone());
                let style = compute_style(&dom, element, parent, rules, base);
                if style.display == Display::None {
                    return None;
                }
                let children = element
                    .children
                    .iter()
                    .enumerate()
                    .filter_map(|(i, ch)| {
                        path.push(i);
                        let r = self.compose_node(ch, Some(&style), path, document, rules);
                        path.pop();
                        r
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
    let mut out = String::new();
    collect_style_nodes(&document.children, &mut out);
    out
}
fn collect_style_nodes(nodes: &[Node], out: &mut String) {
    for n in nodes {
        if let Node::Element(e) = n {
            if e.tag_name == "style" {
                collect_raw_text(&e.children, out);
                out.push('\n')
            } else {
                collect_style_nodes(&e.children, out)
            }
        }
    }
}
fn collect_raw_text(nodes: &[Node], out: &mut String) {
    for n in nodes {
        match n {
            Node::Text(t) => out.push_str(t),
            Node::Element(e) => collect_raw_text(&e.children, out),
        }
    }
}
fn default_style_for_element(element: &Element) -> ComputedStyle {
    let mut s = ComputedStyle::default();
    s.display = match element.tag_name.as_str() {
        "html" | "body" | "div" | "section" | "article" | "header" | "footer" | "main"
        | "aside" | "nav" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "table" | "thead"
        | "tbody" | "tfoot" | "tr" => Display::Block,
        "td" | "th" => Display::TableCell,
        "li" => Display::ListItem,
        "head" | "style" | "script" | "meta" | "title" | "link" => Display::None,
        "br" => Display::Inline,
        _ => Display::Inline,
    };
    match element.tag_name.as_str() {
        "h1" => {
            s.font_size = crate::units::Pt::new(24.0);
            s.line_height = 1.3;
            s.font_weight = FontWeight::Bold;
            s.margin = crate::css::edges::Edges::vertical_horizontal(
                crate::units::Pt::new(8.0),
                crate::units::Pt::ZERO,
            )
        }
        "h2" => {
            s.font_size = crate::units::Pt::new(20.0);
            s.line_height = 1.3;
            s.font_weight = FontWeight::Bold;
            s.margin = crate::css::edges::Edges::vertical_horizontal(
                crate::units::Pt::new(6.0),
                crate::units::Pt::ZERO,
            )
        }
        "h3" => {
            s.font_size = crate::units::Pt::new(16.0);
            s.line_height = 1.3;
            s.font_weight = FontWeight::Bold
        }
        "h4" => {
            s.font_size = crate::units::Pt::new(14.0);
            s.line_height = 1.3;
            s.font_weight = FontWeight::Bold
        }
        "p" => {
            s.line_height = 1.5;
            s.margin.bottom = crate::units::Pt::new(8.0)
        }
        "b" | "strong" => {
            s.font_weight = FontWeight::Bold;
            s.display = Display::Inline
        }
        "i" | "em" => s.display = Display::Inline,
        "ul" => s.list_style_type = ListStyleType::Disc,
        "ol" => s.list_style_type = ListStyleType::Decimal,
        _ => {}
    }
    if let Some(dir) = element.attribute("dir") {
        match dir {
            "rtl" => s.direction = crate::units::Direction::RTL,
            "ltr" => s.direction = crate::units::Direction::LTR,
            _ => {}
        }
    }
    s
}
